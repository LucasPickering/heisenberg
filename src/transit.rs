use crate::{
    config::Config,
    state::{Message, Tx},
    util::http_get,
};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    thread,
    time::Duration,
};
use tracing::error;

/// Time between requests
const DATA_TTL: Duration = Duration::from_secs(30);
/// Max number of pending departures to show for a stop
const MAX_PREDICTIONS: usize = 3;

/// Fetch transit data in a loop. When we get new predictions, send a message to
/// update state
pub fn transit_loop(config: Config, tx: Tx) {
    let stop_ids = config
        .transit_lines
        .iter()
        .flat_map(|line| &line.stops)
        .map(|stop| stop.id);
    let url = format!(
        "https://api-v3.mbta.com/predictions?filter[stop]={}",
        stop_ids.format(",")
    );

    loop {
        if let Ok(api_data) = http_get::<ApiPredictions>(&url) {
            let predictions = TransitPredictions::from_response(
                &config.transit_lines,
                api_data,
            );
            tx.send(Message::Transit(predictions));
        }
        thread::sleep(DATA_TTL);
    }
}

/// Configuration for a transit line to show predictions for
#[derive(Clone, Debug, Deserialize)]
pub struct TransitLine {
    /// Display name for the line
    pub name: String,
    /// Stops on the line to track
    pub stops: Vec<Stop>,
}

/// Definition for a single stop on a line
#[derive(Clone, Debug, Deserialize)]
pub struct Stop {
    /// Display name for the stop
    pub name: String,
    /// MBTA API stop ID
    pub id: u32,
}

/// Predictions for all tracked transit lines/stops
///
/// All hail Philip Eng
#[derive(Debug, Default)]
pub struct TransitPredictions {
    pub lines: Vec<LinePredictions>,
}

impl TransitPredictions {
    /// Gather predictions from
    fn from_response(
        lines: &[TransitLine],
        api_data: ApiPredictions,
    ) -> TransitPredictions {
        // Group API data as {(line, stop): [prediction]}
        let mut grouped: HashMap<(&str, u32), Vec<DateTime<Utc>>> = api_data
            .data
            .iter()
            .filter_map(|prediction| {
                // Departure time will be empty if the stop is being skipped
                let departure_time = prediction.attributes.departure_time?;
                let route_id = prediction.relationships.route.data.id.as_str();
                let stop_id =
                    prediction.relationships.stop.data.id.parse::<u32>().inspect_err(|error|
                    error!(%error, "Invalid stop ID in API response")
                    ).ok()?;

                Some(((route_id, stop_id), departure_time))
            })
            .into_group_map();

        let lines = lines
            .iter()
            .map(|line| {
                let stops = line
                    .stops
                    .iter()
                    .map(|stop| {
                        let predictions: CountdownList = grouped
                            .remove(&(&line.name, stop.id))
                            .unwrap_or_default()
                            .into();
                        StopPredictions {
                            name: stop.name.clone(),
                            predictions,
                        }
                    })
                    .collect();
                LinePredictions {
                    name: line.name.clone(),
                    stops,
                }
            })
            .collect();
        TransitPredictions { lines }
    }
}

/// Arrival predictions for all stops on a line, ready to be displayed
#[derive(Debug)]
pub struct LinePredictions {
    pub name: String,
    pub stops: Vec<StopPredictions>,
}

/// Arrival predictions for a single stop, ready to be displayed
#[derive(Debug)]
pub struct StopPredictions {
    pub name: String,
    pub predictions: CountdownList,
}

/// List of upcoming arrivals for a stop
#[derive(Debug)]
pub struct CountdownList(Vec<Countdown>);

/// Convert a list of timestamps into relative offsets from now, sorting and
/// truncating as necessary
impl From<Vec<DateTime<Utc>>> for CountdownList {
    fn from(value: Vec<DateTime<Utc>>) -> Self {
        let now = Utc::now();
        let countdowns = value
            .into_iter()
            // Get the first n upcoming timestamps
            .sorted()
            .take(MAX_PREDICTIONS)
            .map(|dt| Countdown((dt - now).num_minutes()))
            .collect();
        Self(countdowns)
    }
}

impl Display for CountdownList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "None")
        } else {
            write!(f, "{}", self.0.iter().format(", "))
        }
    }
}

/// Number of minutes until an event
#[derive(Debug)]
pub struct Countdown(i64);

impl Display for Countdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

/// <https://api-v3.mbta.com/docs/swagger/index.html#/Prediction/ApiWeb_PredictionController_index>
#[derive(Clone, Debug, Deserialize)]
struct ApiPredictions {
    data: Vec<Prediction>,
}

#[derive(Clone, Debug, Deserialize)]
struct Prediction {
    attributes: Attributes,
    relationships: Relationships,
}

#[derive(Clone, Debug, Deserialize)]
struct Attributes {
    departure_time: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Deserialize)]
struct Relationships {
    route: Relationship,
    stop: Relationship,
}

#[derive(Clone, Debug, Deserialize)]
struct Relationship {
    data: RelationshipData,
}

#[derive(Clone, Debug, Deserialize)]
struct RelationshipData {
    id: String,
}

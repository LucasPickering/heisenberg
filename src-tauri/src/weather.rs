use crate::{config::Config, util::http_get};
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};
use tauri::{AppHandle, Emitter};

/// Time between requests
const DATA_TTL: Duration = Duration::from_secs(60);
const API_HOST: &str = "https://api.weather.gov";

/// Fetch weather in a loop. When we get a new forecast, send a message to
/// update state
pub fn weather_loop(config: Config, app_handle: AppHandle) {
    let url = format!(
        "{}/gridpoints/{}/{},{}/forecast/hourly",
        API_HOST,
        config.forecast_office,
        config.forecast_gridpoint.0,
        config.forecast_gridpoint.1
    );

    app_handle.emit("weather", Forecast::default()).unwrap(); // TODO const
    loop {
        if let Ok(forecast) = http_get::<ApiForecast>(&url) {
            // We have a new forecast. Update state
            app_handle
                .emit("weather", Forecast::from_api(forecast))
                .unwrap(); // TODO const
        }
        thread::sleep(DATA_TTL);
    }
}

/// What weather is coming?
#[derive(Clone, Debug, Default, Serialize)]
struct Forecast {
    periods: Vec<ForecastPeriod>,
}

/// One time segment in a forecast
#[derive(Clone, Debug, Serialize)]
struct ForecastPeriod {
    start_time: DateTime<Local>,
    /// Temperature in degress Fahrenheit
    temperature: i32,
    /// How likely is precipitation? 0-100
    probability_of_precipitation: i32,
}

impl Forecast {
    /// Reformat the data for easy use in the frontend
    fn from_api(forecast: ApiForecast) -> Self {
        // Sometimes the forecast includes time that's already past. Filter
        // those out
        let now = Utc::now();
        let periods = forecast
            .properties
            .periods
            .into_iter()
            .filter(move |period| period.end_time > now)
            .map(|period| ForecastPeriod {
                start_time: period.start_time.with_timezone(&Local),
                temperature: period.temperature,
                probability_of_precipitation: period
                    .probability_of_precipitation
                    .value
                    .unwrap_or_default(),
            })
            .collect();
        Self { periods }
    }
}

/// Weather is a phenomenon where food and fruit and shit falls from the sky
///
/// This is the internal type for deserializing from the weather.gov API.
///
/// https://www.weather.gov/documentation/services-web-api#/default/gridpoint_forecast
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiForecast {
    properties: ApiForecastProperties,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiForecastProperties {
    periods: Vec<ApiForecastPeriod>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiForecastPeriod {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    temperature: i32,
    probability_of_precipitation: ApiUnit,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiUnit {
    pub value: Option<i32>,
}

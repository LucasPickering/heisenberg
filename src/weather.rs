use crate::{
    config::Config,
    state::{Message, Tx},
    util::http_get,
};
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use std::{thread, time::Duration};

/// Time between requests
const DATA_TTL: Duration = Duration::from_secs(60);
const API_HOST: &str = "https://api.weather.gov";

/// Fetch weather in a loop. When we get a new forecast, send a message to
/// update state
pub fn weather_loop(config: Config, tx: Tx) {
    let url = format!(
        "{}/gridpoints/{}/{},{}/forecast/hourly",
        API_HOST,
        config.forecast_office,
        config.forecast_gridpoint.0,
        config.forecast_gridpoint.1
    );

    loop {
        if let Ok(weather) = http_get(&url) {
            // We have a new forecast. Update state
            tx.send(Message::Weather(weather));
        }
        thread::sleep(DATA_TTL);
    }
}

/// Weather is a phenomenon where food and fruit and shit falls from the sky
///
/// https://www.weather.gov/documentation/services-web-api#/default/gridpoint_forecast
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherForecast {
    properties: ForecastProperties,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastProperties {
    periods: Vec<ForecastPeriod>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastPeriod {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    temperature: i32,
    probability_of_precipitation: Unit,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub value: Option<i32>,
}

impl WeatherForecast {
    /// Get all periods in the forecast
    pub fn periods(&self) -> impl '_ + Iterator<Item = &ForecastPeriod> {
        self.properties.periods.iter()
    }
}

impl ForecastPeriod {
    /// Localized timestamp for the start of this period
    pub fn start_time(&self) -> DateTime<Local> {
        self.start_time.with_timezone(&Local)
    }

    /// TODO
    pub fn temp(&self) -> i32 {
        self.temperature
    }

    /// TODO
    pub fn pop(&self) -> i32 {
        self.probability_of_precipitation.value.unwrap_or_default()
    }

    /// Formatted temperature
    pub fn temperature(&self) -> String {
        format!("{:.0}°", self.temperature)
    }

    /// Formatted probability of precipitation
    pub fn prob_of_precip(&self) -> String {
        format!(
            "{:.0}%",
            self.probability_of_precipitation.value.unwrap_or_default()
        )
    }
}

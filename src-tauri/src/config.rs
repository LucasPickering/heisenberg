use crate::transit::TransitLine;
use serde::Deserialize;
use std::fs::File;
use tracing::info;

/// Global app configuration
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub forecast_office: String,
    pub forecast_gridpoint: (u32, u32),
    /// Transit lines/stops to be displayed
    pub transit_lines: Vec<TransitLine>,
}

impl Config {
    const PATH: &'static str = "./config.json";

    /// Load config from file
    pub fn load() -> Self {
        info!("Loading config from `{}`", Self::PATH);
        let file = File::open(Self::PATH).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}

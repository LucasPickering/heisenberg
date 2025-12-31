use crate::weather::Weather;
use std::{
    fmt::{self, Display},
    sync::mpsc::Sender,
    time::Instant,
};

/// TODO
pub struct State {
    pub start: Instant,
    pub weather: Weather,
    pub now: Instant,
    pub mode: Mode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Weather,
            start: Instant::now(),
            now: Instant::now(),
            weather: Weather::default(),
        }
    }
}

/// TODO
pub enum Message {
    /// Exit the program
    Quit,
    Time(Instant), // TODO
    SetMode(Mode),
    /// Update the weather forecast
    Weather(Weather),
}

/// Message sender channel
pub type Tx = Sender<Message>;

/// What data is being displayed?
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    Weather,
    Transit,
}

impl Mode {
    /// List of all modes
    pub const ALL: [Self; 2] = [Self::Weather, Self::Transit];
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weather => write!(f, "Weather"),
            Self::Transit => write!(f, "Transit"),
        }
    }
}

use crate::{transit::TransitPredictions, weather::WeatherForecast};
use std::{
    fmt::{self, Display},
    sync::mpsc::Sender,
    time::Instant,
};

/// Global application state. This is modified by [Message]s sent to an
/// mpsc channel
pub struct State {
    pub start: Instant,
    pub weather: WeatherForecast,
    pub transit: TransitPredictions,
    pub now: Instant,
    pub mode: Mode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Weather,
            start: Instant::now(),
            now: Instant::now(),
            transit: TransitPredictions::default(),
            weather: WeatherForecast::default(),
        }
    }
}

/// TODO
pub enum Message {
    /// Switch tabs
    Mode(Mode),
    /// Exit the program
    Quit,
    /// Update transit predictions
    Transit(TransitPredictions),
    /// Update the weather forecast
    Weather(WeatherForecast),
}

/// Message sender channel
#[derive(Clone)]
pub struct Tx(Sender<Message>);

impl Tx {
    pub fn new(tx: Sender<Message>) -> Self {
        Self(tx)
    }

    /// Send a message
    pub fn send(&self, message: Message) {
        // Send only fails if the receiver has been dropped. The main thread
        // always keeps it open, so if this fails the main thread is done. We
        // can just kill the thread
        self.0.send(message);
    }
}

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

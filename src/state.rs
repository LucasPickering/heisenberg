use crate::{transit::TransitPredictions, weather::WeatherForecast};
use std::{
    fmt::{self, Display},
    sync::mpsc::Sender,
};

/// Global application state. This is modified by [Message]s sent to an
/// mpsc channel
pub struct State {
    pub transit: TransitPredictions,
    pub weather: WeatherForecast,
    pub mode: Mode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Weather,
            transit: TransitPredictions::default(),
            weather: WeatherForecast::default(),
        }
    }
}

/// A message is sent from background threads to the main thread to modify state
pub enum Message {
    /// Switch to the next tab in the list
    NextMode,
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
        self.0.send(message).expect("Message receiver closed");
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

    /// Get the next mode in the list
    pub fn next(self) -> Self {
        let current = Self::ALL.iter().position(|m| *m == self).unwrap();
        Self::ALL[(current + 1) % Self::ALL.len()]
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weather => write!(f, "Weather"),
            Self::Transit => write!(f, "Transit"),
        }
    }
}

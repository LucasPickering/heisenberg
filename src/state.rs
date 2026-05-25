use crate::{
    sports::SportsSchedule, transit::TransitPredictions,
    weather::WeatherForecast,
};
use std::{
    fmt::{self, Display},
    sync::mpsc::Sender,
};

/// Global application state. This is modified by [Message]s sent to an
/// mpsc channel
#[derive(Default)]
pub struct State {
    pub sports: SportsSchedule,
    pub transit: TransitPredictions,
    pub weather: WeatherForecast,
    pub mode: Mode,
}

/// A message is sent from background threads to the main thread to modify state
pub enum Message {
    /// Switch to the next tab in the list
    NextMode,
    /// Exit the program
    Quit,
    /// Update sports schedule
    Sports(SportsSchedule),
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
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Weather,
    Transit,
    Sports,
}

impl Mode {
    /// List of all modes
    pub const ALL: [Self; 3] = [Self::Weather, Self::Transit, Self::Sports];

    /// Get the next mode in the list
    pub fn next(self) -> Self {
        let current = Self::ALL.iter().position(|m| *m == self).unwrap();
        Self::ALL[(current + 1) % Self::ALL.len()]
    }
}

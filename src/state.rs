use std::{
    fmt::{self, Display},
    time::Instant,
};

/// TODO
pub struct State {
    pub start: Instant,
    pub now: Instant,
    pub mode: Mode,
}

/// TODO
pub enum Message {
    Quit,
    Time(Instant), // TODO
    SetMode(Mode),
}

/// TODO
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

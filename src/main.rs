mod config;
mod state;
mod transit;
mod util;
mod view;
mod weather;

use crate::{
    config::Config,
    state::{Message, State, Tx},
    util::spawn,
};
use ratatui::{
    DefaultTerminal, TerminalOptions, Viewport,
    crossterm::{
        self,
        event::{self, EnableMouseCapture, Event, KeyCode, KeyEvent},
        terminal::EnterAlternateScreen,
    },
    layout::Rect,
};
use std::{io, sync::mpsc};

const WIDTH: u16 = 32;
const HEIGHT: u16 = 16;

/// Initialize the TUI and start the main loop
fn main() {
    let config = Config::load().unwrap();

    // Restore terminal on exit
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        ratatui::restore();
        original_hook(panic_info);
    }));

    let terminal = ratatui::init_with_options(TerminalOptions {
        // Lock the terminal to the Pi's dimensions
        viewport: Viewport::Fixed(Rect {
            x: 0,
            y: 0,
            width: WIDTH,
            height: HEIGHT,
        }),
    });
    crossterm::execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
    ).unwrap();
    run(config, terminal);

    ratatui::restore();
}

/// TODO explain architecture
fn run(config: Config, mut terminal: DefaultTerminal) {
    let mut state = State::default();

    let (tx, rx) = mpsc::channel();
    let tx = Tx::new(tx);

    // Spawn background tasks
    spawn(&config, &tx, move |_, tx| {
        // Input handler
        loop {
            match event::read() {
                Ok(event) => {
                    if let Some(message) = input_message(event) {
                        tx.send(message);
                    }
                }
                // Input closed - exit
                Err(_) => tx.send(Message::Quit),
            }
        }
    });
    spawn(&config, &tx, transit::transit_loop);
    spawn(&config, &tx, weather::weather_loop);

    loop {
        terminal.draw(|frame| view::draw(frame, &state)).unwrap();
        // Block until we get a message
        match rx.recv().unwrap() {
            Message::Mode(mode) => state.mode = mode,
            Message::Quit => break,
            Message::Transit(transit) => state.transit = transit,
            Message::Weather(weather) => state.weather = weather,
        }
    }
}

/// Handle user input and build the corresponding message. Return `None` if
/// the event should be ignored
fn input_message(event: Event) -> Option<Message> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => Some(Message::Quit),
        _ => None,
    }
}

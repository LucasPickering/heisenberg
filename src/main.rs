//! TODO describe architecture
//!
//! This is a panic-first type program. Most errors are fatal. Anyhow has no
//! power here!!

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
    view::DIMENSIONS,
};
use ratatui::{
    DefaultTerminal, Terminal, TerminalOptions, Viewport,
    crossterm::{
        self,
        event::{
            self, EnableMouseCapture, Event, KeyCode, KeyEvent, MouseButton,
            MouseEvent, MouseEventKind,
        },
        terminal::EnterAlternateScreen,
    },
    prelude::CrosstermBackend,
};
use std::{
    fs::OpenOptions,
    io::{self, Stdout},
    sync::mpsc,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    Layer, filter::Targets, fmt::format::FmtSpan, layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Initialize the TUI and start the main loop
fn main() {
    initialize_tracing();
    let config = Config::load();

    let terminal = initialize_terminal();
    run(config, terminal);
    restore_terminal();
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
            Message::NextMode => state.mode = state.mode.next(),
            Message::Quit => break,
            Message::Transit(transit) => state.transit = transit,
            Message::Weather(weather) => state.weather = weather,
        }
    }
}

fn initialize_tracing() {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./heisenberg.log")
        .unwrap();

    // Basically a minimal version of EnvFilter that doesn't require regexes
    // https://github.com/tokio-rs/tracing/issues/1436#issuecomment-918528013
    let targets: Targets = std::env::var("RUST_LOG")
        .ok()
        .and_then(|env| env.parse().ok())
        .unwrap_or_else(|| {
            Targets::new().with_target("heisenberg", LevelFilter::INFO)
        });
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_span_events(FmtSpan::NONE)
        .with_filter(targets);
    tracing_subscriber::registry().with(file_subscriber).init()
}

fn initialize_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    info!("Initializing terminal");
    // Restore terminal on exit
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_terminal();
        original_hook(panic_info);
    }));

    let terminal = ratatui::init_with_options(TerminalOptions {
        // Lock the terminal to the Pi's dimensions
        viewport: Viewport::Fixed(DIMENSIONS.into()),
    });
    crossterm::execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
    ).unwrap();
    terminal
}

/// Set the terminal like we found it
fn restore_terminal() {
    info!("Restoring terminal");
    ratatui::restore();
}

/// Handle user input and build the corresponding message. Return `None` if
/// the event should be ignored
fn input_message(event: Event) -> Option<Message> {
    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => Some(Message::Quit),
        // Cycle mode on click/tap
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            ..
        }) => Some(Message::NextMode),
        _ => None,
    }
}

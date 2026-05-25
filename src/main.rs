//! A TUI program to be displayed on a Raspberry Pi touchscreen. This uses an
//! ELM-like architecture:
//! - The main thread monitors a message mpsc queue
//! - Background threads are spawned to monitor external state
//! - To modify state, the background threads send messages to the main thread
//! - Whenever state is changed, redraw the terminal
//!
//! This is a panic-first type program. Most errors are fatal. Anyhow has no
//! power here!!

mod config;
mod sports;
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
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
            KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
        },
        terminal::{EnterAlternateScreen, LeaveAlternateScreen},
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

/// Start the main program loop
fn run(config: Config, mut terminal: DefaultTerminal) {
    let mut state = State::default();

    let (tx, rx) = mpsc::channel();
    let tx = Tx::new(tx);

    // Listen for signals. The termination feature is enabled so this
    // catches SIGTERM and SIGHUP as well
    let ctrlc_tx = tx.clone();
    ctrlc::set_handler(move || {
        info!("Quit signal detected");
        ctrlc_tx.send(Message::Quit)
    })
    .unwrap();
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
    spawn(&config, &tx, sports::sports_loop);
    spawn(&config, &tx, transit::transit_loop);
    spawn(&config, &tx, weather::weather_loop);

    loop {
        terminal.draw(|frame| view::draw(frame, &state)).unwrap();
        // Block until we get a message
        match rx.recv().unwrap() {
            Message::NextMode => state.mode = state.mode.next(),
            Message::Quit => break,
            Message::Sports(sports) => state.sports = sports,
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

    crossterm::terminal::enable_raw_mode().unwrap();
    crossterm::execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
    ).unwrap();
    Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            // Lock the terminal to the Pi's dimensions
            viewport: Viewport::Fixed(DIMENSIONS.into()),
        },
    )
    .unwrap()
}

/// Set the terminal like we found it
fn restore_terminal() {
    info!("Restoring terminal");
    let _ = crossterm::terminal::disable_raw_mode();
    let _ = crossterm::execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    );
}

/// Handle user input and build the corresponding message. Return `None` if
/// the event should be ignored
fn input_message(event: Event) -> Option<Message> {
    match event {
        // esc, q, or ctrl-c exits
        Event::Key(KeyEvent {
            code: KeyCode::Esc | KeyCode::Char('q'),
            ..
        }) => Some(Message::Quit),
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers,
            ..
        }) if modifiers.contains(KeyModifiers::CONTROL) => Some(Message::Quit),
        // Cycle mode on space/click/tap
        Event::Key(KeyEvent {
            code: KeyCode::Char(' '),
            ..
        })
        | Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            ..
        }) => Some(Message::NextMode),
        _ => None,
    }
}

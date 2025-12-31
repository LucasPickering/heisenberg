mod config;
mod state;
mod view;

use crate::{
    config::Config,
    state::{Message, Mode, State},
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
use std::{
    io,
    sync::mpsc::{self, Sender},
    thread,
    time::{Duration, Instant},
};

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
    let mut state = State {
        mode: Mode::Weather,
        start: Instant::now(),
        now: Instant::now(),
    };

    let (tx, rx) = mpsc::channel();

    // Spawn background tasks
    spawn(&tx, move |tx| {
        loop {
            thread::sleep(Duration::from_millis(100));
            tx.send(Message::Time(Instant::now()))?;
        }
    });
    spawn(&tx, move |tx| {
        // Input handler
        loop {
            let event = event::read()?;
            if let Some(message) = input_message(event) {
                tx.send(message)?;
            }
        }
    });

    loop {
        terminal.draw(|frame| view::draw(frame, &state)).unwrap();
        // Block until we get a message
        match rx.recv().unwrap() {
            Message::Quit => break,
            Message::SetMode(mode) => state.mode = mode,
            Message::Time(time) => state.now = time,
        }
    }
}

/// Message sender channel
type Tx = Sender<Message>;

/// Spawn a background thread with access to the message channel
fn spawn(tx: &Tx, f: impl 'static + FnOnce(Tx) -> anyhow::Result<()> + Send) {
    let tx = tx.clone();
    thread::spawn(move || {
        let result = f(tx);
        // TODO log result
    });
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

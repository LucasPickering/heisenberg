use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{
        self,
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode,
            KeyEvent,
        },
    },
    widgets::List,
};
use std::io;

const WIDTH: u16 = 32;
const HEIGHT: u16 = 16;

fn main() {
    let terminal = ratatui::init();
    crossterm::execute!(io::stdout(), EnableMouseCapture).unwrap();
    run(terminal);
    crossterm::execute!(io::stdout(), DisableMouseCapture).unwrap();
    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal) {
    let mut state = State {
        messages: vec![
            "----5----1----5----2----5----3----5----4".into(),
            "Esc to exit".into(),
        ],
    };
    loop {
        terminal.draw(|frame| render(frame, &state)).unwrap();
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => break,
            event => state.messages.push(format!("{event:?}")),
        }
    }
}

fn render(frame: &mut Frame, state: &State) {
    let list = List::new(state.messages.iter().map(String::as_str));
    frame.render_widget(list, frame.area());
}

struct State {
    messages: Vec<String>,
}

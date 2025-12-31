use crate::{State, state::Mode};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Tabs,
};

/// Draw to the terminal
pub fn draw(frame: &mut Frame, state: &State) {
    let [mode_area, time_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
            .areas(frame.area());

    frame.render_widget(
        Tabs::new(Mode::ALL.iter().map(Mode::to_string))
            .select(index_of(&Mode::ALL, state.mode)),
        mode_area,
    );

    let uptime = state.now - state.start;
    frame.render_widget(
        format!("Uptime: {:.1}s", uptime.as_secs_f32()),
        time_area,
    );
}

/// Get the index of a valid within a slice
fn index_of<T: PartialEq>(list: &[T], value: T) -> Option<usize> {
    list.iter().position(|v| *v == value)
}

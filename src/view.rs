use crate::{
    State, state::Mode, transit::TransitPredictions, weather::WeatherForecast,
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect, Size},
    style::{Color, Stylize},
    widgets::{Block, Tabs},
};
use std::iter;

/// Display width
pub const DIMENSIONS: Size = Size {
    width: 32,
    height: 16,
};

/// Draw to the terminal
pub fn draw(frame: &mut Frame, state: &State) {
    // Draw background color. Makes it easier to visualize the size in dev mode
    frame.render_widget(Block::new().bg(Color::DarkGray), frame.area());

    let [mode_area, content_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
            .areas(frame.area());

    frame.render_widget(
        Tabs::new(Mode::ALL.iter().map(Mode::to_string))
            .select(index_of(&Mode::ALL, state.mode)),
        mode_area,
    );

    match state.mode {
        Mode::Transit => draw_transit(frame, &state.transit, content_area),
        Mode::Weather => draw_weather(frame, &state.weather, content_area),
    }
}

fn draw_transit(frame: &mut Frame, transit: &TransitPredictions, area: Rect) {
    for line in &transit.lines {
        frame.render_widget(
            format!("{}\n{}\n{}\n", line.name, line.inbound, line.outbound),
            area,
        );
    }
}

fn draw_weather(frame: &mut Frame, weather: &WeatherForecast, area: Rect) {
    let [now_area, areas @ ..] =
        Layout::vertical(iter::repeat_n(Constraint::Length(1), 5))
            .areas::<5>(area);

    // Draw current weather
    let Some(now) = weather.now() else {
        frame.render_widget("No weather data available", now_area);
        return;
    };
    frame.render_widget(
        format!("{} {}", now.temperature(), now.prob_of_precip()),
        now_area,
    );

    // Draw upcoming periods
    for (period, area) in weather.future_periods().take(areas.len()).zip(areas)
    {
        frame.render_widget(
            format!(
                "{} {:>4} {:>4}",
                period.start_time().format("%_I%P"),
                period.temperature(),
                period.prob_of_precip(),
            ),
            area,
        );
    }
}

/// Get the index of a valid within a slice
fn index_of<T: PartialEq>(list: &[T], value: T) -> Option<usize> {
    list.iter().position(|v| *v == value)
}

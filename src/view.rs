use crate::{
    State,
    state::Mode,
    transit::{LinePredictions, StopPredictions, TransitPredictions},
    weather::WeatherForecast,
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Tabs, Widget},
};
use std::{iter, sync::LazyLock};

/// Display width
pub const DIMENSIONS: Size = Size {
    width: 32,
    height: 16,
};
/// Styles are statically defined, so we only need one copy
static STYLES: LazyLock<Styles> = LazyLock::new(Styles::default);

/// Draw to the terminal
pub fn draw(frame: &mut Frame, state: &State) {
    // Draw background color. Makes it easier to visualize the size in dev mode
    frame.render_widget(Block::new().bg(Color::DarkGray), frame.area());

    let [mode_area, content_area] =
        Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
            .areas(frame.area());

    frame.render_widget(
        Tabs::new(Mode::ALL.iter().map(Mode::to_string))
            .select(index_of(&Mode::ALL, state.mode))
            .highlight_style(STYLES.tab_highlight),
        mode_area,
    );

    match state.mode {
        Mode::Transit => frame.render_widget(&state.transit, content_area),
        Mode::Weather => frame.render_widget(&state.weather, content_area),
    }
}

impl Widget for &TransitPredictions {
    fn render(self, area: Rect, buf: &mut Buffer) {
        fn line_to_lines(
            line: &LinePredictions,
        ) -> impl Iterator<Item = Line<'_>> {
            // One row for the line label, then another row for each stop
            iter::once(
                Line::from(line.name.as_str()).style(STYLES.transit_line_name),
            )
            .chain(line.stops.iter().map(stop_to_line))
        }

        fn stop_to_line(stop: &StopPredictions) -> Line<'_> {
            Line::from(format!("{:>7} {}", stop.name, stop.predictions))
        }

        let text: Text = self.lines.iter().flat_map(line_to_lines).collect();
        text.render(area, buf);
    }
}

impl Widget for &WeatherForecast {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [now_area, areas @ ..] =
            Layout::vertical(iter::repeat_n(Constraint::Length(1), 5))
                .areas::<5>(area);

        // Draw current weather
        let Some(now) = self.now() else {
            Widget::render("No weather data available", now_area, buf);
            return;
        };
        Widget::render(
            format!("{} {}", now.temperature(), now.prob_of_precip()),
            now_area,
            buf,
        );

        // Draw upcoming periods
        for (period, area) in self.future_periods().take(areas.len()).zip(areas)
        {
            Widget::render(
                format!(
                    "{} {:>4} {:>4}",
                    period.start_time().format("%_I%P"),
                    period.temperature(),
                    period.prob_of_precip(),
                ),
                area,
                buf,
            );
        }
    }
}

/// Get the index of a valid within a slice
fn index_of<T: PartialEq>(list: &[T], value: T) -> Option<usize> {
    list.iter().position(|v| *v == value)
}

/// All styling rules
struct Styles {
    tab_highlight: Style,
    transit_line_name: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            tab_highlight: Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            transit_line_name: Style::default().add_modifier(Modifier::BOLD),
        }
    }
}

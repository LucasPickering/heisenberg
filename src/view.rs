use crate::{
    State,
    state::Mode,
    transit::{LinePredictions, StopPredictions, TransitPredictions},
    weather::{ForecastPeriod, WeatherForecast},
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Tabs, Widget},
};
use std::{iter, sync::LazyLock};

/// Display width
pub const DIMENSIONS: Size = Size {
    width: 18,
    height: 10,
};
/// Styles are statically defined, so we only need one copy
static STYLES: LazyLock<Styles> = LazyLock::new(Styles::default);

/// Draw to the terminal
pub fn draw(frame: &mut Frame, state: &State) {
    let [mode_area, _, content_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(0),
    ])
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
        /// Convert a transit line into a text line
        fn line_to_lines(
            line: &LinePredictions,
        ) -> impl Iterator<Item = Line<'_>> {
            // One row for the line label, then another row for each stop
            iter::once(
                Line::from(line.name.as_str()).style(STYLES.transit_line_name),
            )
            .chain(line.stops.iter().map(stop_to_line))
            .chain(iter::once("".into())) // Blank line between
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
        /// Convert a forecast period into a line of text
        fn period_to_line(
            name: &str,
            period: &ForecastPeriod,
        ) -> Line<'static> {
            Line::from_iter([
                Span::styled(format!("{name:>4}"), STYLES.weather_period),
                Span::from(format!("{:>4}", period.temperature())),
                Span::from(format!("{:>4}", period.prob_of_precip(),)),
            ])
        }

        let [now_area, areas @ ..] =
            Layout::vertical(iter::repeat_n(Constraint::Length(1), 5))
                .areas::<5>(area);

        // Draw current weather
        let Some(now) = self.now() else {
            Widget::render("No weather data available", now_area, buf);
            return;
        };
        Widget::render(period_to_line("Now", now), now_area, buf);

        // Draw upcoming periods
        for (period, area) in self.future_periods().take(areas.len()).zip(areas)
        {
            Widget::render(
                period_to_line(
                    &period.start_time().format("%_I%P").to_string(),
                    period,
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
    /// Highlighted tab name
    tab_highlight: Style,
    /// Transit line names (e.g. "86")
    transit_line_name: Style,
    /// Weather period name (e.g. "5pm")
    weather_period: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            tab_highlight: Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            transit_line_name: Style::default().add_modifier(Modifier::BOLD),
            weather_period: Style::default().add_modifier(Modifier::BOLD),
        }
    }
}

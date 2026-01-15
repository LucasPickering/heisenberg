use crate::{
    State,
    state::Mode,
    transit::{LinePredictions, StopPredictions, TransitPredictions},
    util::scale_to,
    weather::WeatherForecast,
};
use itertools::{Itertools, MinMaxResult};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect, Size},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Text},
    widgets::{Axis, Chart, Dataset, GraphType, Tabs, Widget},
};
use std::{iter, sync::LazyLock};

/// Display width
pub const DIMENSIONS: Size = Size {
    width: 24,
    height: 12,
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
        const PERIODS: usize = 25;

        let (labels, temps, precips): (Vec<_>, Vec<_>, Vec<_>) = self
            .periods()
            .take(PERIODS)
            .map(|period| {
                let time = period.start_time();
                let x = period.start_time().timestamp() as f64;
                let label = {
                    let mut label = time.format("%-I%P").to_string();
                    label.pop(); // Remove the 'm' from 'am'/'pm'
                    label
                };
                let temp = (x, period.temp() as f64);
                let precip = (x, period.pop() as f64);
                (label, temp, precip)
            })
            .multiunzip();

        // Calculate x/y bounds based on the temperature data. Both lines will
        // have the same x values. Bound the y based on temperature, so it zooms
        // in as much as possible. The precip line will still be 0-100, but
        // without labels
        let (min_x, max_x, min_temp, max_temp) = if temps.is_empty() {
            (0.0, 0.0, 0.0, 0.0)
        } else {
            let min_x = temps.first().unwrap().0;
            let max_x = temps.last().unwrap().0;
            let (min_temp, max_temp) =
                match temps.iter().map(|(_, temp)| *temp).minmax() {
                    MinMaxResult::NoElements => (0.0, 0.0),
                    MinMaxResult::OneElement(value) => (value, value),
                    MinMaxResult::MinMax(min, max) => (min, max),
                };
            (min_x, max_x, min_temp, max_temp)
        };

        // Scale the precip values to be in the temperature y range. This will
        // make the dots visually equivalent to being on their own 0-100 scale
        let precips: Vec<_> = precips
            .into_iter()
            .map(|(x, y)| {
                let y = scale_to(y, (0., 100.), (min_temp, max_temp));
                (x, y)
            })
            .collect();

        // Build the lines from temp/precip data
        let datasets = vec![
            Dataset::default()
                .name("precip")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(STYLES.weather_line_precipitation)
                .data(&precips),
            Dataset::default()
                .name("temp")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(STYLES.weather_line_temperature)
                .data(&temps),
        ];

        // Build the axes
        let x_axis = Axis::default()
            .style(Style::default().white())
            .bounds([min_x, max_x])
            // 4 evenly spaced labels
            .labels(labels.into_iter().step_by(PERIODS / 3));
        let y_axis = Axis::default()
            .style(Style::default().white())
            .bounds([min_temp, max_temp])
            .labels([format!("{min_temp:.0}°"), format!("{max_temp:.0}°")])
            .labels_alignment(Alignment::Right);

        // Create the chart and link all the parts together
        let chart = Chart::new(datasets).x_axis(x_axis).y_axis(y_axis);

        chart.render(area, buf);
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
    /// Precipitation line on the weather graph
    weather_line_precipitation: Style,
    /// Temperature line on the weather graph
    weather_line_temperature: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            tab_highlight: Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            transit_line_name: Style::default().add_modifier(Modifier::BOLD),
            weather_line_precipitation: Style::default().blue(),
            weather_line_temperature: Style::default().red(),
        }
    }
}

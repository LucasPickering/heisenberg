use crate::{
    state::{Mode, State},
    transit::{LinePredictions, StopPredictions, TransitPredictions},
    weather::WeatherForecast,
};
use itertools::Itertools;
use xilem::{
    WidgetView,
    masonry::properties::types::Length,
    view::{flex_col, flex_item, flex_row, label, sized_box, text_button},
};

/// Render the GUI based on state
pub fn app_logic(state: &mut State) -> impl WidgetView<State> + use<> {
    // Top nav tabs
    let tabs = flex_row(
        Mode::ALL
            .into_iter()
            .map(|mode| flex_item(mode_tab(mode, mode == state.mode), 1.0))
            .collect_vec(),
    )
    .gap(Length::ZERO);

    let content = match state.mode {
        Mode::Weather => weather(&state.weather).boxed(),
        Mode::Transit => transit(&state.transit).boxed(),
    };
    flex_col((tabs, content))
}

/// Render a single tab in the mode selector up top
fn mode_tab(mode: Mode, is_active: bool) -> impl WidgetView<State> {
    sized_box(text_button(mode.to_string(), move |state: &mut State| {
        state.mode = mode
    }))
    .expand_width()
    .height(Length::px(40.0))
}

/// Render the weather forecast
fn weather(forecast: &WeatherForecast) -> impl WidgetView<State> + use<> {
    let rows = forecast.periods().map(|period| {
        label(format!(
            "{} {:.0}° {:.0}%",
            period.start_time(),
            period.temperature(),
            period.prob_of_precip(),
        ))
    });
    flex_col(rows.collect_vec())
}

/// Render the transit predictions
fn transit(predictions: &TransitPredictions) -> impl WidgetView<State> + use<> {
    fn line(line: &LinePredictions) -> impl WidgetView<State> + use<> {
        flex_row((
            label(line.name.clone()),
            flex_col(line.stops.iter().map(stop).collect_vec()),
        ))
    }

    fn stop(stop: &StopPredictions) -> impl WidgetView<State> + use<> {
        label(format!("{} {}", stop.name, stop.predictions))
    }

    flex_col(predictions.lines.iter().map(line).collect_vec())
}

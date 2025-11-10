import gleam/time/calendar
import gleam/time/timestamp.{type Timestamp}
import lustre/attribute as attr
import lustre/element.{type Element}
import lustre/element/html
import model.{type GlobalModel, type Transit, type Weather}
import page.{type Page}

/// Generate HTML content for the given page
pub fn view(page: Page, model: GlobalModel) -> Element(Nil) {
  let header =
    html.header([attr.class("nav")], [
      tab("/transit", "Transit", page == page.Transit),
      tab("/weather", "Weather", page == page.Weather),
    ])
  let main = case page {
    page.Transit -> transit(model.transit)
    page.Weather -> weather(model.weather)
  }
  html.div([], [
    header,
    html.div([attr.class("main")], [main]),
    footer(model.last_updated),
  ])
}

/// Transit page HTML
fn transit(transit: Transit) -> Element(Nil) {
  html.div([], [html.text("Transit!")])
}

/// Weather page HTML
fn weather(weather: Weather) -> Element(Nil) {
  html.div([], [html.text("Weather!")])
}

/// Create a nav tab in the header
fn tab(path: String, text: String, active: Bool) -> Element(Nil) {
  html.a([attr.href(path)], [html.text(text)])
}

fn footer(last_updated: Timestamp) -> Element(Nil) {
  let last_updated = timestamp.to_rfc3339(last_updated, calendar.utc_offset)
  html.footer([attr.class("footer")], [
    html.text("Last updated " <> last_updated),
  ])
}

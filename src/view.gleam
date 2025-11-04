import lustre/attribute as attr
import lustre/element.{type Element}
import lustre/element/html
import page.{type Page}

/// Generate HTML content for the given page
pub fn view(page: Page) -> Element(Nil) {
  let header =
    html.header([attr.class("nav")], [
      tab("/transit", "Transit", page == page.Transit),
      tab("/weather", "Weather", page == page.Weather),
    ])
  let footer =
    html.footer([attr.class("footer")], [html.text("Last updated TODO")])
  let main = case page {
    page.Transit -> transit()
    page.Weather -> weather()
  }
  html.div([], [header, html.div([attr.class("main")], [main]), footer])
}

/// Transit page HTML
fn transit() -> Element(Nil) {
  html.div([], [html.text("Transit!")])
}

/// Weather page HTML
fn weather() -> Element(Nil) {
  html.div([], [html.text("Weather!")])
}

/// Create a nav tab in the header
fn tab(path: String, text: String, active: Bool) -> Element(Nil) {
  html.a([attr.href(path)], [html.text(text)])
}

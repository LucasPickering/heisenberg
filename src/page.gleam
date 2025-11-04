/// Top-level page
pub type Page {
  Transit
  Weather
}

/// Parse a page name
pub fn parse(page: String) -> Result(Page, Nil) {
  case page {
    "transit" -> Ok(Transit)
    "weather" -> Ok(Weather)
    _ -> Error(Nil)
  }
}

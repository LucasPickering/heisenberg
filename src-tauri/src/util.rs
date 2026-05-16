use serde::de::DeserializeOwned;
use tracing::{error, info};

/// Make an HTTP GET request
pub fn http_get<T: DeserializeOwned>(url: &str) -> Result<T, ()> {
    info!("Fetching {url}");
    match ureq::get(url).call() {
        Ok(mut response) if response.status().is_success() => {
            let data: T = response.body_mut().read_json().unwrap();
            Ok(data)
        }
        Ok(response) => {
            error!(
                status = %response.status(),
                "4xx/5xx response from {url}"
            );
            Err(())
        }
        Err(error) => {
            error!(%error, "Error fetching {url}");
            Err(())
        }
    }
}

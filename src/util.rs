use crate::{config::Config, state::Tx};
use serde::de::DeserializeOwned;
use std::thread;
use tracing::error;

/// Spawn a background thread with access to the message channel
pub fn spawn(
    config: &Config,
    tx: &Tx,
    f: impl 'static + FnOnce(Config, Tx) + Send,
) {
    let config = config.clone();
    let tx = tx.clone();
    thread::spawn(move || f(config, tx));
}

/// Make an HTTP GET request
pub fn http_get<T: DeserializeOwned>(url: &str) -> Result<T, ()> {
    match ureq::get(url).call() {
        Ok(mut response) if response.status().is_success() => {
            let data: T = response.body_mut().read_json().expect("TODO");
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

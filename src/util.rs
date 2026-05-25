use crate::{config::Config, state::Tx};
use serde::de::DeserializeOwned;
use std::{error::Error, thread};
use tracing::{error, info};
use ureq::{RequestBuilder, http::Uri, typestate::WithoutBody};

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

/// Make an HTTP request
pub fn http<T: DeserializeOwned>(
    request: RequestBuilder<WithoutBody>,
) -> Result<T, ()> {
    let url = request.uri_ref().map(Uri::to_string).unwrap_or_default();
    info!("Fetching {url}");
    match request.call() {
        Ok(mut response) if response.status().is_success() => {
            let data: T = response.body_mut().read_json().map_err(|error| {
                error!(
                    url,
                    error = &error as &dyn Error,
                    "Error decoding JSON body"
                );
            })?;
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

/// Scale from one numeric range to another
pub fn scale_to(value: f64, from: (f64, f64), to: (f64, f64)) -> f64 {
    let from_span = from.1 - from.0;
    let to_span = to.1 - to.0;
    (value - (from.0)) / from_span * to_span + to.0
}

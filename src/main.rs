//! A GUI program to be displayed on a Raspberry Pi touchscreen
//!
//! This is a panic-first type program. Most errors are fatal. Anyhow has no
//! power here!!

mod config;
mod state;
mod transit;
mod util;
mod view;
mod weather;

use crate::{config::Config, state::State};
use std::fs::OpenOptions;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer, filter::Targets, fmt::format::FmtSpan, layer::SubscriberExt,
    util::SubscriberInitExt,
};
use xilem::{
    EventLoop, WindowOptions, Xilem, dpi::LogicalSize,
    winit::error::EventLoopError,
};

fn main() -> Result<(), EventLoopError> {
    // TODO fix verbose logging
    //     initialize_tracing();
    let config = Config::load();
    // spawn(&config, &tx, transit::transit_loop);
    // spawn(&config, &tx, weather::weather_loop);
    let app = Xilem::new_simple(
        State::default(),
        view::app_logic,
        WindowOptions::new("Heisenberg")
            .with_resizable(false)
            .with_initial_inner_size(LogicalSize::new(400, 400)),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}

fn initialize_tracing() {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./heisenberg.log")
        .unwrap();

    // Basically a minimal version of EnvFilter that doesn't require regexes
    // https://github.com/tokio-rs/tracing/issues/1436#issuecomment-918528013
    let targets: Targets = std::env::var("RUST_LOG")
        .ok()
        .and_then(|env| env.parse().ok())
        .unwrap_or_else(|| {
            Targets::new().with_target("heisenberg", LevelFilter::INFO)
        });
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_span_events(FmtSpan::NONE)
        .with_filter(targets);
    tracing_subscriber::registry().with(file_subscriber).init()
}

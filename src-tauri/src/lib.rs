use crate::config::Config;
use std::thread;
use tauri::{App, AppHandle};

mod config;
mod transit;
mod util;
mod weather;

pub fn run() {
    let config = Config::load();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app| {
            // TODO make this async?
            spawn(&config, app, transit::transit_loop);
            spawn(&config, app, weather::weather_loop);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn spawn(
    config: &Config,
    app: &App,
    f: impl 'static + FnOnce(Config, AppHandle) + Send,
) {
    let config = config.clone();
    let handle = app.handle().clone();
    thread::spawn(move || f(config, handle));
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

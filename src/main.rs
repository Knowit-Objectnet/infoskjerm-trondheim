extern crate chrono;

use chrono::{Local, Locale};
use log::info;
use rust_embed::RustEmbed;
use slint::{PlatformError, Timer, TimerMode};
use ui::*;

mod food;
mod forecast;
mod xkcd;
mod calendar;

pub mod ui {
    slint::include_modules!();
}

// we embed img folder into the compiled binary for simpler distribution
#[derive(RustEmbed)]
#[folder = "img/"]
struct StaticAssets;

fn main() -> Result<(), PlatformError> {
    env_logger::init();
    info!("Starting up...");

    let main_window = MainWindow::new().unwrap();

    forecast::setup(&main_window);
    xkcd::setup(&main_window);
    food::setup(&main_window);
    calendar::setup(&main_window);

    let clock_timer = Timer::default();
    let clock_handle = main_window.as_weak();

    clock_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            let ui = clock_handle.unwrap();
            let now = Local::now();
            let time = now.format("%H:%M").to_string();
            let mut date = now.format_localized("%A %e. %B", Locale::nb_NO).to_string();

            // Norske dager har ikke stor forbokstav, men vi ønsker det siden det
            // er første ordet i en "setning"
            if let Some(ch) = date.chars().next() {
                let capitalized = ch.to_uppercase().to_string();
                date.replace_range(..1, capitalized.as_str());
            }

            ui.set_time(time.into());
            ui.set_date(date.into());
        },
    );

    main_window.run()
}
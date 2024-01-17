use rust_embed::RustEmbed;

extern crate chrono;

use chrono::Local;
use log::info;
use slint::{Timer, TimerMode};

mod weather;
mod xkcd;

pub mod ui {
    slint::include_modules!();
}

use slint::*;
use ui::*;

// we embed img folder into the compiled binary for simpler distribution
#[derive(RustEmbed)]
#[folder = "img/"]
struct StaticAssets;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    info!("Starting up...");

    let main_window = MainWindow::new().unwrap();

    weather::setup(&main_window);
    xkcd::setup(&main_window);

    let clock_timer = Timer::default();
    let clock_handle = main_window.as_weak();

    clock_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            let ui = clock_handle.unwrap();
            let now = Local::now();
            let datestring = std::format!("{}", now.format("%H:%M:%S"));
            let date = now.format("%d").to_string().into();
            let month = now.format("%b").to_string().to_uppercase().into();
            ui.set_time(datestring.into());
            ui.set_month(month);
            ui.set_date(date);
        },
    );

    main_window.run()
}

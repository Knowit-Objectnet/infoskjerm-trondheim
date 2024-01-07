use rust_embed::RustEmbed;
slint::include_modules!();

extern crate chrono;

use std::rc::Rc;

use chrono::Local;
use log::{error, info};
use slint::{Timer, TimerMode};

mod weather;
mod xkcd;

use crate::weather::*;
use crate::xkcd::*;

// we embed img folder into the compiled binary for simpler distribution
#[derive(RustEmbed)]
#[folder = "img/"]
struct StaticAssets;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    info!("Starting up...");

    let ui = MainWindow::new()?;
    let clock_timer = Timer::default();
    let xkcd_timer = Timer::default();
    let weather_timer = Timer::default();

    let clock_handle = ui.as_weak();
    let xkcd_handle = ui.as_weak();
    let weather_handle = ui.as_weak();

    clock_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            let ui = clock_handle.unwrap();
            let now = Local::now();
            let datestring = format!("{}", now.format("%H:%M:%S"));
            let date = now.format("%d").to_string().into();
            let month = now.format("%b").to_string().to_uppercase().into();
            ui.set_time(datestring.into());
            ui.set_month(month);
            ui.set_date(date);
        },
    );

    match get_current_xkcd() {
        Ok(xkcd) => {
            ui.set_xkcd(xkcd);
            info!("Initial xkcd set")
        }
        Err(e) => {
            error!("Error setting initial XKCD: {}", e)
        }
    }

    xkcd_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(3600 * 2),
        move || {
            let ui = xkcd_handle.unwrap();
            match get_current_xkcd() {
                Ok(xkcd) => ui.set_xkcd(xkcd),
                Err(e) => eprintln!("{}", e),
            }
        },
    );

    match get_forecast() {
        Ok(forecasts) => {
            ui.set_forecasts(Rc::new(forecasts).into());
            info!("Initial weather set")
        }
        Err(e) => error!("Error setting initial forecast{}", e),
    }

    weather_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(900),
        move || {
            let ui = weather_handle.unwrap();
            match get_forecast() {
                Ok(forecasts) => ui.set_forecasts(Rc::new(forecasts).into()),
                Err(e) => eprintln!("{}", e),
            }
        },
    );

    ui.run()
}

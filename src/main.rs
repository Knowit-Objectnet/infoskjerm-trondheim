slint::include_modules!();

extern crate chrono;

use std::rc::Rc;

use chrono::Local;
use slint::{Timer, TimerMode};

mod weather;
mod xkcd;

use crate::weather::*;
use crate::xkcd::*;

fn main() -> Result<(), slint::PlatformError> {
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
            let date = Local::now();
            let datestring = format!("{}", date.format("%H:%M:%S"));
            ui.set_time(datestring.into());
        },
    );

    let xkcd = get_current_xkcd();
    ui.set_xkcdTitle(xkcd.title.into());
    ui.set_xkcdFlavorText(xkcd.flavor_text.into());
    ui.set_xkcdImage(xkcd.image);

    xkcd_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(3600 * 24),
        move || {
            let ui = xkcd_handle.unwrap();
            let xkcd = get_current_xkcd();
            ui.set_xkcdTitle(xkcd.title.into());
            ui.set_xkcdImage(xkcd.image);
            ui.set_xkcdFlavorText(xkcd.flavor_text.into());
        },
    );

    let forecasts = get_forecast();
    ui.set_forecasts(Rc::new(forecasts).into());

    weather_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(900),
        move || {
            let ui = weather_handle.unwrap();
            let forecasts = get_forecast();
            ui.set_forecasts(Rc::new(forecasts).into());
        },
    );

    ui.run()
}

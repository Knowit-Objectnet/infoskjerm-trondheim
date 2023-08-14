slint::include_modules!();

extern crate chrono;

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

    let ui_handle = ui.as_weak();
    let ui_handle2 = ui.as_weak();
    let ui_handle3 = ui.as_weak();

    clock_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(1000),
        move || {
            let ui = ui_handle.unwrap();
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
            let ui = ui_handle2.unwrap();

            let xkcd = get_current_xkcd();
            ui.set_xkcdTitle(xkcd.title.into());
            ui.set_xkcdImage(xkcd.image);
            ui.set_xkcdFlavorText(xkcd.flavor_text.into());
        },
    );

    weather_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(2),
        move || {
            let ui = ui_handle3.unwrap();
            let forecast = get_forecast();
            ui.set_weather(forecast.foo.into());
        },
    );

    ui.run()
}

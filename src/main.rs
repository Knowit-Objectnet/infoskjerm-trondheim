slint::include_modules!();

extern crate chrono;

use chrono::Local;
use slint::{Timer, TimerMode};

mod xkcd;
use crate::xkcd::*;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let clock_timer = Timer::default();
    let xkcd_timer = Timer::default();

    let ui_handle = ui.as_weak();
    let ui_handle2 = ui.as_weak();

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

    xkcd_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(1),
        move || {
            let ui = ui_handle2.unwrap();
            ui.set_xkcdTitle("jeje".into());

            let image = get_current_xkcd_image();
            ui.set_xkcdImage(image);
        },
    );

    ui.run()
}

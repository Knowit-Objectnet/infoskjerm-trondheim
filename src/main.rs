slint::include_modules!();
use slint::{Timer, TimerMode};
extern crate chrono;

use chrono::Local;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let clock_timer = Timer::default();
    let ui_handle = ui.as_weak();

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

    ui.run()
}

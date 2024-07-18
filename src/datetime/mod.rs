use crate::ui::*;
use chrono::{Local, Locale};
use slint::{Timer, TimerMode};

pub fn setup(main_window: &MainWindow) -> Timer {
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

    // We return the timer to prevent it from being dropped
    clock_timer
}

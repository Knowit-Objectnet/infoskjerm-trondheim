/*
This module adds seasonal fluff:
Halloween bats and sppoky face in late october
Snow effect in December
....more? :D
*/

use crate::ui::*;
use chrono::{Datelike, Local};
use slint::Timer;
mod december;
mod halloween;
use december::setup_snow;
use halloween::{setup_halloween_bat, setup_halloween_spooky_face};

pub fn setup_seasons(main_window: &MainWindow) -> Vec<Timer> {
    let mut season_timers = vec![];

    if Local::now().month() == 12 {
        let snow_timer = setup_snow(main_window);
        season_timers.push(snow_timer);
    }

    let now = Local::now();
    if now.month() == 10 && now.day() >= 10 && now.day() <= 31 {
        let face_timer = setup_halloween_spooky_face(main_window);
        let bat_timer = setup_halloween_bat(main_window);

        season_timers.push(face_timer);
        season_timers.push(bat_timer);
    }

    season_timers
}

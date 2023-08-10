slint::include_modules!();
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, Timer, TimerMode};
extern crate chrono;

use chrono::Local;

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
        std::time::Duration::from_secs(10),
        move || {
            let ui = ui_handle2.unwrap();
            ui.set_xkcdTitle("jeje".into());

            let img_bytes = reqwest::blocking::get("https://imgs.xkcd.com/comics/what_to_do.png")
                .unwrap()
                .bytes()
                .unwrap();

            let mut pixmap = tiny_skia::PixmapMut::from_bytes(img_bytes, width, height).unwrap();

            ui.set_xkcdImage(image);
        },
    );

    ui.run()
}

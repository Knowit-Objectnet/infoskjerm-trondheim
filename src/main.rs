slint::include_modules!();
use image::{io::Reader as ImageReader, ColorType, GenericImageView};

use slint::{Image, Rgba8Pixel, SharedPixelBuffer, Timer, TimerMode};
use std::io::Cursor;
use tiny_skia::PixmapMut;

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

            let response = reqwest::blocking::get("https://imgs.xkcd.com/comics/what_to_do_2x.png")
                .expect("Failed to download image");

            let image_data = response.bytes().expect("Failed to read image data");

            // Wrap the image data in a `Cursor` to allow reading from it
            let cursor = Cursor::new(image_data.as_ref());

            // Decode the image into a `RgbaImage` from the `image` crate
            let dynamic_image = ImageReader::new(cursor)
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap();
            let rgba_image = dynamic_image.into_rgba8();

            let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                rgba_image.as_raw(),
                rgba_image.width(),
                rgba_image.height(),
            );
            let image = Image::from_rgba8(buffer);
            ui.set_xkcdImage(image);
        },
    );

    ui.run()
}

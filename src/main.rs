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

            let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(640, 480);
            let width = pixel_buffer.width();
            let height = pixel_buffer.height();
            let mut pixmap =
                tiny_skia::PixmapMut::from_bytes(pixel_buffer.make_mut_bytes(), width, height)
                    .unwrap();
            pixmap.fill(tiny_skia::Color::TRANSPARENT);

            let circle = tiny_skia::PathBuilder::from_circle(320., 240., 150.).unwrap();

            let mut paint = tiny_skia::Paint::default();
            paint.shader = tiny_skia::LinearGradient::new(
                tiny_skia::Point::from_xy(100.0, 100.0),
                tiny_skia::Point::from_xy(400.0, 400.0),
                vec![
                    tiny_skia::GradientStop::new(
                        0.0,
                        tiny_skia::Color::from_rgba8(50, 127, 150, 200),
                    ),
                    tiny_skia::GradientStop::new(
                        1.0,
                        tiny_skia::Color::from_rgba8(220, 140, 75, 180),
                    ),
                ],
                tiny_skia::SpreadMode::Pad,
                tiny_skia::Transform::identity(),
            )
            .unwrap();

            pixmap.fill_path(
                &circle,
                &paint,
                tiny_skia::FillRule::Winding,
                Default::default(),
                None,
            );

            let image = Image::from_rgba8_premultiplied(pixel_buffer);

            ui.set_xkcdImage(image);
        },
    );

    ui.run()
}

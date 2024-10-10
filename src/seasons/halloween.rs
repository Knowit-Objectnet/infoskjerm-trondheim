use crate::{ui::*, StaticAssets};
use rand::Rng;
use slint::{Image, LogicalPosition, Rgba8Pixel, SharedPixelBuffer, Timer, TimerMode};

pub fn setup_halloween_spooky_face(main_window: &MainWindow) -> Timer {
    let halloween_timer = Timer::default();
    let halloween_handle = main_window.as_weak();

    let width = main_window.window().size().width;
    let height = main_window.window().size().height;
    let mut rng = rand::thread_rng();

    halloween_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(10),
        move || {
            let mut current_spooky = halloween_handle.unwrap().get_spooky_face();
            if current_spooky.hidden {
                let face_width = rng.gen_range(100..500);
                let x_pos = rng.gen_range(0..width - (face_width + 20));
                let y_pos = rng.gen_range(0..height - (face_width + 20));
                let spooky_face = SpookyFace {
                    pos: LogicalPosition {
                        x: x_pos as f32,
                        y: y_pos as f32,
                    },
                    size: face_width as f32,
                    hidden: false,
                };
                halloween_handle.unwrap().set_spooky_face(spooky_face);
            } else {
                current_spooky.hidden = true;
                halloween_handle.unwrap().set_spooky_face(current_spooky);
            }
        },
    );

    halloween_timer
}

pub fn setup_halloween_bat(main_window: &MainWindow) -> Timer {
    let width = main_window.window().size().width;
    let height = main_window.window().size().height;

    let timer = Timer::default();
    let mut rng = rand::thread_rng();
    let frames = read_bat_frames();
    let handle = main_window.as_weak();
    let mut frame_number = 0;
    let mut current_pos = LogicalPosition { x: 0.0, y: 0.0 };

    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(100),
        move || {
            let frame = &frames.clone()[frame_number % 9];
            let bat = Bat {
                frame: frame.clone(),
                pos: current_pos,
                size: 200 as f32,
            };

            if frame_number % 200 == 0 {
                current_pos = LogicalPosition {
                    x: rng.gen_range(0.0..width as f32),
                    y: rng.gen_range(0.0..height as f32),
                };
            }

            frame_number += 1;

            handle.unwrap().set_bat(bat)
        },
    );

    timer
}

fn read_bat_frames() -> Vec<Image> {
    let mut frames = vec![];

    for i in 0..9 {
        let bat_frame_path = std::format!("seasons/halloween/bat-{}.png", i);
        let frame_data = match StaticAssets::get(&bat_frame_path) {
            Some(icon_data) => icon_data.data.into_owned(),
            None => StaticAssets::get("not-found.png")
                .unwrap()
                .data
                .into_owned(),
        };

        let bat_frame = image::load_from_memory_with_format(&frame_data, image::ImageFormat::Png)
            .unwrap()
            .into_rgba8();

        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            bat_frame.as_raw(),
            bat_frame.width(),
            bat_frame.height(),
        );

        frames.push(Image::from_rgba8(buffer))
    }
    frames
}

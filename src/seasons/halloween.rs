use crate::ui::*;
use rand::Rng;
use slint::{LogicalPosition, Timer, TimerMode};

pub fn setup_halloween_spooky_face(main_window: &MainWindow) -> Timer {
    let halloween_timer = Timer::default();
    let halloween_handle = main_window.as_weak();

    let width = main_window.window().size().width;
    let height = main_window.window().size().height;
    let mut rng = rand::thread_rng();

    halloween_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(13),
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
    let handle = main_window.as_weak();

    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(20),
        move || {
            let new_pos = LogicalPosition {
                x: rng.gen_range(0.0..width as f32),
                y: rng.gen_range(0.0..height as f32),
            };

            let bat = Bat { pos: new_pos };

            handle.unwrap().set_bat(bat)
        },
    );

    timer
}

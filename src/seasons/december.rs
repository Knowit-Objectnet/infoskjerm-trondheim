use crate::ui::*;
use rand::Rng;
use slint::{Timer, TimerMode, VecModel};
use std::rc::Rc;
use log::info;

struct SnowflakeModel {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32,
    size: f32,
}

impl From<&mut SnowflakeModel> for Snowflake {
    fn from(val: &mut SnowflakeModel) -> Snowflake {
        Snowflake {
            x: val.x.into(),
            y: val.y.into(),
            size: val.size.into(),
        }
    }
}

pub fn setup_snow(main_window: &MainWindow) -> Timer {
    let snow_timer = Timer::default();
    let mut rng = rand::thread_rng();

    let snow_handle = main_window.as_weak();
    let width = 1080;
    let height = 1920;

    let mut flurry = vec![];
    for _ in 0..100 {
        // Make 1/3 small, 1/3 medium and 1/3 large snowflakes, to achieve some sort of parallax.
        let (size, y_velocity) = match rng.gen_range(0..3) {
            0 => (2.0, rng.gen_range(0.1..0.5)),
            1 => (5.0, rng.gen_range(0.5..1.0)),
            _ => (8.0, rng.gen_range(1.0..2.0)),
        };

        let snowflake = SnowflakeModel {
            x: rng.gen_range(0.0..width as f32),
            y: rng.gen_range(0.0..height as f32),
            x_velocity: rng.gen_range(-1.0..1.0) / 2.0,
            y_velocity: y_velocity / 2.0,
            size,
        };
        flurry.push(snowflake);
    }

    snow_timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(16),
        move || {
            let start_time = std::time::Instant::now();
            let mut snowflakes: Vec<Snowflake> = Vec::with_capacity(flurry.len());
            for flake in &mut flurry {
                flake.x += flake.x_velocity;
                flake.y += flake.y_velocity;

                // bounce snowflake if out of bounds
                if flake.x < -15.0 {
                    flake.x = -15.0;
                    flake.x_velocity = -flake.x_velocity;
                } else if flake.x > width as f32 {
                    flake.x = width as f32;
                    flake.x_velocity = -flake.x_velocity;
                }

                // move flake to top when exiting bottom
                if flake.y > height as f32 {
                    flake.y = 0.0;
                }
                snowflakes.push(flake.into());
            }
            snow_handle
                .unwrap()
                .set_snowflakes(Rc::new(VecModel::from(snowflakes)).into());
            let duration = start_time.elapsed();
            info!("Snowflake update took: {:?}", duration);
        },
    );

    snow_timer
}

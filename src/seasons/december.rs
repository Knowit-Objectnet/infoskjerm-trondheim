use std::rc::Rc;
use rand::Rng;
use slint::{Timer, TimerMode, VecModel};
use crate::ui::*;

struct SnowflakeModel {
    x: f32,
    y: f32,
    x_velocity: f32,
    y_velocity: f32
}

impl From<&mut SnowflakeModel> for Snowflake {
    fn from(val: &mut SnowflakeModel) -> Snowflake {
        Snowflake{
            x: val.x.into(),
            y: val.y.into()
        }
    }
}


pub fn setup_snow(main_window: &MainWindow) -> Timer {

    let snow_timer = Timer::default();
    let mut rng = rand::thread_rng();

    let snow_handle = main_window.as_weak();
    let width = main_window.window().size().width;
    let height = main_window.window().size().height;
   

    let mut flurry = vec!();
    for _ in 0..100 {
        let snowflake = SnowflakeModel {
            x: rng.gen_range(0.0..width as f32),
            y: rng.gen_range(0.0..height as f32),
            x_velocity: rng.gen_range(-1.0..1.0),
            y_velocity: rng.gen_range(0.5..2.0)
        };
        flurry.push(snowflake);
    }

    snow_timer.start(TimerMode::Repeated, std::time::Duration::from_millis(10), move || {
        let snowflakes : VecModel<Snowflake> = VecModel::default();
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
        snow_handle.unwrap().set_snowflakes(Rc::new(snowflakes).into());
    });

    snow_timer
}
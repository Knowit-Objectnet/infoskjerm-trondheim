use log::info;
use rust_embed::RustEmbed;
use slint::PlatformError;
use ui::*;

mod calendar;
mod datetime;
mod food;
mod forecast;
mod selfie;
mod xkcd;

pub mod ui {
    slint::include_modules!();
}

// we embed img folder into the compiled binary for simpler distribution
#[derive(RustEmbed)]
#[folder = "img/"]
struct StaticAssets;

fn main() -> Result<(), PlatformError> {
    env_logger::init();
    info!("Starting up...");

    let main_window = MainWindow::new().unwrap();
    forecast::setup(&main_window);
    xkcd::setup(&main_window);
    food::setup(&main_window);
    calendar::setup(&main_window);

    //we need to store the timers in variables to prevent them from being dropped
    let _t = datetime::setup(&main_window);
    #[cfg(feature = "selfie")] //grab selfie of app
    let _s = selfie::grab_selfie(&main_window);

    main_window.run()
}

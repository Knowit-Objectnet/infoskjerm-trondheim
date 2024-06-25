extern crate chrono;

use log::info;
use rust_embed::RustEmbed;
use slint::PlatformError;
use ui::*;

mod food;
mod forecast;
mod xkcd;
mod calendar;
mod datetime;

pub mod ui {
    slint::include_modules!();
}

// we embed img folder into the compiled binary for simpler distribution
#[derive(RustEmbed)]
#[folder = "img/"]
struct StaticAssets;


#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    slint::android::init(app).unwrap();
    env_logger::init();
    info!("Starting up...");

    let main_window = MainWindow::new().unwrap();

    //forecast::setup(&main_window);
    xkcd::setup(&main_window);
    //food::setup(&main_window);
    //calendar::setup(&main_window);
    //we need to store the timer in a variable to prevent it from being dropped
    let _t = datetime::setup(&main_window);

    main_window.run();
}
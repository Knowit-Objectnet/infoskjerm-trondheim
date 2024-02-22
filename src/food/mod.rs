mod server;
mod wolt_models;
mod worker;
use crate::ui::*;

use std::{sync::mpsc, thread};
use tokio::runtime::Runtime;

pub fn setup(main_window: &MainWindow) {
    let window_weak = main_window.as_weak();
    let (tx, rx) = mpsc::channel();
    //spawn server worker thread
    thread::spawn(move || {
        Runtime::new()
            .unwrap()
            .block_on(server::food_endpoint_server(tx))
    });
    //thread for tracking food
    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(worker::food_worker_loop(window_weak, rx))
    });
}

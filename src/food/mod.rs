mod server;
mod wolt_models;
use self::wolt_models::WoltTracking;
use crate::ui::*;
use chrono::Local;
use log::{error, info};
use reqwest::Url;
use slint::Weak;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
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
            .block_on(food_worker_loop(window_weak, rx))
    });
}

async fn food_worker_loop(window: Weak<MainWindow>, rx: Receiver<Url>) {
    let mut current_url: Option<Url> = None;
    let mut current_tracking = FoodTracking::default();

    loop {
        match rx.try_recv() {
            Ok(tracking_url) => {
                //TODO: Use logging instead of println
                info!("Got new tracking url: {}", tracking_url);
                current_url = Some(tracking_url);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                error!("Food tracking channel disconnected.");
            }
        }

        if let Some(url) = &current_url {
            let tracking_response = get_tracking_data(&url).await;

            current_tracking = match tracking_response {
                Ok(tracking_response) => {
                    let remaining_time =
                        ((tracking_response.delivery_eta.unwrap()) - Local::now()).num_minutes();
                    FoodTracking {
                        resturant_name: tracking_response.from_location.name.en.into(),
                        minutes_remaining: remaining_time.to_string().into(),
                        active: tracking_response.status != "delivered",
                    }
                }
                Err(e) => {
                    error!("Error getting tracking data: {}", e);
                    // Reset URL as to not spam the API
                    current_url = None;
                    // Default is hidden in UI
                    FoodTracking::default()
                }
            }
        }

        display_tracking(&window, current_tracking.clone());
        //TODO: Adjust timeout, maybe dynamic based on refresh_in_seconds
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn get_tracking_data(url: &Url) -> Result<WoltTracking, reqwest::Error> {
    let response = reqwest::get(url.clone()).await?;
    let wolt_tracking_data = response.json::<WoltTracking>().await?;
    Ok(wolt_tracking_data)
}

fn display_tracking(window_weak: &Weak<MainWindow>, food_tracking: FoodTracking) {
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| window.set_foodTracking(food_tracking))
        .unwrap();
}

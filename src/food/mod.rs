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
    loop {
        match rx.try_recv() {
            Ok(tracking_url) => {
                //TODO: Use logging instead of println
                info!("Got new tracking url: {}", tracking_url);
                current_url = Some(tracking_url);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                error!("Food tracking channel disconnected. Panicing...");
                panic!();
            }
        }

        let food_tracking = track_food(&current_url).await;
        // If food_tracking is None, we either have no url or the tracking is done.
        // Default FoodTracking is empty and hidden in the UI.
        let food_tracking = food_tracking.unwrap_or_else(|| FoodTracking::default());
        display_tracking(&window, food_tracking);
        //TODO: Adjust timeout, maybe dynamic
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn track_food(url: &Option<Url>) -> Option<FoodTracking> {
    let tracking_url = match url {
        Some(url) => url,
        // No URL, nothing to track.
        None => return None,
    };

    let food_tracking = get_tracking_data(&tracking_url).await;

    match food_tracking {
        Ok(tracking_data) => {
            //TODO: maybe show status before tracking ETA is present?
            if tracking_data.status == "delivered" || tracking_data.delivery_eta.is_none() {
                return None;
            }
            let remaining_time =
                ((tracking_data.delivery_eta.unwrap()) - Local::now()).num_minutes();
            Some(FoodTracking {
                active: true,
                minutes_remaining: remaining_time.to_string().into(),
                resturant_name: tracking_data.from_location.name.en.into(),
            })
        }

        //TODO: Log error
        Err(_e) => None,
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

use super::wolt_models::WoltTracking;
use crate::ui::*;
use chrono::Local;
use log::{error, info};
use reqwest::Url;
use slint::{SharedString, Weak};
use std::sync::mpsc::{Receiver, TryRecvError};

pub async fn food_worker_loop(window: Weak<MainWindow>, rx: Receiver<Url>) {
    let mut current_url = None;
    let mut current_tracking = FoodTracking::default();

    loop {
        match rx.try_recv() {
            Ok(tracking_url) => {
                info!("Got new tracking url: {}", tracking_url);
                current_url = Some(tracking_url);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => {
                error!("Food tracking channel disconnected.");
            }
        }

        if let Some(url) = &current_url {
            info!("Fetching tracking data for url: {}", url);
            let tracking_response = get_tracking_data(&url).await;
            let tracking_status = get_tracking_status(tracking_response);

            if !tracking_status.active {
                info!("Tracking status is not active. Resetting current url.");
                current_url = None;
            }

            current_tracking = tracking_status;
        }

        info!("Displaying tracking information: {:?}", current_tracking);
        display_tracking(&window, current_tracking.clone());
        info!("Sleeping for 5 seconds");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn get_tracking_data(url: &Url) -> Result<WoltTracking, reqwest::Error> {
    info!("Sending GET request to: {}", url);
    let response = reqwest::get(url.clone()).await?;
    let wolt_tracking_data = response.json::<WoltTracking>().await?;
    info!("Received tracking data: {:?}", wolt_tracking_data);
    Ok(wolt_tracking_data)
}

fn get_tracking_status(tracking_data: Result<WoltTracking, reqwest::Error>) -> FoodTracking {
    match tracking_data {
        Ok(tracking_data) => {
            info!("Processing tracking data: {:?}", tracking_data);
            let minutes_remaining: SharedString = match tracking_data.delivery_eta {
                Some(eta) => ((eta) - Local::now()).num_minutes().to_string().into(),
                None => "ukjent antall".into(),
            };

            let active = tracking_data.status != "delivered";

            FoodTracking {
                resturant_name: tracking_data.from_location.name.en.into(),
                minutes_remaining,
                active,
            }
        }
        Err(e) => {
            error!("Error getting tracking data: {}", e);
            FoodTracking::default()
        }
    }
}

fn display_tracking(window_weak: &Weak<MainWindow>, food_tracking: FoodTracking) {
    info!("Updating UI with tracking information: {:?}", food_tracking);
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| window.set_foodTracking(food_tracking))
        .unwrap();
}

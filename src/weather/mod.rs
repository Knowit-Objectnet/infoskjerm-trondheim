use std::rc::Rc;
use std::thread;

use super::StaticAssets;
use reqwest::header;
use slint::Weak;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, VecModel};

use log::{error, info};

use chrono::{DateTime, Local, Utc};

use crate::ui::*;
mod models;

pub fn setup(window: &MainWindow) {
    let window_weak = window.as_weak();
    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(weather_worker_loop(window_weak))
    });
}

async fn weather_worker_loop(window: Weak<MainWindow>) {
    loop {
        let forecast_vector = get_forecast().await;
        display_forecast(&window, forecast_vector);
        tokio::time::sleep(std::time::Duration::from_secs(60 * 15)).await;
    }
}

fn display_forecast(window_weak: &Weak<MainWindow>, forecasts: Vec<models::ForecastModel>) {
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {
            let vm: VecModel<Forecast> = VecModel::default();

            for f in forecasts {
                let icon = get_icon(f.icon_name);
                let forecast = Forecast {
                    time: f.time.to_owned().into(),
                    temp: f.temp.to_owned().into(),
                    icon,
                    precipitation: f.precipitation.to_owned().into(),
                };
                vm.push(forecast);
            }

            window.set_forecasts(Rc::new(vm).into());
        })
        .unwrap();
}

async fn get_forecast() -> Vec<models::ForecastModel> {
    info! {"Fetching weather data... "}

    let api_url =
        "https://api.met.no/weatherapi/locationforecast/2.0/compact.json?lat=63.2549&lon=10.2342";

    let client = reqwest::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Knowit Infoskjerm - https://github.com/Knowit-Objectnet/infoskjerm-trondheim",
        ),
    );

    let response = client.get(api_url).headers(headers).send().await;

    let forecast_data: models::ForecastRaw = match response {
        Ok(res) => res.json().await.unwrap_or_default(),
        Err(err) => {
            error!("Failed to fetch weather data: {}", err);
            models::ForecastRaw::default()
        }
    };

    let next_hours_of_forecasts = forecast_data.properties.timeseries[0..7].to_vec();
    let mut forecast_vector = Vec::default();

    for f in next_hours_of_forecasts {
        let next_hour = f.data.next_1_hours.unwrap_or_default();

        let icon_name = next_hour.summary.symbol_code;

        let datetime = DateTime::parse_from_rfc3339(f.time.as_str())
            .unwrap()
            .with_timezone(&Utc);
        let local_datetime = datetime.with_timezone(&Local);
        let time = local_datetime.format("%H:%M").to_string().into();

        let temp = std::format!("{:.1}", f.data.instant.details.air_temperature).into();

        let precipitation = std::format!("{:.1}", next_hour.details.precipitation_amount).into();

        forecast_vector.push(models::ForecastModel {
            time,
            temp,
            icon_name,
            precipitation,
        });
    }
    forecast_vector
}

fn get_icon(icon_name: String) -> Image {
    let icon_path = std::format!("weather/{}.png", icon_name);
    let icon_data = match StaticAssets::get(&icon_path) {
        Some(icon_data) => icon_data.data.into_owned(),
        None => StaticAssets::get("not-found.png")
            .unwrap()
            .data
            .into_owned(),
    };

    let weather_icon = image::load_from_memory_with_format(&icon_data, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        weather_icon.as_raw(),
        weather_icon.width(),
        weather_icon.height(),
    );

    Image::from_rgba8(buffer)
}

use std::rc::Rc;
use std::thread;

use super::StaticAssets;
use reqwest::header;
use serde::{Deserialize, Serialize};
use slint::Weak;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, VecModel};

use log::{error, info};

use chrono::{DateTime, Local, Utc};

use crate::ui::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForecastRaw {
    properties: Properties,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Properties {
    pub timeseries: Vec<Series>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Series {
    time: String,
    data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    instant: Instant,
    #[serde(rename = "next_1_hours")]
    next_1_hours: Option<Next1Hours>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Instant {
    details: InstantDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstantDetails {
    #[serde(rename = "air_pressure_at_sea_level")]
    air_pressure_at_sea_level: f32,
    #[serde(rename = "air_temperature")]
    air_temperature: f32,
    #[serde(rename = "cloud_area_fraction")]
    cloud_area_fraction: f32,
    #[serde(rename = "relative_humidity")]
    relative_humidity: f32,
    #[serde(rename = "wind_from_direction")]
    wind_from_direction: f32,
    #[serde(rename = "wind_speed")]
    wind_speed: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Next1Hours {
    pub summary: Summary,
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    #[serde(rename = "symbol_code")]
    pub symbol_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    #[serde(rename = "precipitation_amount")]
    pub precipitation_amount: f32,
}

pub fn setup(window: &FooMainWindow) -> thread::JoinHandle<()> {
    let window_weak = window.as_weak();

    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(weather_worker_loop(window_weak))
    })
}

async fn get_forecast() -> VecModel<Forecast> {
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

    let forecast_data: ForecastRaw = match response {
        Ok(res) => res.json().await.unwrap_or_default(),
        Err(err) => {
            error!("Failed to fetch weather data: {}", err);
            ForecastRaw::default()
        }
    };

    let next_hours_of_forecasts = forecast_data.properties.timeseries[0..7].to_vec();
    let forecast_vector = VecModel::default();

    for f in next_hours_of_forecasts {
        let next_hour = f.data.next_1_hours.unwrap_or_default();

        let icon_name = next_hour.summary.symbol_code;
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

        let icon = Image::from_rgba8(buffer);

        let datetime = DateTime::parse_from_rfc3339(f.time.as_str())
            .unwrap()
            .with_timezone(&Utc);
        let local_datetime = datetime.with_timezone(&Local);
        let time = local_datetime.format("%H:%M").to_string().into();

        let temp = std::format!("{:.1}", f.data.instant.details.air_temperature).into();

        let precipitation = std::format!("{:.1}", next_hour.details.precipitation_amount).into();

        forecast_vector.push(Forecast {
            time,
            temp,
            icon,
            precipitation,
        });
    }
    forecast_vector
}

async fn weather_worker_loop(window: Weak<FooMainWindow>) {
    let forecast_vector = get_forecast().await;
    display_forecast(window.clone(), forecast_vector);
}

fn display_forecast(window_weak: Weak<FooMainWindow>, forecasts: VecModel<Forecast>) {
    window_weak
        .upgrade_in_event_loop(move |window: FooMainWindow| {
            window
                .global::<WeatherAdapter>()
                .set_forecasts(Rc::new(forecasts).into());
        })
        .unwrap();
}

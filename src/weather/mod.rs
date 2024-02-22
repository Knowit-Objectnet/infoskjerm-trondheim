use std::rc::Rc;
use std::thread;

use self::weather_models::{ForecastModel, ForecastRaw, Series};

use super::StaticAssets;
use reqwest::header;
use slint::Weak;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, VecModel};

use log::{error, info};

use crate::ui::*;
mod weather_models;

const API_URL: &str =
    "https://api.met.no/weatherapi/locationforecast/2.0/compact.json?lat=63.2549&lon=10.2342";
const USER_AGENT_STR: &str = "Knowit Infoskjerm - github.com/Knowit-Objectnet/infoskjerm-trondheim";

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
        let forecast_vectors = get_forecasts().await;
        display_forecast(&window, forecast_vectors);
        tokio::time::sleep(std::time::Duration::from_secs(60 * 15)).await;
    }
}

fn display_forecast(
    window_weak: &Weak<MainWindow>,
    forecasts: (
        Vec<weather_models::ForecastModel>,
        Vec<weather_models::ForecastModel>,
    ),
) {
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {
            let today_forecast: VecModel<Forecast> = VecModel::default();
            let tomorrow_forecast: VecModel<Forecast> = VecModel::default();

            for f in forecasts.0 {
                let icon = get_icon(f.icon_name);
                let forecast = Forecast {
                    time: f.time.to_owned().into(),
                    temp: f.temp.to_owned().into(),
                    icon,
                    precipitation: f.precipitation.to_owned().into(),
                };
                today_forecast.push(forecast);
            }

            for f in forecasts.1 {
                let icon = get_icon(f.icon_name);
                let forecast = Forecast {
                    time: f.time.to_owned().into(),
                    temp: f.temp.to_owned().into(),
                    icon,
                    precipitation: f.precipitation.to_owned().into(),
                };
                tomorrow_forecast.push(forecast);
            }

            window.set_todayForecast(Rc::new(today_forecast).into());
            window.set_tomorrowForecast(Rc::new(tomorrow_forecast).into());
        })
        .unwrap();
}

async fn get_forecasts() -> (
    Vec<weather_models::ForecastModel>,
    Vec<weather_models::ForecastModel>,
) {
    info! {"Fetching weather data... "}

    let client = reqwest::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(USER_AGENT_STR),
    );

    let response = client.get(API_URL).headers(headers).send().await;

    let forecast_data: weather_models::ForecastRaw = match response {
        Ok(res) => res.json().await.unwrap_or_default(),
        Err(err) => {
            error!("Failed to fetch weather data: {}", err);
            weather_models::ForecastRaw::default()
        }
    };

    let next_hours_of_forecasts = get_next_forecasts(&forecast_data);
    let tomorrow_forecasts = get_tomorrows_forecasts(&forecast_data);

    (next_hours_of_forecasts, tomorrow_forecasts)
}

fn get_next_forecasts(forecast_data: &ForecastRaw) -> Vec<ForecastModel> {
    let next_hours_of_forecasts = forecast_data.properties.timeseries[0..7].to_vec();
    map_to_forecast_model(next_hours_of_forecasts)
}

fn get_tomorrows_forecasts(forecast_data: &ForecastRaw) -> Vec<ForecastModel> {
    let desired_times = vec![
        "06:00", "07:00", "08:00", "09:00", "15:00", "16:00", "17:00",
    ];
    let tomorrows_forecast_times = forecast_data
        .properties
        .timeseries
        .iter()
        .filter(|f| {
            let timestring = f.time.format("%H:%M").to_string();
            let timestr = timestring.as_str();
            let is_tomorrow = f.time.date_naive()
                == chrono::Local::now().date_naive() + chrono::Duration::days(1);
            desired_times.contains(&timestr) && is_tomorrow
        })
        .cloned()
        .collect::<Vec<_>>();

    map_to_forecast_model(tomorrows_forecast_times)
}

fn map_to_forecast_model(forecast_series: Vec<Series>) -> Vec<ForecastModel> {
    let mut forecast_vector = Vec::default();

    for f in forecast_series {
        let time = f.time.format("%H:%M").to_string();

        let temp = std::format!("{:.1}", f.data.instant.details.air_temperature);

        let next_hour = f.data.next_1_hours.unwrap_or_default();
        let icon_name = next_hour.summary.symbol_code;
        let precipitation = std::format!("{:.1}", next_hour.details.precipitation_amount);

        forecast_vector.push(weather_models::ForecastModel {
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

use log::{error, info};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, Weak};
use std::thread;

use self::forecast_models::{ForecastModel, ForecastRaw};

use crate::{ui::*, StaticAssets};

mod forecast_models;

const API_URL: &str =
    "https://api.met.no/weatherapi/locationforecast/2.0/complete.json?lat=63.2549&lon=10.2342";
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
        // TODO: Feilhåndter bedre enn unwrap !
        let data = get_forecast_data().await.unwrap();
        let today = get_forecast_today(&data);
        let tomorrow = get_forecast_tomorrow(&data);
        display_forecast(&window, today, tomorrow);
        tokio::time::sleep(std::time::Duration::from_secs(60 * 15)).await;
    }
}

fn display_forecast(window: &Weak<MainWindow>, today: ForecastModel, tomorrow: ForecastModel) {
    let _ = window.upgrade_in_event_loop(|window: MainWindow| {
        window.set_todayForecast(today.into());
        window.set_tomorrowForecast(tomorrow.into());
    });
}

impl From<ForecastModel> for Forecast {
    fn from(val: ForecastModel) -> Self {
        Forecast {
            icon: get_icon(val.icon_name),
            precipitation: val.precip.into(),
            temp: val.temp.into()
        }
    }
}

async fn get_forecast_data() -> Option<ForecastRaw> {
    info! { "Fetching weather data" }

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static(USER_AGENT_STR),
    );

    let response = client.get(API_URL).headers(headers).send().await;

    let forecast_json = match response {
        Ok(res) => res.json().await,
        Err(err) => {
            error!("Failed to fetch weather data: {}", err);
            return None;
        }
    };

    match forecast_json {
        Ok(res) => Some(res),
        Err(err) => {
            error!("Failed to deserialize json: {}", err);
            None
        }
    }
}

fn get_forecast_today(data: &ForecastRaw) -> ForecastModel {
    let first_forecast = &data.properties.timeseries[0];

    let next_6_hours = first_forecast
        .data
        .next_6_hours
        .as_ref()
        .expect("next_6_hours should be in forecast");

    let temp =
        (next_6_hours.details.air_temperature_max + next_6_hours.details.air_temperature_min) / 2.0;

    ForecastModel {
        icon_name: next_6_hours.summary.symbol_code.to_owned(),
        temp: std::format!("{:.0}", temp),
        precip: std::format!("{:.0}", next_6_hours.details.precipitation_amount),
    }
}

fn get_forecast_tomorrow(data: &ForecastRaw) -> ForecastModel {
    let tomorrow = (chrono::Local::now().date_naive() + chrono::Duration::try_days(1).unwrap())
        .and_hms_opt(8, 0, 0)
        .unwrap();

    let predicate = |t: &str| {
        if let Ok(time) = chrono::DateTime::parse_from_rfc3339(t) {
            return time.naive_utc() == tomorrow;
        }
        false
    };

    let tomorrow_forecast = data
        .properties
        .timeseries
        .iter()
        .find(|s| predicate(&s.time))
        .expect("Tomorrows forcast should be in timeseries");

    let next_6_hours = tomorrow_forecast
        .data
        .next_6_hours
        .as_ref()
        .expect("next_6_hours should be in forecast");

    let temp =
        (next_6_hours.details.air_temperature_max + next_6_hours.details.air_temperature_min) / 2.0;

    ForecastModel {
        icon_name: next_6_hours.summary.symbol_code.to_owned(),
        temp: std::format!("{:.0}", temp),
        precip: std::format!("{:.0}", next_6_hours.details.precipitation_amount),
    }
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

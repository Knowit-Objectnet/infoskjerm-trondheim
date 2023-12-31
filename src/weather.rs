use std::error;

use super::Forecast;
use super::StaticAssets;
use log::info;
use reqwest::header;
use serde::{Deserialize, Serialize};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, VecModel};

use chrono::{DateTime, Local, Utc};

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
pub fn get_forecast() -> Result<VecModel<Forecast>, Box<dyn error::Error>> {
    let api_url =
        "https://api.met.no/weatherapi/locationforecast/2.0/compact.json?lat=63.2549&lon=10.2342";

    let client = reqwest::blocking::Client::new();
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Knowit Infoskjerm - https://github.com/Knowit-Objectnet/infoskjerm-trondheim",
        ),
    );

    let response = client.get(api_url).headers(headers).send()?;
    let forecast_data = response.json::<ForecastRaw>()?;
    let next_hours_of_forecasts = forecast_data.properties.timeseries[0..7].to_vec();
    let forecast_vector = VecModel::default();

    for f in next_hours_of_forecasts {
        let next_hour = f.data.next_1_hours.unwrap_or_default();

        let icon_name = next_hour.summary.symbol_code;
        let icon_path = format!("weather/{}.png", icon_name);
        let icon_data = match StaticAssets::get(&icon_path) {
            Some(icon_data) => icon_data.data.into_owned(),
            None => StaticAssets::get("not-found.png")
                .unwrap()
                .data
                .into_owned(),
        };

        let weather_icon =
            image::load_from_memory_with_format(&icon_data, image::ImageFormat::Png)?.into_rgba8();

        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            weather_icon.as_raw(),
            weather_icon.width(),
            weather_icon.height(),
        );

        let icon = Image::from_rgba8(buffer);

        let datetime = DateTime::parse_from_rfc3339(f.time.as_str())?.with_timezone(&Utc);
        let local_datetime = datetime.with_timezone(&Local);
        let time = local_datetime.format("%H:%M").to_string().into();

        let temp = format!("{:.1}", f.data.instant.details.air_temperature).into();

        let precipitation = format!("{:.1}", next_hour.details.precipitation_amount).into();

        forecast_vector.push(Forecast {
            time,
            temp,
            icon,
            precipitation,
        })
    }

    info!("Loaded forecast new forecast");
    Ok(forecast_vector)
}

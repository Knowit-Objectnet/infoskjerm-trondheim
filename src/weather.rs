use super::Forecast;
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
pub fn get_forecast() -> VecModel<Forecast> {
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

    let response = client.get(api_url).headers(headers).send().unwrap();
    let forecast_data = response.json::<ForecastRaw>().unwrap();
    let next_hours_of_forecasts = forecast_data.properties.timeseries[0..7].to_vec();

    let forecast_vector = VecModel::default();

    //TODO: error handling
    for f in next_hours_of_forecasts {
        let icon_raw = image::open(format!(
            "img/weather/{}.png",
            f.data.next_1_hours.clone().unwrap().summary.symbol_code
        ))
        .expect("Error loading weather icon")
        .into_rgba8();

        let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
            icon_raw.as_raw(),
            icon_raw.width(),
            icon_raw.height(),
        );
        let icon = Image::from_rgba8(buffer);

        let datetime = DateTime::parse_from_rfc3339(f.time.as_str())
            .unwrap()
            .with_timezone(&Utc);

        let local_datetime = datetime.with_timezone(&Local);
        let time = local_datetime.format("%H:%M").to_string().into();

        forecast_vector.push(Forecast {
            time,
            temp: format!("{:.1}", f.data.instant.details.air_temperature).into(),
            icon,
            precipitation: format!(
                "{:.1}",
                f.data.next_1_hours.unwrap().details.precipitation_amount
            )
            .into(),
        })
    }

    forecast_vector
}

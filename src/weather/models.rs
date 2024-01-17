use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ForecastModel {
    pub time: String,
    pub temp: String,
    pub icon_name: String,
    pub precipitation: String,
}

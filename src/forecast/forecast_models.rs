use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForecastRaw {
    pub properties: Properties,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Series>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Units {
    pub air_pressure_at_sea_level: String,
    pub air_temperature: String,
    pub air_temperature_max: String,
    pub air_temperature_min: String,
    pub air_temperature_percentile_10: String,
    pub air_temperature_percentile_90: String,
    pub cloud_area_fraction: String,
    pub cloud_area_fraction_high: String,
    pub cloud_area_fraction_low: String,
    pub cloud_area_fraction_medium: String,
    pub dew_point_temperature: String,
    pub fog_area_fraction: String,
    pub precipitation_amount: String,
    pub precipitation_amount_max: String,
    pub precipitation_amount_min: String,
    pub probability_of_precipitation: String,
    pub probability_of_thunder: String,
    pub relative_humidity: String,
    pub ultraviolet_index_clear_sky: String,
    pub wind_from_direction: String,
    pub wind_speed: String,
    pub wind_speed_of_gust: String,
    pub wind_speed_percentile_10: String,
    pub wind_speed_percentile_90: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub time: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub instant: Instant,
    pub next_1_hours: Option<Next1Hours>,
    pub next_6_hours: Option<Next6Hours>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Instant {
    pub details: InstantDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantDetails {
    pub air_pressure_at_sea_level: f64,
    pub air_temperature: f64,
    pub air_temperature_percentile_10: f64,
    pub air_temperature_percentile_90: f64,
    pub cloud_area_fraction: f64,
    pub cloud_area_fraction_high: f64,
    pub cloud_area_fraction_low: f64,
    pub cloud_area_fraction_medium: f64,
    pub dew_point_temperature: f64,
    pub fog_area_fraction: Option<f64>,
    pub relative_humidity: f64,
    pub ultraviolet_index_clear_sky: Option<f64>,
    pub wind_from_direction: f64,
    pub wind_speed: f64,
    pub wind_speed_of_gust: Option<f64>,
    pub wind_speed_percentile_10: f64,
    pub wind_speed_percentile_90: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Next1Hours {
    pub summary: Next1HoursSummary,
    pub details: Next1HoursDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Next1HoursSummary {
    pub symbol_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Next1HoursDetails {
    pub precipitation_amount: f64,
    pub precipitation_amount_max: f64,
    pub precipitation_amount_min: f64,
    pub probability_of_precipitation: f64,
    pub probability_of_thunder: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Next6Hours {
    pub summary: Next6HoursSummary,
    pub details: Next6HoursDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Next6HoursSummary {
    pub symbol_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Next6HoursDetails {
    pub air_temperature_max: f64,
    pub air_temperature_min: f64,
    pub precipitation_amount: f64,
    pub precipitation_amount_max: f64,
    pub precipitation_amount_min: f64,
    pub probability_of_precipitation: f64,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ForecastModel {
    pub icon_name: String,
    pub temp: String,
    pub precip: String,
}

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WoltTracking {
    pub status: String,
    #[serde(rename = "from_location")]
    pub from_location: FromLocation,
    #[serde(rename = "delivery_eta")]
    pub delivery_eta: Option<DateTime<Local>>,
    #[serde(rename = "refresh_in_seconds")]
    pub refresh_in_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FromLocation {
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub en: String,
}

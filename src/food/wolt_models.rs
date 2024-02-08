use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub status: String,
    #[serde(rename = "preorder_status")]
    pub preorder_status: String,
    #[serde(rename = "from_location")]
    pub from_location: FromLocation,
    #[serde(rename = "to_location")]
    pub to_location: ToLocation,
    pub couriers: Vec<Courier>,
    #[serde(rename = "pre_estimate")]
    pub pre_estimate: String,
    #[serde(rename = "delivery_eta")]
    pub delivery_eta: String,
    #[serde(rename = "pickup_eta")]
    pub pickup_eta: String,
    #[serde(rename = "requested_dropoff_time")]
    pub requested_dropoff_time: String,
    #[serde(rename = "refresh_in_seconds")]
    pub refresh_in_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FromLocation {
    pub coordinates: Coordinates,
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub en: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToLocation {
    pub coordinates: Coordinates2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates2 {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Courier {
    pub id: String,
    pub coordinates: Coordinates3,
    #[serde(rename = "vehicle_type")]
    pub vehicle_type: String,
    #[serde(rename = "is_delivering")]
    pub is_delivering: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates3 {
    pub lat: f64,
    pub lon: f64,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedQuay {
    pub id: String,
    pub name: String,
    pub lines: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedStop {
    pub id: String,
    pub name: String,
    pub quays: Option<Vec<TrackedQuay>>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackedStops {
    pub stops: Vec<TrackedStop>
}
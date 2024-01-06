use serde::{Serialize, Deserialize};

use crate::models::DateTime;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct NextServiceResponse {
    pub metadata: Metadata,
    pub next_service: NextService,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Metadata {
    pub time_stamp: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct NextService {
    pub lines: Vec<Line>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all="PascalCase")]
pub struct Line {
    pub stop_code: String,
    pub line_code: String,
    pub line_name: String,
    pub service_type: String,
    pub departure_status: String,
    pub scheduled_platform: String,
    pub actual_platform: String,
    pub scheduled_departure_time: DateTime,
    pub computed_departure_time: DateTime,
    pub trip_number: String,
    pub update_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct ServiceAtAGlanceResponse {
    pub metadata: Metadata,
    pub trips: Trips,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Trips {
    pub trip: Vec<Trip>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all="PascalCase")]
pub struct Trip {
    pub trip_number: String,
    pub cars: String,
}
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct TimeRange {
    #[serde(rename = "_start")]
    pub start: DateTime<Local>,
    #[serde(rename = "_stop")]
    pub stop: DateTime<Local>,
}

#[derive(Deserialize)]
pub(super) struct Record {
    #[serde(rename = "_time")]
    pub time: DateTime<Local>,
    pub color: Option<String>,
}

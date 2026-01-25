use chrono::{NaiveDate, NaiveTime};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub start_date: NaiveDate,
    pub start_time: NaiveTime,
    pub venue: Venue,

    #[serde(rename = "@status")]
    pub status: EventStatus,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Ok,
    Cancelled,
    Postponed,
    Unpublished,
    Deleted,
}

#[derive(Debug, Serialize)]
pub struct Venue {
    pub name: String,
}

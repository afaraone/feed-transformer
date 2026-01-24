use chrono::{NaiveDate, NaiveTime};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct ParsedEvent {
    pub id: String,
    pub title: String,
    pub start_date: NaiveDate,
    pub start_time: NaiveTime,
    pub venue: Venue,

    #[serde(rename = "@status")]
    pub status: EventStatus,
}

#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    #[default]
    Ok,
    Cancelled,
    Postponed,
    Unpublished,
    Deleted,
}

#[derive(Debug, Serialize, Default)]
pub struct Venue {
    pub name: String,
}

impl ParsedEvent {
    pub fn new(
        id: String,
        title: String,
        start_date: NaiveDate,
        start_time: NaiveTime,
        status: EventStatus,
        venue: Venue,
    ) -> Self {
        Self {
            id,
            title,
            start_date,
            start_time,
            status,
            venue,
        }
    }
}

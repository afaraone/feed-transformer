use chrono::{NaiveDate, NaiveTime};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SkEvent {
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

impl SkEvent {
    pub fn new(
        id: String,
        title: String,
        start_date: NaiveDate,
        start_time: NaiveTime,
        status: EventStatus,
        venue_name: String,
    ) -> Self {
        SkEvent {
            id,
            title,
            start_date,
            start_time,
            status,
            venue: Venue { name: venue_name },
        }
    }
}

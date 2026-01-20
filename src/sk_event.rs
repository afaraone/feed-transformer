use chrono::{NaiveDate, NaiveTime};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SkEvent {
    id: String,
    title: String,
    start_date: NaiveDate,
    start_time: NaiveTime,
    venue: Venue,
}

#[derive(Debug, Serialize)]
struct Venue {
    name: String,
}

impl SkEvent {
    pub fn new(
        id: String,
        title: String,
        start_date: NaiveDate,
        start_time: NaiveTime,
        venue_name: String,
    ) -> Self {
        SkEvent {
            id,
            title,
            start_date,
            start_time,
            venue: Venue { name: venue_name },
        }
    }
}

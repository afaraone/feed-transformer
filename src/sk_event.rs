use chrono::{NaiveDate, NaiveTime};

#[allow(dead_code)]
#[derive(Debug)]
pub struct SkEvent {
    id: String,
    title: String,
    start_date: NaiveDate,
    start_time: NaiveTime,
    venue: Venue,
}

#[allow(dead_code)]
#[derive(Debug)]
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

use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TmEvent {
    pub event_id: Option<String>,
    pub primary_event_url: Option<String>,
    pub event_name: Option<String>,
    pub source: Option<String>,
    pub event_status: Option<String>,
    pub event_start_local_date: Option<NaiveDate>,
    pub event_start_local_time: Option<NaiveTime>,
    pub classification_segment_id: Option<String>,
    pub classification_genre: Option<String>,
    pub classification_sub_type: Option<String>,
    pub venue: Option<Venue>,
    pub attractions: Option<Vec<AttractionWrapper>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub venue_id: Option<String>,
    pub venue_name: Option<String>,
    pub venue_city: Option<String>,
    pub venue_state_code: Option<String>,
    pub venue_country_code: Option<String>,
    pub venue_zip_code: Option<String>,
    pub venue_timezone: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct AttractionWrapper {
    pub attraction: Attraction,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Attraction {
    pub attraction_id: Option<String>,
    pub attraction_name: Option<String>,
    pub classification_segment: Option<String>,
    pub classification_sub_type: Option<String>,
}

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TmEvent {
    pub event_id: Option<String>,
    #[serde(skip_serializing)]
    pub primary_event_url: Option<String>,
    pub event_name: Option<String>,
    #[serde(skip_serializing)]
    pub source: Option<String>,
    #[serde(skip_serializing)]
    pub event_status: Option<String>,
    #[serde(skip_serializing)]
    pub event_start_local_date: Option<NaiveDate>,
    #[serde(skip_serializing)]
    pub event_start_local_time: Option<NaiveTime>,
    #[serde(skip_serializing)]
    pub classification_segment_id: Option<String>,
    #[serde(skip_serializing)]
    pub classification_genre: Option<String>,
    #[serde(skip_serializing)]
    pub classification_sub_type: Option<String>,
    #[serde(skip_serializing)]
    pub venue: Option<Venue>,
    #[serde(skip_serializing)]
    pub attractions: Option<Vec<AttractionWrapper>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Default)]
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
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AttractionWrapper {
    pub attraction: Attraction,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Attraction {
    pub attraction_id: Option<String>,
    pub attraction_name: Option<String>,
    pub classification_segment: Option<String>,
    pub classification_sub_type: Option<String>,
}

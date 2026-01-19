use crate::sk_event::SkEvent;
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TmEvent {
    event_id: Option<String>,
    primary_event_url: Option<String>,
    event_name: Option<String>,
    source: Option<String>,
    event_status: Option<String>,
    event_start_local_date: Option<NaiveDate>,
    event_start_local_time: Option<NaiveTime>,
    classification_segment_id: Option<String>,
    classification_genre: Option<String>,
    classification_sub_type: Option<String>,
    venue: Option<Venue>,
    attractions: Option<Vec<AttractionWrapper>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Venue {
    venue_id: Option<String>,
    venue_name: Option<String>,
    venue_city: Option<String>,
    venue_state_code: Option<String>,
    venue_country_code: Option<String>,
    venue_zip_code: Option<String>,
    venue_timezone: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct AttractionWrapper {
    attraction: Attraction,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Attraction {
    attraction_id: Option<String>,
    attraction_name: Option<String>,
    classification_segment: Option<String>,
    classification_sub_type: Option<String>,
}

const TM_MUSIC_CLASSIFICATION_ID: &str = "KZFzniwnSyZfZ7v7nJ";

#[allow(dead_code)]
impl TmEvent {
    fn is_music_event(&self) -> bool {
        if let Some(classification_segment_id) = &self.classification_segment_id {
            classification_segment_id == TM_MUSIC_CLASSIFICATION_ID
        } else {
            false
        }
    }

    pub fn try_into_sk_event(self) -> Result<SkEvent, String> {
        if !self.is_music_event() {
            return Err("Must be music event!".to_string());
        }

        let id = self.event_id.ok_or("Missing id!")?;
        let title = self.event_name.ok_or("Missing title!")?;
        let start_date = self.event_start_local_date.ok_or("Missing start_date!")?;
        let start_time = self.event_start_local_time.ok_or("Missing start_time!")?;
        let venue_name = self
            .venue
            .and_then(|v| v.venue_name)
            .ok_or("Missing venue name")?;

        Ok(SkEvent::new(id, title, start_date, start_time, venue_name))
    }
}

use crate::sk_event::{EventStatus, SkEvent};
use chrono::{NaiveDate, NaiveTime};
use serde::Deserialize;
use thiserror::Error;

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
#[derive(Debug, Deserialize, Default)]
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
#[derive(Debug, Deserialize, Default)]
struct AttractionWrapper {
    attraction: Attraction,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Attraction {
    attraction_id: Option<String>,
    attraction_name: Option<String>,
    classification_segment: Option<String>,
    classification_sub_type: Option<String>,
}

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("Invalid event: {0}")]
    InvalidEvent(String),

    #[error("Ignored event")]
    IgnoredEvent,
}

const TM_MUSIC_CLASSIFICATION_ID: &str = "KZFzniwnSyZfZ7v7nJ";

impl TmEvent {
    fn is_music_event(&self) -> bool {
        if let Some(classification_segment_id) = &self.classification_segment_id {
            classification_segment_id == TM_MUSIC_CLASSIFICATION_ID
        } else {
            false
        }
    }

    pub fn try_into_sk_event(self) -> Result<SkEvent, TransformError> {
        if !self.is_music_event() {
            return Err(TransformError::IgnoredEvent);
        }

        let id = self
            .event_id
            .ok_or_else(|| TransformError::InvalidEvent("Missing id".into()))?;

        let title = self
            .event_name
            .ok_or_else(|| TransformError::InvalidEvent("Missing title".into()))?;

        let start_date = self
            .event_start_local_date
            .ok_or_else(|| TransformError::InvalidEvent("Missing start_date".into()))?;

        let start_time = self
            .event_start_local_time
            .ok_or_else(|| TransformError::InvalidEvent("Missing start_time".into()))?;

        let venue_name = self
            .venue
            .and_then(|v| v.venue_name)
            .ok_or_else(|| TransformError::InvalidEvent("Missing venue_name".into()))?;

        let status = match self.event_status.as_deref() {
            Some("onsale") | Some("offsale") => Ok(EventStatus::Ok),
            Some("rescheduled") | Some("postponed") => Ok(EventStatus::Postponed),
            Some("cancelled") => Ok(EventStatus::Cancelled),
            _ => Err(TransformError::InvalidEvent("Missing venue_name".into())),
        }?;

        Ok(SkEvent::new(
            id, title, start_date, start_time, status, venue_name,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_music_event() -> TmEvent {
        TmEvent {
            event_id: Some("event_id".to_string()),
            event_start_local_date: Some(NaiveDate::default()),
            event_name: Some("event title".to_string()),
            event_start_local_time: Some(NaiveTime::default()),
            classification_segment_id: Some(TM_MUSIC_CLASSIFICATION_ID.to_string()),
            venue: Some(Venue {
                venue_name: Some("venue name".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn non_music_event() -> TmEvent {
        TmEvent {
            classification_segment_id: Some("foo".to_string()),
            ..Default::default()
        }
    }

    fn invalid_music_event_missing_event_name() -> TmEvent {
        TmEvent {
            event_id: Some("event_id".to_string()),
            event_start_local_date: Some(NaiveDate::default()),
            event_start_local_time: Some(NaiveTime::default()),
            classification_segment_id: Some(TM_MUSIC_CLASSIFICATION_ID.to_string()),
            venue: Some(Venue {
                venue_name: Some("venue name".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_try_into_sk_event_with_valid_music_event() {
        let tm_event = valid_music_event();
        let result = tm_event.try_into_sk_event();
        assert!(matches!(result, Ok(_)));
    }

    #[test]
    fn test_try_into_sk_event_with_non_music_event() {
        let tm_event = non_music_event();
        let result = tm_event.try_into_sk_event();
        assert!(matches!(result, Err(TransformError::IgnoredEvent)));
    }

    #[test]
    fn test_try_into_sk_event_with_invalid_music_event() {
        let tm_event = invalid_music_event_missing_event_name();
        let result = tm_event.try_into_sk_event();
        assert!(matches!(result, Err(TransformError::InvalidEvent(_))));
    }
}

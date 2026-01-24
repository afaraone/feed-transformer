use crate::sk::EventStatus;
use crate::tm::TmEvent;
use chrono::{NaiveDate, NaiveTime};

const TM_MUSIC_CLASSIFICATION_ID: &str = "KZFzniwnSyZfZ7v7nJ";

pub struct Mapper {
    event: TmEvent,
}

impl Mapper {
    pub fn new(event: TmEvent) -> Self {
        Self { event }
    }

    pub fn should_parse(&self) -> bool {
        if let Some(classification_segment_id) = &self.event.classification_segment_id {
            classification_segment_id == TM_MUSIC_CLASSIFICATION_ID
        } else {
            false
        }
    }

    pub fn source(self) -> TmEvent {
        self.event
    }

    pub fn id(&self) -> Option<String> {
        self.event.event_id.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.event.event_name.clone()
    }

    pub fn start_date(&self) -> Option<NaiveDate> {
        self.event.event_start_local_date
    }

    pub fn start_time(&self) -> Option<NaiveTime> {
        self.event.event_start_local_time
    }

    pub fn venue_name(&self) -> Option<String> {
        self.event.venue.as_ref().and_then(|v| v.venue_name.clone())
    }

    pub fn status(&self) -> Option<EventStatus> {
        match self.event.event_status.as_deref() {
            Some("onsale") | Some("offsale") => Some(EventStatus::Ok),
            Some("rescheduled") | Some("postponed") => Some(EventStatus::Postponed),
            Some("cancelled") => Some(EventStatus::Cancelled),
            _ => None,
        }
    }
}

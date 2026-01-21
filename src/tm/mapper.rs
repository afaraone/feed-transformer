use crate::{
    sk_event::{EventStatus, SkEvent},
    tm::event::Event,
};

const TM_MUSIC_CLASSIFICATION_ID: &str = "KZFzniwnSyZfZ7v7nJ";

pub enum MapResult {
    Valid(SkEvent),
    Invalid,
    Ignored,
}

fn is_music_event(event: &Event) -> bool {
    match &event.classification_segment_id {
        Some(classication_segment_id) => classication_segment_id == TM_MUSIC_CLASSIFICATION_ID,
        None => false,
    }
}

pub fn map(event: Event) -> MapResult {
    if !is_music_event(&event) {
        return MapResult::Ignored;
    }

    let id = event.event_id;
    let title = event.event_name;
    let start_date = event.event_start_local_date;
    let start_time = event.event_start_local_time;
    let venue_name = event.venue.and_then(|v| v.venue_name);
    let status = match event.event_status.as_deref() {
        Some("onsale") | Some("offsale") => Some(EventStatus::Ok),
        Some("rescheduled") | Some("postponed") => Some(EventStatus::Postponed),
        Some("cancelled") => Some(EventStatus::Cancelled),
        _ => None,
    };

    if let (
        Some(id),
        Some(title),
        Some(start_date),
        Some(start_time),
        Some(status),
        Some(venue_name),
    ) = (id, title, start_date, start_time, status, venue_name)
    {
        MapResult::Valid(SkEvent::new(
            id, title, start_date, start_time, status, venue_name,
        ))
    } else {
        MapResult::Invalid
    }
}

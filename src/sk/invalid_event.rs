use serde::Serialize;

use crate::tm::TmEvent;

#[derive(Serialize)]
pub struct InvalidEvent {
    source: TmEvent,
    errors: Vec<String>,
}

impl InvalidEvent {
    pub fn new(source: TmEvent, errors: Vec<String>) -> Self {
        Self { source, errors }
    }
}

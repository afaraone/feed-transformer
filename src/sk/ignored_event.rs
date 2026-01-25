use crate::tm::TmEvent;
use serde::Serialize;

#[derive(Serialize)]
pub struct IgnoredEvent {
    source: TmEvent,
}

impl IgnoredEvent {
    pub fn new(source: TmEvent) -> Self {
        Self { source }
    }
}

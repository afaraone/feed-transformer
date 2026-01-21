use std::fmt::Display;

#[derive(Default)]
pub struct RunnerMetrics {
    accepted_events: i32,
    invalid_events: i32,
    ignored_events: i32,
    unparsed_events: i32,
}

impl RunnerMetrics {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn increment_accepted(&mut self) {
        self.accepted_events += 1;
    }

    pub fn increment_invalid(&mut self) {
        self.invalid_events += 1;
    }

    pub fn increment_ignored(&mut self) {
        self.ignored_events += 1;
    }

    pub fn increment_unparsed(&mut self) {
        self.unparsed_events += 1;
    }
}

impl Display for RunnerMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "---------------------------------\nFeed completed\nAccepted Events: {}\nInvalid Events: {}\nIgnored Events: {}\n---------------------------------",
            self.accepted_events, self.invalid_events, self.ignored_events
        )
    }
}

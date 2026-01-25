use crate::sk::{Event, IgnoredEvent, InvalidEvent, Venue};
use crate::tm::Mapper;

pub enum BuildFailure {
    Invalid(InvalidEvent),
    Ignored(IgnoredEvent),
}

struct ValidationErrors {
    errors: Vec<String>,
}

impl ValidationErrors {
    fn new() -> Self {
        Self { errors: Vec::new() }
    }

    fn check_required<T>(&mut self, name: &str, opt: Option<T>) -> Option<T> {
        if opt.is_none() {
            self.errors.push(format!("Missing field: {}", name));
        }
        opt
    }

    fn result(self) -> Result<(), Vec<String>> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors)
        }
    }
}

pub fn build_event(mapper: Mapper) -> Result<Event, BuildFailure> {
    if !mapper.should_parse() {
        let source = mapper.source();
        let event = IgnoredEvent::new(source);
        return Err(BuildFailure::Ignored(event));
    }

    let mut validation_errors = ValidationErrors::new();

    let id = validation_errors.check_required("id", mapper.id());
    let title = validation_errors.check_required("title", mapper.title());
    let start_date = validation_errors.check_required("start_date", mapper.start_date());
    let start_time = validation_errors.check_required("start_time", mapper.start_time());
    let venue_name = validation_errors.check_required("venue_name", mapper.venue_name());
    let status = validation_errors.check_required("status", mapper.status());

    if let Err(validation_errors) = validation_errors.result() {
        let source = mapper.source();
        let event = InvalidEvent::new(source, validation_errors);
        return Err(BuildFailure::Invalid(event));
    }

    Ok(Event {
        id: id.unwrap(),
        title: title.unwrap(),
        start_date: start_date.unwrap(),
        start_time: start_time.unwrap(),
        status: status.unwrap(),
        venue: Venue {
            name: venue_name.unwrap(),
        },
    })
}

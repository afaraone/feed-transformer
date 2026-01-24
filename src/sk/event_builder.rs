use crate::sk::{ParsedEvent, Venue};
use crate::tm::{Mapper, TmEvent};

pub enum BuildFailure {
    Invalid {
        source: Box<TmEvent>,
        errors: Vec<String>,
    },
    Ignored {
        source: Box<TmEvent>,
    },
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

pub fn build_event(mapper: Mapper) -> Result<ParsedEvent, BuildFailure> {
    if !mapper.should_parse() {
        return Err(BuildFailure::Ignored {
            source: Box::new(mapper.source()),
        });
    }

    let mut validation_errors = ValidationErrors::new();

    let id = validation_errors.check_required("id", mapper.id());
    let title = validation_errors.check_required("title", mapper.title());
    let start_date = validation_errors.check_required("start_date", mapper.start_date());
    let start_time = validation_errors.check_required("start_time", mapper.start_time());
    let venue_name = validation_errors.check_required("venue_name", mapper.venue_name());
    let status = validation_errors.check_required("status", mapper.status());

    if let Err(validation_errors) = validation_errors.result() {
        return Err(BuildFailure::Invalid {
            source: Box::new(mapper.source()),
            errors: validation_errors,
        });
    }

    Ok(ParsedEvent {
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

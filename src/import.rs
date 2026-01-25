use crate::{
    runner_metrics::RunnerMetrics,
    sk::{BuildFailure, build_event},
    stream_iterator::StreamError,
    tm::{Mapper, TmEvent},
    writer::{WriteError, Writer},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("{0}")]
    WriteError(#[from] WriteError),
}

pub fn import(
    events: impl Iterator<Item = Result<TmEvent, StreamError>>,
    runner_metrics: &mut RunnerMetrics,
    mut inventory_writer: impl Writer,
) -> Result<(), ImportError> {
    inventory_writer.start()?;

    for tm_event in events {
        let Ok(tm_event) = tm_event else {
            runner_metrics.increment_unparsed();
            continue;
        };

        let mapper = Mapper::new(tm_event);

        match build_event(mapper) {
            Ok(songkick_event) => {
                inventory_writer.add(&songkick_event)?;
                runner_metrics.increment_accepted();
            }
            Err(BuildFailure::Invalid(_invalid_event)) => runner_metrics.increment_invalid(),
            Err(BuildFailure::Ignored(_ignored_event)) => runner_metrics.increment_ignored(),
        }
    }

    inventory_writer.end()?;

    Ok(())
}

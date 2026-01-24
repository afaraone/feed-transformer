use thiserror::Error;

use crate::{
    inventory_writer::{InventoryWriter, WriteError},
    runner_metrics::RunnerMetrics,
    sk::{BuildFailure, build_event},
    stream_iterator::StreamIterator,
    tm::Mapper,
};

#[derive(Error, Debug)]
pub enum ImportError {
    #[error("{0}")]
    WriteError(#[from] WriteError),
}

pub fn import(
    events: StreamIterator,
    runner_metrics: &mut RunnerMetrics,
    writer: &mut InventoryWriter,
) -> Result<(), ImportError> {
    writer.start()?;

    for tm_event in events {
        let Ok(tm_event) = tm_event else {
            runner_metrics.increment_unparsed();
            continue;
        };

        let mapper = Mapper::new(tm_event);

        match build_event(mapper) {
            Ok(songkick_event) => {
                writer.add(&songkick_event)?;
                runner_metrics.increment_accepted();
            }
            Err(BuildFailure::Invalid {
                source: _,
                errors: _,
            }) => runner_metrics.increment_invalid(),
            Err(BuildFailure::Ignored { source: _ }) => runner_metrics.increment_ignored(),
        }
    }

    writer.end()?;

    Ok(())
}

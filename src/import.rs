use thiserror::Error;

use crate::{
    inventory_writer::{InventoryWriter, WriteError},
    runner_metrics::RunnerMetrics,
    stream_iterator::StreamIterator,
    tm,
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
        if let Ok(tm_event) = tm_event {
            match tm::mapper::map(tm_event) {
                tm::mapper::MapResult::Valid(songkick_event) => {
                    writer.add_event(&songkick_event)?;
                    runner_metrics.increment_accepted();
                }
                tm::mapper::MapResult::Ignored => runner_metrics.increment_ignored(),
                tm::mapper::MapResult::Invalid => runner_metrics.increment_invalid(),
            }
        } else {
            runner_metrics.increment_unparsed();
        }
    }

    writer.end()?;

    Ok(())
}

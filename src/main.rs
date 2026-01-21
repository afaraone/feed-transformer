use feed_transformer::parse_file_path::parse_file_path;
#[cfg(feature = "track-mem")]
use feed_transformer::performance_metrics::PerformanceMetrics;
use feed_transformer::runner_metrics::RunnerMetrics;
use feed_transformer::stream_iterator::StreamIterator;
use feed_transformer::{inventory_writer::InventoryWriter, tm_event::TransformError};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner_metrics = RunnerMetrics::new();
    let file_path = parse_file_path()?;
    let stream_iterator = StreamIterator::new(&file_path)?;
    let mut writer = InventoryWriter::new("output/success.xml")?;

    writer.start()?;

    for event in stream_iterator {
        if let Ok(event) = event {
            match event.try_into_sk_event() {
                Ok(songkick_event) => {
                    writer.add_event(&songkick_event)?;
                    runner_metrics.increment_accepted();
                }
                Err(TransformError::IgnoredEvent) => runner_metrics.increment_ignored(),
                Err(TransformError::InvalidEvent(_)) => runner_metrics.increment_invalid(),
            }
        } else {
            runner_metrics.increment_unparsed();
        }
    }

    writer.end()?;
    println!("{}", runner_metrics);

    Ok(())
}

fn main() {
    #[cfg(feature = "track-mem")]
    let performance_metrics = PerformanceMetrics::start();

    let result = run();

    #[cfg(feature = "track-mem")]
    performance_metrics.report();

    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

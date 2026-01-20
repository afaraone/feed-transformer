#[cfg(feature = "track-mem")]
use feed_transformer::performance_metrics::PerformanceMetrics;
use feed_transformer::stream_iterator::StreamIterator;
use feed_transformer::{inventory_writer::InventoryWriter, tm_event::TransformError};
use std::env;

const TOP_LEVEL_KEY: &str = "events";

fn parse_file_path() -> Option<String> {
    let mut args = env::args();
    args.next(); // Skip the name of the binary being run
    args.next()
}

fn main() {
    #[cfg(feature = "track-mem")]
    let performance_metrics = PerformanceMetrics::start();

    let file_path = parse_file_path().unwrap_or_else(|| {
        eprintln!("Please provide a file path!");
        std::process::exit(1);
    });

    let stream_iterator = StreamIterator::new(&file_path, TOP_LEVEL_KEY).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let mut writer = InventoryWriter::new("output/success.xml").unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    writer.start().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    let mut accepted_events = 0;
    let mut invalid_events = 0;
    let mut ignored_events = 0;
    let mut unparsed_events = 0;

    for event in stream_iterator {
        if let Ok(event) = event {
            match event.try_into_sk_event() {
                Ok(songkick_event) => {
                    writer.add_event(&songkick_event).unwrap_or_else(|e| {
                        eprintln!("{e}");
                        std::process::exit(1);
                    });
                    accepted_events += 1;
                }
                Err(TransformError::IgnoredEvent) => ignored_events += 1,
                Err(TransformError::InvalidEvent(_)) => invalid_events += 1,
            }
        } else {
            unparsed_events += 1;
        }
    }

    writer.end().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    println!("---------------------------------");
    println!("Ingestion complete:");
    println!("Accepted events: {}", accepted_events);
    println!("Invalid events: {}", invalid_events);
    println!("Ignored events: {}", ignored_events);
    println!("Unparsed events: {}", unparsed_events);
    println!("---------------------------------");

    #[cfg(feature = "track-mem")]
    performance_metrics.report();
}

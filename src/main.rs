use feed_transformer::inventory_writer::InventoryWriter;
#[cfg(feature = "track-mem")]
use feed_transformer::performance_metrics::PerformanceMetrics;
use feed_transformer::stream_iterator::StreamIterator;
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

    let songkick_events = stream_iterator
        .flatten()
        .filter_map(|e| e.try_into_sk_event().ok());

    for event in songkick_events {
        writer.add_event(&event).unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });
    }

    writer.end().unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    #[cfg(feature = "track-mem")]
    performance_metrics.report();
}

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

    let count = stream_iterator
        .flatten()
        .filter_map(|e| e.try_into_sk_event().ok())
        .count();

    println!("{} events found!", count);

    #[cfg(feature = "track-mem")]
    performance_metrics.report();
}

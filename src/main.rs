use feed_transformer::import::import;
use feed_transformer::parse_file_path::parse_file_path;
#[cfg(feature = "track-mem")]
use feed_transformer::performance_metrics::PerformanceMetrics;
use feed_transformer::runner_metrics::RunnerMetrics;
use feed_transformer::stream_iterator::StreamIterator;
use feed_transformer::writer::{Writer, XmlWriter};

// Core logic here unwrapping to a Result to keep main() clean
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner_metrics = RunnerMetrics::new();
    let file_path = parse_file_path()?;
    let stream_iterator = StreamIterator::new(&file_path)?;
    let inventory_writer = XmlWriter::new("output/success.xml")?;

    import(stream_iterator, &mut runner_metrics, inventory_writer)?;
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

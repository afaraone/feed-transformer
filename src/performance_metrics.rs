#![cfg(feature = "track-mem")]

use peak_alloc::PeakAlloc;
use std::time::Instant;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

pub struct PerformanceMetrics {
    start_time: Instant,
}

impl PerformanceMetrics {
    pub fn start() -> Self {
        PerformanceMetrics {
            start_time: std::time::Instant::now(),
        }
    }

    pub fn report(&self) {
        let duration = self.start_time.elapsed();

        println!("---------------------------------");
        println!("Performance Metrics:");
        println!("Peak usage:    {:.2} MB", PEAK_ALLOC.peak_usage_as_mb());
        println!("Time elapsed:  {:.2}s", duration.as_secs_f64());
        println!("---------------------------------");
    }
}

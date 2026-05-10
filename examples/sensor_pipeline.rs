//! Multi-sensor ingestion pattern
//!
//! Demonstrates coordinating multiple high-throughput sensor streams
//! with minimal latency and memory overhead.

use sensor_systems::{
    pipelines::{sensor_ingestion::*, buffer_management::*},
    debugging::profiling::*,
};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Multi-Sensor Ingestion Pipeline ===");
    println!();

    // Create ingestion pipeline with 10k frame capacity
    let pipeline = Arc::new(SensorIngestionPipeline::new(10_000));

    // Create latency profiler
    let latency_stats = Arc::new(LatencyStats::new(1000));

    // Simulate multi-sensor ingestion
    let mut handles = vec![];

    for sensor_id in 0..4 {
        let pipeline = Arc::clone(&pipeline);
        let stats = Arc::clone(&latency_stats);

        let handle = thread::spawn(move || {
            for frame_num in 0..100 {
                let _scope = LatencyScope::start(Some(Arc::clone(&stats)));

                let frame = SensorFrame {
                    timestamp_ns: frame_num * 1_000_000,
                    sensor_id,
                    data: vec![0u8; 1024], // 1KB sample data
                };

                if pipeline.ingest(frame) {
                    // Frame accepted
                } else {
                    // Buffer full - frame dropped (backpressure)
                }
                thread::sleep(Duration::from_micros(100));
            }
        });

        handles.push(handle);
    }

    // Wait for all sensors to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Report statistics
    println!("Pipeline Statistics:");
    let stats = pipeline.stats();
    println!("  Frames received: {}", stats.frames_received);
    println!("  Frames dropped: {}", stats.frames_dropped);
    println!("  Total bytes ingested: {} MB", stats.total_bytes / 1_000_000);

    if let Some(min) = latency_stats.min() {
        println!("\nLatency Statistics:");
        println!("  Min: {} ns", min);
        println!("  Max: {} ns", latency_stats.max().unwrap());
        println!("  P50: {} ns", latency_stats.percentile(50.0).unwrap());
        println!("  P99: {} ns", latency_stats.percentile(99.0).unwrap());
        if let Some(avg) = latency_stats.avg() {
            println!("  Avg: {:.0} ns", avg);
        }
    }
}

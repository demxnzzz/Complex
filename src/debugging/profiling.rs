//! Latency measurement and profiling hooks
//!
//! Production-grade profiling for latency-sensitive paths without
//! performance impact in non-profiling builds.

use std::time::{Instant, Duration};
use std::collections::VecDeque;
use std::sync::Mutex;

/// High-resolution latency measurement
#[derive(Clone, Copy, Debug)]
pub struct LatencyMeasurement {
    pub duration_ns: u64,
    pub timestamp: Instant,
}

/// Latency statistics collector
pub struct LatencyStats {
    measurements: Mutex<VecDeque<u64>>,
    max_samples: usize,
}

impl LatencyStats {
    /// Create a new latency stats collector
    pub fn new(max_samples: usize) -> Self {
        Self {
            measurements: Mutex::new(VecDeque::with_capacity(max_samples)),
            max_samples,
        }
    }

    /// Record a latency measurement
    pub fn record(&self, duration_ns: u64) {
        let mut measurements = self.measurements.lock().unwrap();
        measurements.push_back(duration_ns);
        if measurements.len() > self.max_samples {
            measurements.pop_front();
        }
    }

    /// Get percentile latency
    pub fn percentile(&self, p: f32) -> Option<u64> {
        let measurements = self.measurements.lock().unwrap();
        if measurements.is_empty() {
            return None;
        }

        let mut sorted: Vec<_> = measurements.iter().copied().collect();
        sorted.sort_unstable();

        let idx = ((p / 100.0) * sorted.len() as f32) as usize;
        Some(sorted[idx.min(sorted.len() - 1)])
    }

    /// Get minimum latency
    pub fn min(&self) -> Option<u64> {
        self.measurements.lock().unwrap().iter().min().copied()
    }

    /// Get maximum latency
    pub fn max(&self) -> Option<u64> {
        self.measurements.lock().unwrap().iter().max().copied()
    }

    /// Get average latency
    pub fn avg(&self) -> Option<f64> {
        let measurements = self.measurements.lock().unwrap();
        if measurements.is_empty() {
            return None;
        }
        Some(measurements.iter().sum::<u64>() as f64 / measurements.len() as f64)
    }
}

/// Scoped latency measurement helper
pub struct LatencyScope {
    start: Instant,
    stats: Option<std::sync::Arc<LatencyStats>>,
}

impl LatencyScope {
    /// Start measuring latency
    pub fn start(stats: Option<std::sync::Arc<LatencyStats>>) -> Self {
        Self {
            start: Instant::now(),
            stats,
        }
    }
}

impl Drop for LatencyScope {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        if let Some(stats) = &self.stats {
            stats.record(duration.as_nanos() as u64);
        }
    }
}

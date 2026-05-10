//! Ring buffer and memory pooling strategies
//!
//! Zero-copy ring buffer design for sustained high-throughput ingestion
//! with explicit backpressure and dropped frame handling.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Pre-allocated ring buffer for sensor data
/// Designed to handle backpressure and frame drops gracefully
pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    write_pos: AtomicUsize,
    read_pos: AtomicUsize,
}

impl<T: Clone> RingBuffer<T> {
    /// Create a new ring buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }

        Self {
            buffer,
            write_pos: AtomicUsize::new(0),
            read_pos: AtomicUsize::new(0),
        }
    }

    /// Try to write an item to the buffer
    /// Returns true if successful, false if full (backpressure)
    pub fn try_write(&mut self, item: T) -> bool {
        let write_pos = self.write_pos.load(Ordering::Acquire);
        let next_write = (write_pos + 1) % self.buffer.len();
        let read_pos = self.read_pos.load(Ordering::Acquire);

        if next_write == read_pos {
            // Buffer full
            return false;
        }

        self.buffer[write_pos] = Some(item);
        self.write_pos.store(next_write, Ordering::Release);
        true
    }

    /// Try to read an item from the buffer
    pub fn try_read(&mut self) -> Option<T> {
        let read_pos = self.read_pos.load(Ordering::Acquire);
        let write_pos = self.write_pos.load(Ordering::Acquire);

        if read_pos == write_pos {
            // Buffer empty
            return None;
        }

        let item = self.buffer[read_pos].take();
        self.read_pos.store((read_pos + 1) % self.buffer.len(), Ordering::Release);
        item
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.write_pos.load(Ordering::Acquire) == self.read_pos.load(Ordering::Acquire)
    }

    /// Get utilization as percentage
    pub fn utilization(&self) -> f32 {
        let write_pos = self.write_pos.load(Ordering::Acquire);
        let read_pos = self.read_pos.load(Ordering::Acquire);
        let used = if write_pos >= read_pos {
            write_pos - read_pos
        } else {
            self.buffer.len() - read_pos + write_pos
        };
        (used as f32 / self.buffer.len() as f32) * 100.0
    }
}

//! Sensor Systems: Production Rust for Embedded Robotics
//!
//! Core abstraction for low-latency systems patterns in embedded robotics.
//! Demonstrates production-grade approaches to sensor ingestion, memory management,
//! IPC, timing synchronization, and systems-level debugging.

pub mod pipelines;
pub mod systems;
pub mod debugging;

pub use pipelines::{sensor_ingestion, buffer_management};
pub use systems::{ipc, timing, scheduling};
pub use debugging::{profiling, error_context};

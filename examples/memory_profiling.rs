//! Memory allocation tracking and analysis
//!
//! Demonstrates memory profiling patterns for understanding
//! allocation behavior in hot paths.

use sensor_systems::pipelines::buffer_management::*;

fn main() {
    println!("=== Memory Profiling ===");
    println!();

    // Create ring buffers of various sizes
    let buffer_configs = vec![
        ("Small (256 items)", 256),
        ("Medium (4K items)", 4096),
        ("Large (64K items)", 65536),
    ];

    for (name, capacity) in buffer_configs {
        let mut buffer: RingBuffer<Vec<u8>> = RingBuffer::new(capacity);

        // Fill buffer with sample data
        let mut written = 0;
        for i in 0..capacity {
            let data = vec![0u8; 1024]; // 1KB per entry
            if buffer.try_write(data) {
                written += 1;
            } else {
                break;
            }
        }

        println!("{}:", name);
        println!("  Capacity: {} items", capacity);
        println!("  Written: {}", written);
        println!("  Utilization: {:.1}%", buffer.utilization());
        println!("  Memory per item: 1024 bytes");
        println!("  Total used: {} MB", (written * 1024) / (1024 * 1024));
        println!();
    }

    println!("Allocation Strategy:");
    println!("  - Pre-allocated ring buffers (no allocation in hot path)");
    println!("  - Fixed-size circular data structure");
    println!("  - Atomic operations for lock-free coordination");
    println!("  - Backpressure handling (buffer full = drop frame)");
}

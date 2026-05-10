//! Latency profiling benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sensor_systems::pipelines::buffer_management::*;

fn ring_buffer_write(c: &mut Criterion) {
    c.bench_function("ring_buffer_write_256", |b| {
        let mut buffer: RingBuffer<Vec<u8>> = RingBuffer::new(256);
        b.iter(|| {
            let data = vec![black_box(0u8); 1024];
            let _ = buffer.try_write(data);
        });
    });

    c.bench_function("ring_buffer_write_4096", |b| {
        let mut buffer: RingBuffer<Vec<u8>> = RingBuffer::new(4096);
        b.iter(|| {
            let data = vec![black_box(0u8); 1024];
            let _ = buffer.try_write(data);
        });
    });
}

criterion_group!(benches, ring_buffer_write);
criterion_main!(benches);

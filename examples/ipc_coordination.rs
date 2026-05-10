//! Process-to-process data flow coordination
//!
//! Demonstrates IPC patterns for distributed sensor processing
//! across multiple processes (typical robotics middleware pattern).

use sensor_systems::systems::ipc::*;

fn main() {
    println!("=== IPC Process Coordination ===");
    println!();

    // Create shared memory for sensor buffer ring
    match SharedMemoryHandle::new("/sensor_buffer", 16 * 1024 * 1024) {
        Ok(shm) => {
            println!("Shared memory created:");
            println!("  Name: {}", shm.name);
            println!("  Size: {} MB", shm.size_bytes / (1024 * 1024));
        }
        Err(e) => eprintln!("Failed to create shared memory: {}", e),
    }

    // Create named pipes for synchronization
    match NamedPipeHandle::new("/tmp/sensor_sync_fifo") {
        Ok(pipe) => {
            println!("Named pipe created:");
            println!("  Path: {}", pipe.path);
        }
        Err(e) => eprintln!("Failed to create named pipe: {}", e),
    }

    // Initialize multi-process coordinator
    let mut coordinator = ProcessCoordinator::new(4);
    println!("\nProcess Coordinator initialized for {} processes", coordinator.num_processes);

    // Set up IPC infrastructure
    match coordinator.setup_ipc() {
        Ok(_) => {
            println!("IPC infrastructure initialized");
            println!("\nProduction configuration:");
            println!("  - Sensor data in shared memory");
            println!("  - Named pipes for event signaling");
            println!("  - POSIX semaphores for synchronization");
            println!("  - Process 1: Sensor ingestion (producer)");
            println!("  - Process 2: IMU processing");
            println!("  - Process 3: LiDAR processing");
            println!("  - Process 4: Data fusion (consumer)");
        }
        Err(e) => eprintln!("Failed to setup IPC: {}", e),
    }
}

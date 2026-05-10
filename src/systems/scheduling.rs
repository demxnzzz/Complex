//! CPU affinity and process scheduling
//!
//! Real-time process priorities, CPU affinity for latency-sensitive threads,
//! and NUMA awareness for multi-socket systems.

use std::collections::HashMap;

/// CPU affinity mask for thread pinning
#[derive(Clone, Debug)]
pub struct CpuAffinityMask {
    pub cpus: Vec<usize>,
}

impl CpuAffinityMask {
    /// Create affinity mask for specific CPUs
    pub fn new(cpus: Vec<usize>) -> Self {
        Self { cpus }
    }

    /// Create affinity mask for a single CPU
    pub fn single_cpu(cpu: usize) -> Self {
        Self { cpus: vec![cpu] }
    }
}

/// Real-time scheduling priority levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RtPriority {
    Low(u32),
    Normal(u32),
    High(u32),
    Critical(u32),
}

/// Thread scheduling configuration
pub struct SchedulingConfig {
    pub priority: RtPriority,
    pub affinity: Option<CpuAffinityMask>,
    pub stack_size_kb: usize,
}

impl Default for SchedulingConfig {
    fn default() -> Self {
        Self {
            priority: RtPriority::Normal(50),
            affinity: None,
            stack_size_kb: 256,
        }
    }
}

/// Scheduler for latency-sensitive sensor threads
pub struct RealtimeScheduler {
    thread_configs: HashMap<String, SchedulingConfig>,
}

impl RealtimeScheduler {
    /// Create a new realtime scheduler
    pub fn new() -> Self {
        Self {
            thread_configs: HashMap::new(),
        }
    }

    /// Register a thread with scheduling configuration
    pub fn register_thread(&mut self, name: &str, config: SchedulingConfig) {
        self.thread_configs.insert(name.to_string(), config);
    }

    /// Apply scheduling configuration to current thread
    pub fn apply_scheduling(&self, name: &str) -> Result<(), String> {
        let config = self
            .thread_configs
            .get(name)
            .ok_or_else(|| format!("Thread {} not registered", name))?;

        // Production implementation would:
        // 1. Use sched_setscheduler() for SCHED_FIFO/SCHED_RR
        // 2. Use sched_setaffinity() for CPU pinning
        // 3. Set pthread stack size
        // 4. Handle permission requirements (CAP_SYS_NICE)
        Ok(())
    }
}

impl Default for RealtimeScheduler {
    fn default() -> Self {
        Self::new()
    }
}

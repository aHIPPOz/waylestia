//! Module Perf — CPU/GPU/RAM monitoring, performance tracking

use sysinfo::{System, SystemExt, CpuExt, Pid, ProcessExt};
use crate::state::PerfMetrics;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PerfMonitor {
    system: System,
}

impl PerfMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    /// Update and return current performance metrics
    pub fn update(&mut self) -> PerfMetrics {
        self.system.refresh_all();
        
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        
        let total_memory = self.system.total_memory() as f32;
        let used_memory = self.system.used_memory() as f32;
        let ram_usage = if total_memory > 0.0 {
            (used_memory / total_memory) * 100.0
        } else {
            0.0
        };
        
        let uptime_seconds = self.system.uptime();
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        PerfMetrics {
            cpu_usage,
            gpu_usage: 0.0, // TODO: Integrate GPU monitoring (nvidia-smi, amdgpu, intel)
            ram_usage,
            ram_used_mb: (used_memory / 1024.0 / 1024.0) as u32,
            ram_total_mb: (total_memory / 1024.0 / 1024.0) as u32,
            uptime_seconds,
            timestamp,
        }
    }

    /// Get metrics for a specific process
    pub fn get_process_metrics(&self, pid: u32) -> Option<ProcessMetrics> {
        if let Some(process) = self.system.process(Pid::from(pid)) {
            Some(ProcessMetrics {
                pid,
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_mb: process.memory() as u32 / 1024,
            })
        } else {
            None
        }
    }

    /// Get top N processes by CPU usage
    pub fn top_processes_by_cpu(&self, n: usize) -> Vec<ProcessMetrics> {
        let mut processes: Vec<_> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessMetrics {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_mb: process.memory() as u32 / 1024,
            })
            .collect();
        
        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.into_iter().take(n).collect()
    }

    /// Get top N processes by RAM usage
    pub fn top_processes_by_memory(&self, n: usize) -> Vec<ProcessMetrics> {
        let mut processes: Vec<_> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| ProcessMetrics {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory_mb: process.memory() as u32 / 1024,
            })
            .collect();
        
        processes.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
        processes.into_iter().take(n).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: u32,
}

impl Default for PerfMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perf_monitor() {
        let mut monitor = PerfMonitor::new();
        let metrics = monitor.update();
        assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 100.0);
        assert!(metrics.ram_usage >= 0.0 && metrics.ram_usage <= 100.0);
    }
}

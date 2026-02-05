//! Module State — global state management, thread-safe synchronization

use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfMetrics {
    pub cpu_usage: f32,
    pub gpu_usage: f32,
    pub ram_usage: f32,
    pub ram_used_mb: u32,
    pub ram_total_mb: u32,
    pub uptime_seconds: u64,
    pub timestamp: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub perf: Option<PerfMetrics>,
    pub active_workspace: Option<String>,
    pub active_window: Option<String>,
    pub windows_count: u32,
}

pub type SharedState = Arc<Mutex<GlobalState>>;

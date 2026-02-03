//! Module Perf — récupération CPU/GPU/RAM, uptime

use sysinfo::{System, SystemExt, CpuExt};

pub fn get_perf() -> (f32, f32, f32, u64) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cpu = sys.global_cpu_info().cpu_usage();
    let ram = sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0;
    let gpu = 0.0; // TODO: Intégrer une lib GPU Linux (nvidia, amdgpu...)
    let uptime = sys.uptime();
    (cpu, gpu, ram, uptime)
}

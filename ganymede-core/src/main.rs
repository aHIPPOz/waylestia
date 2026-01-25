//! Daemon principal du shell Wayland custom (Hyprland-centric)

use serde::{Deserialize, Serialize};
use tokio::net::UnixListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[derive(Serialize, Deserialize, Debug)]
pub struct PerfUpdate {
    pub cpu: f32,
    pub gpu: f32,
    pub ram: f32,
    pub uptime: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[ganymede-core] Daemon démarré");
    // TODO: Init Hyprland IPC, state, etc.
    // Expose dummy perf state via Unix socket (MVP)
    let listener = UnixListener::bind("/tmp/ganymede-core.sock")?;
    loop {
        let (stream, _) = listener.accept().await?;
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf).await?;
        // Parse JSON, respond with dummy perf
        let msg: serde_json::Value = serde_json::from_str(&buf)?;
        if msg["type"] == "get_perf" {
            let perf = PerfUpdate { cpu: 12.3, gpu: 8.1, ram: 42.0, uptime: 123456 };
            let resp = serde_json::to_string(&perf)? + "\n";
            reader.get_mut().write_all(resp.as_bytes()).await?;
        }
    }
}

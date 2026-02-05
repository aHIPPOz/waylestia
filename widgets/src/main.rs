//! Waylestia Widgets Engine — Daemon that manages widget lifecycle and rendering

use waylestia_widgets::{
    WidgetLoader, WidgetRenderer, WidgetStateManager, IPCServer,
    WIDGETS_DIR, IPC_SOCKET,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[waylestia-widgets-engine] Starting widget daemon...");

    // Initialize widget loader
    let loader = WidgetLoader::new(WIDGETS_DIR);
    
    // Discover available widgets
    match loader.discover() {
        Ok(widgets) => {
            println!("[waylestia-widgets] Discovered {} widgets:", widgets.len());
            for (widget_id, path) in &widgets {
                println!("  - {} ({})", widget_id, path.display());
            }
        }
        Err(e) => {
            eprintln!("[waylestia-widgets] Warning: Could not discover widgets: {}", e);
        }
    }

    // Initialize renderer
    let renderer = WidgetRenderer::new(Default::default());
    println!("[waylestia-widgets] Renderer initialized (GJS enabled, IPC enabled)");

    // Initialize state manager
    let state = Arc::new(Mutex::new(WidgetStateManager::new()));
    println!("[waylestia-widgets] State manager initialized");

    // Start IPC server
    let ipc_server = IPCServer::new(IPC_SOCKET.to_string(), state);
    
    println!("[waylestia-widgets] Starting IPC server on {}", IPC_SOCKET);
    ipc_server.start().await?;

    Ok(())
}

//! Waylestia Widgets Engine — Rust-based widget runtime using Servo webview

pub mod manifest;
pub mod loader;
pub mod renderer;
pub mod ipc;
pub mod state;

pub use manifest::{WidgetManifest, WidgetMetadata};
pub use loader::WidgetLoader;
pub use renderer::WidgetRenderer;
pub use state::WidgetState;

pub const WIDGETS_DIR: &str = "assets/widgets";
pub const IPC_SOCKET: &str = "/tmp/waylestia-widgets.sock";

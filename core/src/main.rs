//! Daemon principal du shell Wayland complet (Hyprland-centric, suite OS)

mod hyprland;
mod ipc;
mod state;
mod perf;
mod media;
mod security;

use state::{GlobalState, SharedState};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[waylestia-core] Daemon démarré");
    // Initialisation de l’état global partagé
    let state: SharedState = Arc::new(Mutex::new(GlobalState::default()));
    // TODO: Lancer Hyprland IPC, perf, media, sécurité, etc.
    // TODO: Lancer serveur IPC (Unix socket)
    // TODO: Boucle d’event principale
    Ok(())
}

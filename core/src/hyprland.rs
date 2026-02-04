//! Module Hyprland — gestion IPC, events, workspaces, tiling

use hyprland::data::{Client, Workspace};
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprData;

pub fn start_hyprland_event_loop() {
    // TODO: Lancer un thread d’écoute des events Hyprland
    // Exemple : subscribe à window events, workspace events, etc.
    // EventListener::new().add_window_event(...)
}

pub fn get_workspaces() -> Vec<Workspace> {
    // TODO: Retourner la liste des workspaces Hyprland
    vec![]
}

pub fn get_clients() -> Vec<Client> {
    // TODO: Retourner la liste des fenêtres Hyprland
    vec![]
}

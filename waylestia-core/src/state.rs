//! Module State — gestion état global, synchronisation, accès thread-safe

use std::sync::{Arc, Mutex};

#[derive(Default, Debug, Clone)]
pub struct GlobalState {
    pub cpu: f32,
    pub gpu: f32,
    pub ram: f32,
    pub uptime: u64,
    pub media: Option<String>,
    // TODO: Ajouter workspaces, fenêtres, sécurité, etc.
}

pub type SharedState = Arc<Mutex<GlobalState>>;

//! Widget State — global state management and widget lifecycle

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct WidgetInstance {
    pub instance_id: String,
    pub widget_id: String,
    pub state: serde_json::Value,
    pub visible: bool,
    pub geometry: WidgetGeometry,
}

#[derive(Debug, Clone, Default)]
pub struct WidgetGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub type SharedWidgetState = Arc<Mutex<WidgetStateManager>>;

#[derive(Debug)]
pub struct WidgetStateManager {
    instances: HashMap<String, WidgetInstance>,
}

impl Default for WidgetStateManager {
    fn default() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }
}

impl WidgetStateManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new widget instance
    pub fn create_instance(
        &mut self,
        widget_id: String,
    ) -> String {
        let instance_id = Uuid::new_v4().to_string();
        let instance = WidgetInstance {
            instance_id: instance_id.clone(),
            widget_id,
            state: json!({}),
            visible: false,
            geometry: WidgetGeometry::default(),
        };
        self.instances.insert(instance_id.clone(), instance);
        instance_id
    }

    /// Get instance by ID
    pub fn get_instance(&self, instance_id: &str) -> Option<WidgetInstance> {
        self.instances.get(instance_id).cloned()
    }

    /// Update instance state
    pub fn update_state(&mut self, instance_id: &str, new_state: serde_json::Value) -> anyhow::Result<()> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            instance.state = new_state;
            Ok(())
        } else {
            anyhow::bail!("Widget instance not found: {}", instance_id)
        }
    }

    /// Update instance geometry
    pub fn update_geometry(
        &mut self,
        instance_id: &str,
        geometry: WidgetGeometry,
    ) -> anyhow::Result<()> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            instance.geometry = geometry;
            Ok(())
        } else {
            anyhow::bail!("Widget instance not found: {}", instance_id)
        }
    }

    /// Show/hide instance
    pub fn set_visible(&mut self, instance_id: &str, visible: bool) -> anyhow::Result<()> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            instance.visible = visible;
            Ok(())
        } else {
            anyhow::bail!("Widget instance not found: {}", instance_id)
        }
    }

    /// Destroy instance
    pub fn destroy_instance(&mut self, instance_id: &str) -> anyhow::Result<()> {
        self.instances.remove(instance_id);
        Ok(())
    }

    /// List all instances
    pub fn list_instances(&self) -> Vec<WidgetInstance> {
        self.instances.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_creation() {
        let mut state = WidgetStateManager::new();
        let instance_id = state.create_instance("test-widget".to_string());
        assert!(state.get_instance(&instance_id).is_some());
    }

    #[test]
    fn test_widget_lifecycle() {
        let mut state = WidgetStateManager::new();
        let instance_id = state.create_instance("test-widget".to_string());
        
        let _ = state.set_visible(&instance_id, true);
        let instance = state.get_instance(&instance_id).unwrap();
        assert!(instance.visible);
        
        let _ = state.destroy_instance(&instance_id);
        assert!(state.get_instance(&instance_id).is_none());
    }
}

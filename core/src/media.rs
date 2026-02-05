//! Module Media — PipeWire integration, audio device management, volume control

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub volume: f32,          // 0.0-1.0
    pub muted: bool,
    pub active: bool,
    pub sample_rate: u32,
    pub channels: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceType {
    Speaker,
    Headphones,
    Headset,
    HDMI,
    USB,
    Bluetooth,
    LineOut,
    Other,
}

pub struct MediaManager {
    devices: HashMap<String, AudioDevice>,
    active_device: Option<String>,
}

impl MediaManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            active_device: None,
        }
    }

    /// Initialize and discover audio devices
    pub fn initialize(&mut self) -> anyhow::Result<()> {
        // TODO: Integrate with PipeWire
        // For now, add placeholder devices
        self.devices.insert(
            "speaker-0".to_string(),
            AudioDevice {
                id: "speaker-0".to_string(),
                name: "Built-in Speaker".to_string(),
                device_type: DeviceType::Speaker,
                volume: 0.7,
                muted: false,
                active: true,
                sample_rate: 48000,
                channels: 2,
            },
        );

        self.devices.insert(
            "headphone-0".to_string(),
            AudioDevice {
                id: "headphone-0".to_string(),
                name: "Built-in Headphones".to_string(),
                device_type: DeviceType::Headphones,
                volume: 0.5,
                muted: false,
                active: false,
                sample_rate: 48000,
                channels: 2,
            },
        );

        self.active_device = Some("speaker-0".to_string());
        Ok(())
    }

    /// List all audio devices
    pub fn list_devices(&self) -> Vec<AudioDevice> {
        self.devices.values().cloned().collect()
    }

    /// Get a specific device
    pub fn get_device(&self, device_id: &str) -> Option<AudioDevice> {
        self.devices.get(device_id).cloned()
    }

    /// Set volume for a device (0.0-1.0)
    pub fn set_volume(&mut self, device_id: &str, volume: f32) -> anyhow::Result<()> {
        let volume = volume.clamp(0.0, 1.0);
        if let Some(device) = self.devices.get_mut(device_id) {
            device.volume = volume;
            // TODO: Actually set volume via PipeWire
            Ok(())
        } else {
            anyhow::bail!("Device not found: {}", device_id)
        }
    }

    /// Mute/unmute a device
    pub fn set_muted(&mut self, device_id: &str, muted: bool) -> anyhow::Result<()> {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.muted = muted;
            // TODO: Actually mute via PipeWire
            Ok(())
        } else {
            anyhow::bail!("Device not found: {}", device_id)
        }
    }

    /// Switch active output device
    pub fn set_active_device(&mut self, device_id: &str) -> anyhow::Result<()> {
        if !self.devices.contains_key(device_id) {
            anyhow::bail!("Device not found: {}", device_id);
        }

        // Deactivate previous device
        if let Some(prev_id) = &self.active_device.clone() {
            if let Some(prev_device) = self.devices.get_mut(prev_id) {
                prev_device.active = false;
            }
        }

        // Activate new device
        if let Some(device) = self.devices.get_mut(device_id) {
            device.active = true;
        }

        self.active_device = Some(device_id.to_string());
        // TODO: Actually switch device via PipeWire
        Ok(())
    }

    /// Get active output device
    pub fn get_active_device(&self) -> Option<AudioDevice> {
        self.active_device
            .as_ref()
            .and_then(|id| self.devices.get(id).cloned())
    }

    /// Get volume of active device
    pub fn get_volume(&self) -> Option<f32> {
        self.get_active_device().map(|d| d.volume)
    }

    /// Set volume of active device
    pub fn set_active_volume(&mut self, volume: f32) -> anyhow::Result<()> {
        if let Some(device_id) = self.active_device.clone() {
            self.set_volume(&device_id, volume)
        } else {
            anyhow::bail!("No active device")
        }
    }
}

impl Default for MediaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_manager() {
        let mut manager = MediaManager::new();
        assert!(manager.initialize().is_ok());
        assert!(!manager.list_devices().is_empty());
    }

    #[test]
    fn test_volume_control() {
        let mut manager = MediaManager::new();
        let _ = manager.initialize();
        
        assert!(manager.set_volume("speaker-0", 0.5).is_ok());
        assert!(manager.set_volume("speaker-0", 1.5).is_ok()); // Should clamp to 1.0
        
        let device = manager.get_device("speaker-0").unwrap();
        assert_eq!(device.volume, 1.0);
    }
}

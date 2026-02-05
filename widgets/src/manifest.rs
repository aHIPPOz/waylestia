//! Widget Manifest — TOML-based widget metadata and configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetManifest {
    #[serde(rename = "widget")]
    pub metadata: WidgetMetadata,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub permissions: PermissionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub main: String, // Path to main HTML file (e.g., "index.html")
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplayConfig {
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default)]
    pub resizable: bool,
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default)]
    pub always_on_top: bool,
    #[serde(default)]
    pub transparent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionConfig {
    #[serde(default)]
    pub ipc: bool,
    #[serde(default)]
    pub filesystem: bool,
    #[serde(default)]
    pub network: bool,
    #[serde(default)]
    pub media: bool,
    #[serde(default)]
    pub notifications: bool,
}

fn default_width() -> u32 { 400 }
fn default_height() -> u32 { 300 }

impl WidgetManifest {
    pub fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.metadata.id.is_empty() {
            anyhow::bail!("Widget ID cannot be empty");
        }
        if self.metadata.main.is_empty() {
            anyhow::bail!("Widget main file must be specified");
        }
        if self.display.width == 0 || self.display.height == 0 {
            anyhow::bail!("Widget dimensions must be > 0");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_validation() {
        let manifest = WidgetManifest {
            metadata: WidgetMetadata {
                id: "test-widget".to_string(),
                name: "Test Widget".to_string(),
                version: "0.1.0".to_string(),
                description: "A test widget".to_string(),
                author: "Test Author".to_string(),
                license: "MIT".to_string(),
                main: "index.html".to_string(),
                icon: None,
                category: None,
            },
            display: DisplayConfig::default(),
            permissions: PermissionConfig::default(),
        };
        assert!(manifest.validate().is_ok());
    }
}

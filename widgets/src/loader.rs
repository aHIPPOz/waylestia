//! Widget Loader — discovers and loads widgets from filesystem

use crate::manifest::WidgetManifest;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Result};

pub struct WidgetLoader {
    widgets_dir: PathBuf,
}

impl WidgetLoader {
    pub fn new(widgets_dir: impl AsRef<Path>) -> Self {
        Self {
            widgets_dir: widgets_dir.as_ref().to_path_buf(),
        }
    }

    /// Discover all widgets in the widgets directory
    pub fn discover(&self) -> Result<Vec<(String, PathBuf)>> {
        if !self.widgets_dir.exists() {
            return anyhow::bail!("Widgets directory does not exist: {:?}", self.widgets_dir);
        }

        let mut widgets = Vec::new();
        for entry in std::fs::read_dir(&self.widgets_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let manifest_path = path.join("manifest.toml");
                if manifest_path.exists() {
                    let widget_id = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    widgets.push((widget_id, path));
                }
            }
        }

        Ok(widgets)
    }

    /// Load a specific widget by ID
    pub fn load_widget(&self, widget_id: &str) -> Result<WidgetManifest> {
        let widget_path = self.widgets_dir.join(widget_id);
        let manifest_path = widget_path.join("manifest.toml");

        if !manifest_path.exists() {
            return Err(anyhow!(
                "Widget manifest not found: {}",
                manifest_path.display()
            ));
        }

        let manifest = WidgetManifest::from_file(&manifest_path)?;
        manifest.validate()?;

        // Ensure main file exists
        let main_file = widget_path.join(&manifest.metadata.main);
        if !main_file.exists() {
            return Err(anyhow!(
                "Widget main file not found: {}",
                main_file.display()
            ));
        }

        Ok(manifest)
    }

    /// Get the root directory of a widget
    pub fn widget_root(&self, widget_id: &str) -> PathBuf {
        self.widgets_dir.join(widget_id)
    }

    /// Get the absolute path to widget's main file
    pub fn get_main_file(&self, widget_id: &str) -> Result<PathBuf> {
        let manifest = self.load_widget(widget_id)?;
        let root = self.widget_root(widget_id);
        Ok(root.join(&manifest.metadata.main))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_creation() {
        let loader = WidgetLoader::new("assets/widgets");
        assert!(loader.widgets_dir.ends_with("assets/widgets"));
    }
}

//! Widget Renderer — renders widgets using Servo webview engine

use crate::manifest::WidgetManifest;
use crate::state::WidgetInstance;
use std::path::PathBuf;

/// Configuration for the widget renderer
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub enable_gjs: bool,  // Use GJS for JavaScript instead of standard JS
    pub enable_ipc: bool,  // Enable IPC communication with core
    pub debug: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            enable_gjs: true,
            enable_ipc: true,
            debug: false,
        }
    }
}

/// Widget renderer using Servo
pub struct WidgetRenderer {
    config: RenderConfig,
}

impl WidgetRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self { config }
    }

    /// Generate HTML wrapper for widget
    pub fn generate_wrapper(
        &self,
        instance: &WidgetInstance,
        manifest: &WidgetManifest,
        widget_root: &PathBuf,
    ) -> anyhow::Result<String> {
        let wrapper = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        html, body {{
            width: 100%;
            height: 100%;
            overflow: hidden;
        }}
    </style>
    <script>
        // Widget IPC Bridge
        window.waylestia = {{
            instanceId: "{instance_id}",
            widgetId: "{widget_id}",
            sendMessage: function(type, data) {{
                if (window.gjs) {{
                    window.gjs.sendWidgetMessage({{
                        instance_id: this.instanceId,
                        widget_id: this.widgetId,
                        type: type,
                        data: data
                    }});
                }} else {{
                    console.warn('GJS bridge not available');
                }}
            }},
            onMessage: function(callback) {{
                if (window.gjs) {{
                    window.gjs.onWidgetMessage(this.instanceId, callback);
                }}
            }}
        }};
    </script>
</head>
<body>
    <div id="waylestia-widget-root"></div>
    <script type="module" src="{}"></script>
</body>
</html>"#,
            manifest.metadata.name,
            instance.instance_id,
            manifest.metadata.id,
            manifest.metadata.main
        );
        Ok(wrapper)
    }

    /// Prepare widget for rendering
    pub fn prepare_render(
        &self,
        instance: &WidgetInstance,
        manifest: &WidgetManifest,
        widget_root: &PathBuf,
    ) -> anyhow::Result<RenderJob> {
        manifest.validate()?;

        let wrapper = self.generate_wrapper(instance, manifest, widget_root)?;
        let widget_dir_url = format!(
            "file://{}",
            widget_root.display()
        );

        Ok(RenderJob {
            instance_id: instance.instance_id.clone(),
            widget_id: manifest.metadata.id.clone(),
            html_wrapper: wrapper,
            widget_root_url: widget_dir_url,
            width: manifest.display.width,
            height: manifest.display.height,
            transparent: manifest.display.transparent,
            always_on_top: manifest.display.always_on_top,
            config: self.config.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RenderJob {
    pub instance_id: String,
    pub widget_id: String,
    pub html_wrapper: String,
    pub widget_root_url: String,
    pub width: u32,
    pub height: u32,
    pub transparent: bool,
    pub always_on_top: bool,
    pub config: RenderConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::WidgetGeometry;

    #[test]
    fn test_renderer_creation() {
        let renderer = WidgetRenderer::new(RenderConfig::default());
        assert!(renderer.config.enable_gjs);
    }
}

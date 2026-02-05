//! Widget IPC — Protocol Buffer-based inter-process communication

use crate::state::SharedWidgetState;
use tokio::net::UnixListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde_json::{json, Value};
use anyhow::Result;

pub struct IPCServer {
    socket_path: String,
    state: SharedWidgetState,
}

#[derive(Debug, serde::Deserialize)]
pub struct IPCMessage {
    pub msg_type: String,
    pub widget_id: Option<String>,
    pub instance_id: Option<String>,
    pub data: Value,
}

#[derive(Debug, serde::Serialize)]
pub struct IPCResponse {
    pub success: bool,
    pub message: String,
    pub data: Value,
}

impl IPCServer {
    pub fn new(socket_path: String, state: SharedWidgetState) -> Self {
        Self {
            socket_path,
            state,
        }
    }

    pub async fn start(&self) -> Result<()> {
        // Remove existing socket if it exists
        let _ = std::fs::remove_file(&self.socket_path);

        let listener = UnixListener::bind(&self.socket_path)?;
        eprintln!("[Widgets IPC] Listening on {}", self.socket_path);

        loop {
            let (stream, _) = listener.accept().await?;
            let state = self.state.clone();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_client(stream, state).await {
                    eprintln!("[Widgets IPC] Client error: {}", e);
                }
            });
        }
    }

    async fn handle_client(
        stream: tokio::net::UnixStream,
        state: SharedWidgetState,
    ) -> Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        while reader.read_line(&mut line).await? > 0 {
            let response = match serde_json::from_str::<IPCMessage>(&line) {
                Ok(msg) => Self::process_message(msg, &state),
                Err(e) => IPCResponse {
                    success: false,
                    message: format!("JSON parse error: {}", e),
                    data: json!({}),
                },
            };

            let response_json = serde_json::to_string(&response)?;
            writer.write_all(response_json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
            
            line.clear();
        }

        Ok(())
    }

    fn process_message(msg: IPCMessage, state: &SharedWidgetState) -> IPCResponse {
        match msg.msg_type.as_str() {
            "create_instance" => {
                if let Some(widget_id) = msg.widget_id {
                    let mut state = state.lock().unwrap();
                    let instance_id = state.create_instance(widget_id);
                    IPCResponse {
                        success: true,
                        message: "Instance created".to_string(),
                        data: json!({ "instance_id": instance_id }),
                    }
                } else {
                    IPCResponse {
                        success: false,
                        message: "Missing widget_id".to_string(),
                        data: json!({}),
                    }
                }
            }
            "destroy_instance" => {
                if let Some(instance_id) = msg.instance_id {
                    let mut state = state.lock().unwrap();
                    match state.destroy_instance(&instance_id) {
                        Ok(_) => IPCResponse {
                            success: true,
                            message: "Instance destroyed".to_string(),
                            data: json!({}),
                        },
                        Err(e) => IPCResponse {
                            success: false,
                            message: e.to_string(),
                            data: json!({}),
                        },
                    }
                } else {
                    IPCResponse {
                        success: false,
                        message: "Missing instance_id".to_string(),
                        data: json!({}),
                    }
                }
            }
            "update_state" => {
                if let (Some(instance_id), data) = (msg.instance_id, msg.data) {
                    let mut state = state.lock().unwrap();
                    match state.update_state(&instance_id, data) {
                        Ok(_) => IPCResponse {
                            success: true,
                            message: "State updated".to_string(),
                            data: json!({}),
                        },
                        Err(e) => IPCResponse {
                            success: false,
                            message: e.to_string(),
                            data: json!({}),
                        },
                    }
                } else {
                    IPCResponse {
                        success: false,
                        message: "Missing instance_id or data".to_string(),
                        data: json!({}),
                    }
                }
            }
            "list_instances" => {
                let state = state.lock().unwrap();
                let instances: Vec<_> = state
                    .list_instances()
                    .iter()
                    .map(|i| json!({
                        "instance_id": i.instance_id,
                        "widget_id": i.widget_id,
                        "visible": i.visible,
                    }))
                    .collect();
                IPCResponse {
                    success: true,
                    message: "Instances listed".to_string(),
                    data: json!(instances),
                }
            }
            _ => IPCResponse {
                success: false,
                message: format!("Unknown message type: {}", msg.msg_type),
                data: json!({}),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_ipc_server_creation() {
        let state = Arc::new(Mutex::new(crate::state::WidgetStateManager::new()));
        let _server = IPCServer::new("/tmp/test-widgets.sock".to_string(), state);
    }
}

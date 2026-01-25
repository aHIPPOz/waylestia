//! Module IPC — gestion Unix socket, JSON, events, API locale

use tokio::net::UnixListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::state::SharedState;

pub async fn start_ipc_server(state: SharedState) -> anyhow::Result<()> {
    let listener = UnixListener::bind("/tmp/waylestia-core.sock")?;
    loop {
        let (stream, _) = listener.accept().await?;
        let mut reader = BufReader::new(stream);
        let mut buf = String::new();
        reader.read_line(&mut buf).await?;
        // TODO: Parse JSON, router les requêtes, répondre
    }
}

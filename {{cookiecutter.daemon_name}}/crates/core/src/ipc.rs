use std::path::{Path, PathBuf};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, info};

use crate::desired::DesiredState;
use crate::error::{CoreError, Result};

/// Local Unix-domain socket server stub (chassis-local IPC only).
///
/// Product Runtime API / MQTT stay out of this crate — wire them in product ADRs.
pub struct LocalIpcServer {
    path: PathBuf,
}

impl LocalIpcServer {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Bind and serve one JSON line request then return.
    /// Full product protocol is an ADR decision.
    pub async fn serve_once(&self, desired: DesiredState) -> Result<()> {
        if self.path.exists() {
            let _ = std::fs::remove_file(&self.path);
        }
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let listener =
            UnixListener::bind(&self.path).map_err(|e| CoreError::Ipc(e.to_string()))?;
        info!(path = %self.path.display(), "local ipc listening");

        let (mut stream, _addr) = listener
            .accept()
            .await
            .map_err(|e| CoreError::Ipc(e.to_string()))?;

        let mut buf = vec![0u8; 1024];
        let n = stream
            .read(&mut buf)
            .await
            .map_err(|e| CoreError::Ipc(e.to_string()))?;
        let req = String::from_utf8_lossy(&buf[..n]);
        debug!(%req, "ipc request");

        let body = serde_json::json!({
            "ok": true,
            "desired": desired,
        });
        let resp = format!("{body}\n");
        stream
            .write_all(resp.as_bytes())
            .await
            .map_err(|e| CoreError::Ipc(e.to_string()))?;

        Ok(())
    }

    /// Client helper for tests / demos.
    pub async fn request(path: impl AsRef<Path>, payload: &str) -> Result<String> {
        let mut stream = UnixStream::connect(path.as_ref())
            .await
            .map_err(|e| CoreError::Ipc(e.to_string()))?;
        stream
            .write_all(payload.as_bytes())
            .await
            .map_err(|e| CoreError::Ipc(e.to_string()))?;
        let mut buf = String::new();
        stream
            .read_to_string(&mut buf)
            .await
            .map_err(|e| CoreError::Ipc(e.to_string()))?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn serve_once_roundtrip() {
        let path = std::env::temp_dir().join(format!(
            "device-ipc-test-{}.sock",
            std::process::id()
        ));
        let server = LocalIpcServer::new(&path);
        let desired = DesiredState { enabled: true };

        let path_clone = path.clone();
        let serve = tokio::spawn(async move { server.serve_once(desired).await });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let resp = LocalIpcServer::request(&path_clone, "ping\n")
            .await
            .expect("client");
        assert!(resp.contains("\"ok\":true"));
        serve.await.expect("join").expect("serve");
        let _ = std::fs::remove_file(&path_clone);
    }
}

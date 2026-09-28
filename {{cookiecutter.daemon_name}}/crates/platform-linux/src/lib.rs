//! Linux host backend (device / CI targets).

use std::process::Command;

use async_trait::async_trait;
use device_core::error::{CoreError, Result};
use device_core::HostBackend;

#[derive(Debug, Default)]
pub struct LinuxHostBackend;

#[async_trait]
impl HostBackend for LinuxHostBackend {
    fn hostname(&self) -> String {
        hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".into())
    }

    async fn free_disk_bytes(&self, path: &str) -> Result<u64> {
        let path = path.to_string();
        tokio::task::spawn_blocking(move || free_disk_bytes_sync(&path))
            .await
            .map_err(|e| CoreError::Host(e.to_string()))?
    }
}

fn free_disk_bytes_sync(path: &str) -> Result<u64> {
    let out = Command::new("df")
        .args(["-k", "-P", path])
        .output()
        .map_err(|e| CoreError::Host(e.to_string()))?;
    if !out.status.success() {
        return Err(CoreError::Host(format!(
            "df failed: {}",
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text
        .lines()
        .nth(1)
        .ok_or_else(|| CoreError::Host("df: no data line".into()))?;
    let avail_k: u64 = line
        .split_whitespace()
        .nth(3)
        .ok_or_else(|| CoreError::Host("df: no avail column".into()))?
        .parse()
        .map_err(|e| CoreError::Host(format!("df parse: {e}")))?;
    Ok(avail_k.saturating_mul(1024))
}

use async_trait::async_trait;

use crate::error::Result;

/// Host OS operations the daemon needs. Implemented per platform crate.
#[async_trait]
pub trait HostBackend: Send + Sync {
    /// Reported hostname for identity / diagnostics.
    fn hostname(&self) -> String;

    /// Free bytes under `path` (best-effort).
    async fn free_disk_bytes(&self, path: &str) -> Result<u64>;
}

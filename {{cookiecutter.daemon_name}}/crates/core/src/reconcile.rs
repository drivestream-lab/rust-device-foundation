use std::sync::Arc;
use std::time::Duration;

use tokio::sync::watch;
use tracing::{debug, info};

use crate::desired::{DesiredState, ObservedState};
use crate::error::Result;
use crate::host::HostBackend;

/// Periodic observe → compare → act stub (desired-state reconciliation).
pub struct ReconcileLoop {
    host: Arc<dyn HostBackend>,
    desired_rx: watch::Receiver<DesiredState>,
    interval: Duration,
}

impl ReconcileLoop {
    pub fn new(
        host: Arc<dyn HostBackend>,
        desired_rx: watch::Receiver<DesiredState>,
        interval: Duration,
    ) -> Self {
        Self {
            host,
            desired_rx,
            interval,
        }
    }

    pub async fn observe(&self) -> Result<ObservedState> {
        let hostname = self.host.hostname();
        let free_disk_bytes = self.host.free_disk_bytes(".").await?;
        Ok(ObservedState {
            hostname,
            free_disk_bytes,
        })
    }

    /// One reconcile tick: observe, log desired vs observed. Product act steps TBD.
    pub async fn tick(&self) -> Result<()> {
        let desired = self.desired_rx.borrow().clone();
        let observed = self.observe().await?;
        debug!(?desired, ?observed, "reconcile tick");
        if desired.enabled {
            info!(
                hostname = %observed.hostname,
                free_disk_bytes = observed.free_disk_bytes,
                "daemon enabled; host observed"
            );
        }
        Ok(())
    }

    /// Run until the desired-state sender is dropped.
    pub async fn run(self) -> Result<()> {
        let mut ticker = tokio::time::interval(self.interval);
        loop {
            ticker.tick().await;
            if let Err(e) = self.tick().await {
                tracing::warn!(error = %e, "reconcile tick failed");
            }
            // Closed sender → has_changed returns Err
            if self.desired_rx.has_changed().is_err() {
                break;
            }
        }
        Ok(())
    }

    /// Run a bounded number of ticks (tests / short demos).
    pub async fn run_n(&self, n: usize) -> Result<()> {
        for i in 0..n {
            self.tick().await?;
            if i + 1 < n {
                tokio::time::sleep(self.interval).await;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CoreError;
    use async_trait::async_trait;

    struct FakeHost;

    #[async_trait]
    impl HostBackend for FakeHost {
        fn hostname(&self) -> String {
            "test-host".into()
        }

        async fn free_disk_bytes(&self, _path: &str) -> Result<u64> {
            Ok(1_000_000)
        }
    }

    #[tokio::test]
    async fn tick_observes_host() {
        let host: Arc<dyn HostBackend> = Arc::new(FakeHost);
        let (tx, rx) = watch::channel(DesiredState { enabled: true });
        let loop_ = ReconcileLoop::new(host, rx, Duration::from_millis(1));
        loop_.tick().await.expect("tick");
        drop(tx);
    }

    #[tokio::test]
    async fn observe_maps_host_error() {
        struct ErrHost;
        #[async_trait]
        impl HostBackend for ErrHost {
            fn hostname(&self) -> String {
                "x".into()
            }
            async fn free_disk_bytes(&self, _path: &str) -> Result<u64> {
                Err(CoreError::Host("disk".into()))
            }
        }
        let host: Arc<dyn HostBackend> = Arc::new(ErrHost);
        let (_tx, rx) = watch::channel(DesiredState::default());
        let loop_ = ReconcileLoop::new(host, rx, Duration::from_millis(1));
        assert!(loop_.observe().await.is_err());
    }
}

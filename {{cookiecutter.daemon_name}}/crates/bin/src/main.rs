//! {{ cookiecutter.daemon_name }} — chassis binary entrypoint.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use device_core::desired::DesiredState;
use device_core::ipc::LocalIpcServer;
use device_core::reconcile::ReconcileLoop;
use device_core::HostBackend;
use tracing::info;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
compile_error!("{{ cookiecutter.daemon_name }} supports macos and linux hosts only");

#[cfg(target_os = "macos")]
fn host_backend() -> Arc<dyn HostBackend> {
    Arc::new(device_platform_mac::MacHostBackend)
}

#[cfg(target_os = "linux")]
fn host_backend() -> Arc<dyn HostBackend> {
    Arc::new(device_platform_linux::LinuxHostBackend)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let ticks: usize = std::env::var("DAEMON_TICKS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);

    let sock = std::env::var("DAEMON_IPC_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::temp_dir().join("{{ cookiecutter.daemon_name }}.sock")
        });

    info!(
        daemon = "{{ cookiecutter.daemon_name }}",
        ticks,
        ipc = %sock.display(),
        "starting chassis"
    );

    let host = host_backend();
    let (desired_tx, desired_rx) = tokio::sync::watch::channel(DesiredState { enabled: true });

    let reconcile = ReconcileLoop::new(host, desired_rx, Duration::from_millis(200));
    reconcile.run_n(ticks).await?;

    if std::env::var("DAEMON_SKIP_IPC").ok().as_deref() != Some("1") {
        let server = LocalIpcServer::new(&sock);
        let desired = DesiredState { enabled: true };
        let path = sock.clone();
        let serve = tokio::spawn(async move { server.serve_once(desired).await });
        tokio::time::sleep(Duration::from_millis(50)).await;
        let resp = LocalIpcServer::request(&path, "health\n").await?;
        info!(%resp, "ipc response");
        serve.await??;
        let _ = std::fs::remove_file(&path);
    }

    drop(desired_tx);
    info!("chassis exit");
    Ok(())
}

//! Device daemon core: host backend trait, desired-state stub, local IPC listener.
//!
//! Product catalogs (MQTT gateway, storage classes, Runtime API schemas) live in
//! product ADRs — not here.

pub mod desired;
pub mod error;
pub mod host;
pub mod ipc;
pub mod reconcile;

pub use desired::{DesiredState, ObservedState};
pub use error::CoreError;
pub use host::HostBackend;
pub use ipc::LocalIpcServer;
pub use reconcile::ReconcileLoop;

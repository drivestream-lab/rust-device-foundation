use serde::{Deserialize, Serialize};

/// Chassis desired-state stub. Product schemas extend or replace this in ADRs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DesiredState {
    pub enabled: bool,
}

/// Observed local facts used by the reconcile loop.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservedState {
    pub hostname: String,
    pub free_disk_bytes: u64,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("host backend: {0}")]
    Host(String),

    #[error("ipc: {0}")]
    Ipc(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, CoreError>;

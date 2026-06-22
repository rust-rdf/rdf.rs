// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with an Oxigraph store.
#[derive(Debug, Error)]
pub enum OxigraphError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] oxigraph::store::StorageError),

    #[error("other error")]
    Other,
}

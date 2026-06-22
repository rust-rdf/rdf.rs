// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with a Virtuoso store.
#[derive(Debug, Error)]
pub enum VirtuosoError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] odbc_api::Error),

    #[error("other error")]
    Other,
}

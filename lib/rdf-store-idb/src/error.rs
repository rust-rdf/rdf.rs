// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with an IndexedDB store.
#[derive(Debug, Error)]
pub enum IdbError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] idb::Error),

    #[error("other error")]
    Other,
}

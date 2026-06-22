// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with a SQLite store.
#[derive(Debug, Error)]
pub enum SqliteError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] turso::Error),

    #[error("other error")]
    Other,
}

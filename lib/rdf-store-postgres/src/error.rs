// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with a PostgreSQL store.
#[derive(Debug, Error)]
pub enum PostgresError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] tokio_postgres::Error),

    #[error("other error")]
    Other,
}

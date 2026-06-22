// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with a Neo4j store.
#[derive(Debug, Error)]
pub enum Neo4jError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] neo4rs::Error),

    #[error("other error")]
    Other,
}

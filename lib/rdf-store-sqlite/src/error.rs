// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with a SQLite store.
#[derive(Debug, Error)]
pub enum SqliteError {
    /// This backend currently stores statements only in the default graph.
    #[error("SQLite named-graph storage is not implemented")]
    UnsupportedNamedGraph,

    /// The singleton was used as an RDF node rather than a graph selector.
    #[error("the default-graph marker is only valid in the graph slot")]
    InvalidDefaultGraphTerm,

    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] turso::Error),

    #[error("other error")]
    Other,
}

// This is free and unencumbered software released into the public domain.

use crate::MongoTripleId;
use rdf_model::TripleSlot;
use thiserror::Error;

/// An error when interacting with a MongoDB store.
#[derive(Clone, Debug, Error)]
pub enum MongoError {
    #[error("missing database name in mongodb:// URL")]
    NoDatabase,

    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] mongodb::error::Error),

    #[error("invalid triple: {0}")]
    InvalidTriple(MongoTripleId),

    #[error("invalid {1} term: {0}")]
    InvalidTripleTerm(MongoTripleId, TripleSlot),

    #[error("other error")]
    Other,
}

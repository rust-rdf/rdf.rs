// This is free and unencumbered software released into the public domain.

use crate::ValkeyTripleId;
use rdf_model::TripleSlot;
use thiserror::Error;

/// An error when interacting with a Valkey store.
#[derive(Error, Debug)]
pub enum ValkeyError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] fred::error::Error),

    #[error("invalid triple: {0}")]
    InvalidTriple(ValkeyTripleId),

    #[error("invalid {1} term: {0}")]
    InvalidTripleTerm(ValkeyTripleId, TripleSlot),

    #[error("other error")]
    Other,
}

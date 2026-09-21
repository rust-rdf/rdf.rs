// This is free and unencumbered software released into the public domain.

use crate::ValkeyTripleId;
use rdf_model::TripleSlot;
use thiserror::Error;

/// An error when interacting with a Valkey store.
#[derive(Clone, Debug, Error)]
pub enum ValkeyError {
    /// Matching across all graphs or ambiguous/unsupported graph names is not
    /// implemented. Use an explicit default graph or a supported IRI graph name.
    #[error(
        "Valkey graph selection requires an explicit default graph or an IRI name other than 'default'"
    )]
    UnsupportedGraphPattern,

    /// A pattern still contains wildcard slots required to produce a statement.
    #[error("cannot construct a statement from an unbound pattern")]
    UnboundPattern,

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

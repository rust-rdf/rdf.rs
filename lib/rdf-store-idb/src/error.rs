// This is free and unencumbered software released into the public domain.

use rdf_model::TripleSlot;
use thiserror::Error;

/// An error when interacting with an IndexedDB store.
#[derive(Error, Debug)]
pub enum IdbError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("server returned: {0}")]
    Server(#[from] idb::Error),

    #[error("other error")]
    Other,
}

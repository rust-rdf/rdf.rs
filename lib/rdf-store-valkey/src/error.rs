// This is free and unencumbered software released into the public domain.

use crate::ValkeyTripleKey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValkeyError {
    #[error("read-only transaction")]
    ReadOnly,
    #[error("server returned: {0}")]
    Server(#[from] fred::error::Error),
    #[error("invalid triple: {0}")]
    InvalidTriple(ValkeyTripleKey),
    #[error("invalid term: {0}")]
    InvalidTerm(ValkeyTripleKey),
    #[error("other error")]
    Other,
}

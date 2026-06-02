// This is free and unencumbered software released into the public domain.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValkeyError {
    #[error("read-only transaction")]
    ReadOnly,
    #[error("server returned: {0}")]
    Server(#[from] fred::error::Error),
    #[error("other error")]
    Other,
}

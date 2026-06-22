// This is free and unencumbered software released into the public domain.

use thiserror::Error;

/// An error when interacting with a QLever store.
#[derive(Clone, Debug, Error)]
pub enum QleverError {
    #[error("read-only transaction")]
    ReadOnly,

    #[error("other error")]
    Other,
}

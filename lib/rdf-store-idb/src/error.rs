// This is free and unencumbered software released into the public domain.

use core::{error::Error, fmt};
use send_wrapper::SendWrapper;

/// An error when interacting with an IndexedDB store.
///
/// Driver errors retain their original JavaScript value behind a thread-affinity
/// guard to satisfy the storage traits' `Send` bound. Formatting, inspecting, or
/// dropping such an error on a different thread from its origin panics.
#[derive(Debug)]
pub enum IdbError {
    /// A mutation was attempted through a read-only transaction.
    ReadOnly,

    /// An IndexedDB driver error, accessible only on its originating thread.
    Server(SendWrapper<idb::Error>),

    /// An otherwise unclassified failure.
    Other,
}

impl From<idb::Error> for IdbError {
    fn from(error: idb::Error) -> Self {
        Self::Server(SendWrapper::new(error))
    }
}

impl fmt::Display for IdbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadOnly => f.write_str("read-only transaction"),
            Self::Server(error) => write!(f, "server returned: {}", **error),
            Self::Other => f.write_str("other error"),
        }
    }
}

impl Error for IdbError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Server(error) => Some(&**error),
            _ => None,
        }
    }
}

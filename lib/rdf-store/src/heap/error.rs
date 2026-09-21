// This is free and unencumbered software released into the public domain.

/// An error when using a heap-store transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeapStoreError {
    /// A mutation, commit, or rollback was attempted on a read-only transaction.
    ReadOnly,
    /// The transaction has already committed, possibly through another handle.
    Committed,
    /// The transaction has already rolled back, possibly through another handle.
    RolledBack,
}

impl core::error::Error for HeapStoreError {}

impl core::fmt::Display for HeapStoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::ReadOnly => "read-only heap transaction",
            Self::Committed => "heap transaction already committed",
            Self::RolledBack => "heap transaction already rolled back",
        })
    }
}

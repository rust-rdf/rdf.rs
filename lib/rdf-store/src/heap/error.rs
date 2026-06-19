// This is free and unencumbered software released into the public domain.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeapStoreError;

impl core::error::Error for HeapStoreError {}

impl core::fmt::Display for HeapStoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HeapStoreError")
    }
}

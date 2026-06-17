// This is free and unencumbered software released into the public domain.

use crate::HdtWriterResult;
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the HDT binary format.
pub struct HdtWriter<T: AsyncWrite + Unpin> {
    sink: T,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for HdtWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HdtWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for HdtWriter<T> {
    /// Creates an HDT writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self { sink: input }
    }
}

impl<T: AsyncWrite + Unpin> HdtWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> HdtWriterResult<T> {
        Ok(self.sink)
    }
}

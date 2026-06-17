// This is free and unencumbered software released into the public domain.

use crate::CottasWriterResult;
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the COTTAS binary format.
pub struct CottasWriter<T: AsyncWrite + Unpin> {
    sink: T,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for CottasWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CottasWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for CottasWriter<T> {
    /// Creates a COTTAS writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self { sink: input }
    }
}

impl<T: AsyncWrite + Unpin> CottasWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> CottasWriterResult<T> {
        Ok(self.sink)
    }
}

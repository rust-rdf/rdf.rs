// This is free and unencumbered software released into the public domain.

use crate::NquadsWriterResult;
use oxttl::nquads::{NQuadsSerializer, TokioAsyncWriterNQuadsSerializer};
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the N-Quads text format.
pub struct NquadsWriter<T: AsyncWrite + Unpin> {
    pub serializer: TokioAsyncWriterNQuadsSerializer<T>,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for NquadsWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NquadsWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for NquadsWriter<T> {
    /// Creates an N-Quads writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self {
            serializer: NQuadsSerializer::new().for_tokio_async_writer(input),
        }
    }
}

impl<T: AsyncWrite + Unpin> NquadsWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> NquadsWriterResult<T> {
        Ok(self.serializer.finish())
    }
}

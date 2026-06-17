// This is free and unencumbered software released into the public domain.

use crate::NtriplesWriterResult;
use oxttl::ntriples::{NTriplesSerializer, TokioAsyncWriterNTriplesSerializer};
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the N-Triples text format.
pub struct NtriplesWriter<T: AsyncWrite + Unpin> {
    pub serializer: TokioAsyncWriterNTriplesSerializer<T>,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for NtriplesWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NtriplesWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for NtriplesWriter<T> {
    /// Creates an N-Triples writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self {
            serializer: NTriplesSerializer::new().for_tokio_async_writer(input),
        }
    }
}

impl<T: AsyncWrite + Unpin> NtriplesWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> NtriplesWriterResult<T> {
        Ok(self.serializer.finish())
    }
}

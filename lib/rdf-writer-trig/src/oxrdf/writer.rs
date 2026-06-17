// This is free and unencumbered software released into the public domain.

use crate::TrigWriterResult;
use oxttl::trig::{TokioAsyncWriterTriGSerializer, TriGSerializer};
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the TriG text format.
pub struct TrigWriter<T: AsyncWrite + Unpin> {
    pub serializer: TokioAsyncWriterTriGSerializer<T>,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for TrigWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TrigWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for TrigWriter<T> {
    /// Creates a TriG writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self {
            serializer: TriGSerializer::new().for_tokio_async_writer(input),
        }
    }
}

impl<T: AsyncWrite + Unpin> TrigWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> TrigWriterResult<T> {
        Ok(self.serializer.finish().await?)
    }
}

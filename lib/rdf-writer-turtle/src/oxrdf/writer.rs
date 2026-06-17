// This is free and unencumbered software released into the public domain.

use crate::TurtleWriterResult;
use oxttl::turtle::{TokioAsyncWriterTurtleSerializer, TurtleSerializer};
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the Turtle text format.
pub struct TurtleWriter<T: AsyncWrite + Unpin> {
    pub serializer: TokioAsyncWriterTurtleSerializer<T>,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for TurtleWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TurtleWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for TurtleWriter<T> {
    /// Creates a Turtle writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self {
            serializer: TurtleSerializer::new().for_tokio_async_writer(input),
        }
    }
}

impl<T: AsyncWrite + Unpin> TurtleWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> TurtleWriterResult<T> {
        Ok(self.serializer.finish().await?)
    }
}

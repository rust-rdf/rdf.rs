// This is free and unencumbered software released into the public domain.

use crate::JellyWriterResult;
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the Jelly binary format.
pub struct JellyWriter<T: AsyncWrite + Unpin> {
    sink: T,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for JellyWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "JellyWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for JellyWriter<T> {
    /// Creates a Jelly writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self { sink: input }
    }
}

impl<T: AsyncWrite + Unpin> JellyWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> JellyWriterResult<T> {
        Ok(self.sink)
    }
}

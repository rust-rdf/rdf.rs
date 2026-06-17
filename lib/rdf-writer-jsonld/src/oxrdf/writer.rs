// This is free and unencumbered software released into the public domain.

use crate::JsonldWriterResult;
use oxjsonld::{JsonLdSerializer, TokioAsyncWriterJsonLdSerializer};
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the JSON-LD text format.
pub struct JsonldWriter<T: AsyncWrite + Unpin> {
    pub serializer: TokioAsyncWriterJsonLdSerializer<T>,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for JsonldWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "JsonldWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for JsonldWriter<T> {
    /// Creates a JSON-LD writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self {
            serializer: JsonLdSerializer::new().for_tokio_async_writer(input),
        }
    }
}

impl<T: AsyncWrite + Unpin> JsonldWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> JsonldWriterResult<T> {
        Ok(self.serializer.finish().await?)
    }
}

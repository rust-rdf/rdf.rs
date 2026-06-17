// This is free and unencumbered software released into the public domain.

use crate::RdfxmlWriterResult;
use oxrdfxml::{RdfXmlSerializer, TokioAsyncWriterRdfXmlSerializer};
use tokio::io::AsyncWrite;

/// A writer (aka serializer) for the RDF/XML text format.
pub struct RdfxmlWriter<T: AsyncWrite + Unpin> {
    pub serializer: TokioAsyncWriterRdfXmlSerializer<T>,
}

impl<T: AsyncWrite + Unpin> core::fmt::Debug for RdfxmlWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RdfxmlWriter")
    }
}

impl<T: AsyncWrite + Unpin> From<T> for RdfxmlWriter<T> {
    /// Creates an RDF/XML writer for a Tokio `AsyncWrite` sink.
    fn from(input: T) -> Self {
        Self {
            serializer: RdfXmlSerializer::new().for_tokio_async_writer(input),
        }
    }
}

impl<T: AsyncWrite + Unpin> RdfxmlWriter<T> {
    /// Finishes writing output and returns the underlying sink.
    pub async fn finish(self) -> RdfxmlWriterResult<T> {
        Ok(self.serializer.finish().await?)
    }
}

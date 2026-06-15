// This is free and unencumbered software released into the public domain.

use crate::{RdfxmlReaderResult, RdfxmlTriple};
use alloc::boxed::Box;
use futures::Stream;
use oxrdfxml::{RdfXmlParser, TokioAsyncReaderRdfXmlParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the RDF/XML text format.
pub struct RdfxmlReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderRdfXmlParser<T>,
    #[allow(dead_code)]
    pub(crate) handle: Handle,
}

impl<T: AsyncRead + Unpin + Send + 'static> From<T> for RdfxmlReader<T> {
    /// Creates an RDF/XML reader for an `AsyncRead` source.
    fn from(input: T) -> Self {
        Self {
            parser: RdfXmlParser::new().for_tokio_async_reader(input),
            handle: Handle::current(),
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> RdfxmlReader<T> {
    pub fn into_stream(mut self) -> impl Stream<Item = RdfxmlReaderResult<RdfxmlTriple>> {
        async_stream::stream! {
            while let Some(input) = self.parser.next().await {
                yield match input {
                    Ok(triple) => Ok(triple.into()),
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = RdfxmlReaderResult<RdfxmlTriple>> {
        let handle = self.handle.clone();
        let stream = Box::pin(self.into_stream());
        StreamIter::new(stream, handle)
    }
}

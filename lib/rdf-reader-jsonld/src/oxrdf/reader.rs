// This is free and unencumbered software released into the public domain.

use crate::{JsonldReaderResult, JsonldTriple};
use alloc::boxed::Box;
use futures::Stream;
use oxjsonld::{JsonLdParser, TokioAsyncReaderJsonLdParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the JSON-LD text format.
pub struct JsonldReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderJsonLdParser<T>,
    #[allow(dead_code)]
    pub(crate) handle: Handle,
}

impl<T: AsyncRead + Unpin + Send + 'static> From<T> for JsonldReader<T> {
    /// Creates a JSON-LD reader for an `AsyncRead` source.
    fn from(input: T) -> Self {
        Self {
            parser: JsonLdParser::new().for_tokio_async_reader(input),
            handle: Handle::current(),
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> JsonldReader<T> {
    pub fn into_stream(mut self) -> impl Stream<Item = JsonldReaderResult<JsonldTriple>> {
        async_stream::stream! {
            while let Some(input) = self.parser.next().await {
                yield match input {
                    Ok(triple) => Ok(triple.into()),
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = JsonldReaderResult<JsonldTriple>> {
        let handle = self.handle.clone();
        let stream = Box::pin(self.into_stream());
        StreamIter::new(stream, handle)
    }
}

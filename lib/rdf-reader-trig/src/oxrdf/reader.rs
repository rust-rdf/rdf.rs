// This is free and unencumbered software released into the public domain.

use super::{TrigQuad, TrigReaderResult};
use futures::Stream;
use oxttl::trig::{TokioAsyncReaderTriGParser, TriGParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the TriG text format.
pub struct TrigReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderTriGParser<T>,
    pub(crate) handle: Handle,
}

impl<T: AsyncRead + Unpin + Send + 'static> From<T> for TrigReader<T> {
    /// Creates a TriG reader for an `AsyncRead` source.
    fn from(input: T) -> Self {
        Self {
            parser: TriGParser::new().for_tokio_async_reader(input),
            handle: Handle::current(),
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> TrigReader<T> {
    pub fn into_stream(mut self) -> impl Stream<Item = TrigReaderResult<TrigQuad>> {
        async_stream::stream! {
            while let Some(input) = self.parser.next().await {
                yield match input {
                    Ok(quad) => Ok(quad.into()),
                    Err(err) => Err(err),
                }
            }
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> IntoIterator for TrigReader<T> {
    type Item = TrigReaderResult<TrigQuad>;
    type IntoIter = StreamIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let handle = self.handle.clone();
        StreamIter::new(self.into_stream(), handle)
    }
}

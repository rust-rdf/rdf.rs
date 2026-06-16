// This is free and unencumbered software released into the public domain.

use crate::{NquadsQuad, NquadsReaderResult};
use futures::Stream;
use oxttl::nquads::{NQuadsParser, TokioAsyncReaderNQuadsParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the N-Quads text format.
pub struct NquadsReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderNQuadsParser<T>,
    pub(crate) handle: Handle,
}

impl<T: AsyncRead + Unpin + Send + 'static> From<T> for NquadsReader<T> {
    /// Creates an N-Quads reader for an `AsyncRead` source.
    fn from(input: T) -> Self {
        Self {
            parser: NQuadsParser::new().for_tokio_async_reader(input),
            handle: Handle::current(),
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> NquadsReader<T> {
    pub fn into_stream(mut self) -> impl Stream<Item = NquadsReaderResult<NquadsQuad>> {
        async_stream::stream! {
            while let Some(input) = self.parser.next().await {
                yield match input {
                    Ok(triple) => Ok(triple.into()),
                    Err(err) => Err(err),
                }
            }
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> IntoIterator for NquadsReader<T> {
    type Item = NquadsReaderResult<NquadsQuad>;
    type IntoIter = StreamIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let handle = self.handle.clone();
        StreamIter::new(self.into_stream(), handle)
    }
}

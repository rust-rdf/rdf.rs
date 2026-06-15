// This is free and unencumbered software released into the public domain.

use crate::{NquadsQuad, NquadsReaderResult};
use alloc::boxed::Box;
use futures::Stream;
use oxttl::nquads::{NQuadsParser, TokioAsyncReaderNQuadsParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the N-Quads text format.
pub struct NquadsReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderNQuadsParser<T>,
    #[allow(dead_code)]
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

    pub fn into_iter(self) -> impl Iterator<Item = NquadsReaderResult<NquadsQuad>> {
        let handle = self.handle.clone();
        let stream = Box::pin(self.into_stream());
        StreamIter::new(stream, handle)
    }
}

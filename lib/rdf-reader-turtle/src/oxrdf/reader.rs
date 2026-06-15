// This is free and unencumbered software released into the public domain.

use super::{TurtleReaderResult, TurtleTriple};
use alloc::boxed::Box;
use futures::Stream;
use oxttl::turtle::{TokioAsyncReaderTurtleParser, TurtleParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the Turtle text format.
pub struct TurtleReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderTurtleParser<T>,
    #[allow(dead_code)]
    pub(crate) handle: Handle,
}

impl<T: AsyncRead + Unpin + Send + 'static> From<T> for TurtleReader<T> {
    /// Creates a Turtle reader for an `AsyncRead` source.
    fn from(input: T) -> Self {
        Self {
            parser: TurtleParser::new().for_tokio_async_reader(input),
            handle: Handle::current(),
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> TurtleReader<T> {
    pub fn into_stream(mut self) -> impl Stream<Item = TurtleReaderResult<TurtleTriple>> {
        async_stream::stream! {
            while let Some(input) = self.parser.next().await {
                yield match input {
                    Ok(triple) => Ok(triple.into()),
                    Err(err) => Err(err),
                }
            }
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = TurtleReaderResult<TurtleTriple>> {
        let handle = self.handle.clone();
        let stream = Box::pin(self.into_stream());
        StreamIter::new(stream, handle)
    }
}

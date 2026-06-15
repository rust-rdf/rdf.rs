// This is free and unencumbered software released into the public domain.

use crate::{TurtleReaderResult, TurtleTriple};
use futures::Stream;
use oxttl::turtle::{TokioAsyncReaderTurtleParser, TurtleParser};
use tokio::io::AsyncRead;

/// A reader for the Turtle text format.
pub struct TurtleReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderTurtleParser<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> TurtleReader<T> {
    pub async fn open(input: T) -> TurtleReaderResult<Self> {
        let parser = TurtleParser::new().for_tokio_async_reader(input);
        Ok(Self { parser })
    }

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
}

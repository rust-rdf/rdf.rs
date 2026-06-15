// This is free and unencumbered software released into the public domain.

use crate::{TrigReaderResult, TrigTriple};
use futures::Stream;
use oxttl::trig::{TokioAsyncReaderTriGParser, TriGParser};
use tokio::io::AsyncRead;

pub struct TrigReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderTriGParser<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> TrigReader<T> {
    pub async fn open(input: T) -> TrigReaderResult<Self> {
        let parser = TriGParser::new().for_tokio_async_reader(input);
        Ok(Self { parser })
    }

    pub fn into_stream(mut self) -> impl Stream<Item = TrigReaderResult<TrigTriple>> {
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

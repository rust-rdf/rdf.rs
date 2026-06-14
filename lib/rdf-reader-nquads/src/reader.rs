// This is free and unencumbered software released into the public domain.

use crate::{NquadsQuad, NquadsReaderResult};
use futures::Stream;
use oxttl::ntriples::{NTriplesParser, TokioAsyncReaderNTriplesParser};
use tokio::io::AsyncRead;

pub struct NquadsReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderNTriplesParser<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> NquadsReader<T> {
    pub async fn open(input: T) -> NquadsReaderResult<Self> {
        let parser = NTriplesParser::new().for_tokio_async_reader(input);
        Ok(Self { parser })
    }

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

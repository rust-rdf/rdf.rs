// This is free and unencumbered software released into the public domain.

use crate::{NtriplesReaderResult, NtriplesTriple};
use futures::Stream;
use oxttl::ntriples::{NTriplesParser, TokioAsyncReaderNTriplesParser};
use tokio::io::AsyncRead;

pub struct NtriplesReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderNTriplesParser<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> NtriplesReader<T> {
    pub async fn open(input: T) -> NtriplesReaderResult<Self> {
        let parser = NTriplesParser::new().for_tokio_async_reader(input);
        Ok(Self { parser })
    }

    pub fn into_stream(mut self) -> impl Stream<Item = NtriplesReaderResult<NtriplesTriple>> {
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

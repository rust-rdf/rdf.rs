// This is free and unencumbered software released into the public domain.

use crate::{JsonldReaderResult, JsonldTriple};
use futures::Stream;
use oxjsonld::{JsonLdParser, TokioAsyncReaderJsonLdParser};
use tokio::io::AsyncRead;

/// A reader for the JSON-LD text format.
pub struct JsonldReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderJsonLdParser<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> JsonldReader<T> {
    pub async fn open(input: T) -> JsonldReaderResult<Self> {
        let parser = JsonLdParser::new().for_tokio_async_reader(input);
        Ok(Self { parser })
    }

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
}

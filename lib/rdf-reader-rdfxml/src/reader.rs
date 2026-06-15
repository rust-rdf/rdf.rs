// This is free and unencumbered software released into the public domain.

use crate::{RdfxmlReaderResult, RdfxmlTriple};
use futures::Stream;
use oxrdfxml::{RdfXmlParser, TokioAsyncReaderRdfXmlParser};
use tokio::io::AsyncRead;

pub struct RdfxmlReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderRdfXmlParser<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> RdfxmlReader<T> {
    pub async fn open(input: T) -> RdfxmlReaderResult<Self> {
        let parser = RdfXmlParser::new().for_tokio_async_reader(input);
        Ok(Self { parser })
    }

    pub fn into_stream(mut self) -> impl Stream<Item = RdfxmlReaderResult<RdfxmlTriple>> {
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

// This is free and unencumbered software released into the public domain.

use super::{NtriplesReaderResult, NtriplesTriple};
use futures::Stream;
use oxttl::ntriples::{NTriplesParser, TokioAsyncReaderNTriplesParser};
use rdf_reader::StreamIter;
use tokio::{io::AsyncRead, runtime::Handle};

/// A reader for the N-Triples text format.
pub struct NtriplesReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: TokioAsyncReaderNTriplesParser<T>,
    #[allow(dead_code)]
    pub(crate) handle: Handle,
}

impl<T: AsyncRead + Unpin + Send + 'static> From<T> for NtriplesReader<T> {
    /// Creates an N-Triples reader for an `AsyncRead` source.
    fn from(input: T) -> Self {
        Self {
            parser: NTriplesParser::new().for_tokio_async_reader(input),
            handle: Handle::current(),
        }
    }
}

impl<T: AsyncRead + Unpin + Send + 'static> NtriplesReader<T> {
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

impl<T: AsyncRead + Unpin + Send + 'static> IntoIterator for NtriplesReader<T> {
    type Item = NtriplesReaderResult<NtriplesTriple>;
    type IntoIter = StreamIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let handle = self.handle.clone();
        StreamIter::new(self.into_stream(), handle)
    }
}

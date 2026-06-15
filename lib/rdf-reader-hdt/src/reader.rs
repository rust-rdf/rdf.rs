// This is free and unencumbered software released into the public domain.

use crate::{HdtReaderResult, HdtTriple};
use core::marker::PhantomData;
use futures::Stream;
use hdt::{Hdt, sophia::api::graph::Graph};
use std::io::BufReader;
use tokio::io::AsyncRead;
use tokio_util::io::SyncIoBridge;

/// A reader for the HDT binary format.
pub struct HdtReader<T: AsyncRead + Unpin + Send + 'static> {
    pub(crate) parser: Hdt,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: AsyncRead + Unpin + Send + 'static> HdtReader<T> {
    pub async fn open(input: T) -> HdtReaderResult<Self> {
        let input = BufReader::new(SyncIoBridge::new(input));
        let parser = tokio::task::spawn_blocking(move || Hdt::read(input))
            .await
            .unwrap()?; // TODO
        Ok(Self {
            parser,
            phantom: PhantomData,
        })
    }

    pub fn into_stream(self) -> impl Stream<Item = HdtReaderResult<HdtTriple>> {
        async_stream::stream! {
            let triples = self.parser.triples();
            for triple in triples {
                let triple = triple.unwrap(); // TODO
                yield Ok(triple.into());
            }
        }
    }
}

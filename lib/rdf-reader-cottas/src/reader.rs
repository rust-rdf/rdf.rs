// This is free and unencumbered software released into the public domain.

use crate::CottasReaderResult;
use arrow_array::StringArray;
use arrow_schema::DataType;
use core::{pin::Pin, task::Poll};
use futures::{Stream, StreamExt, task::Context};
use parquet::{
    arrow::{
        ParquetRecordBatchStreamBuilder,
        async_reader::{AsyncFileReader, ParquetRecordBatchStream},
    },
    errors::Result,
};
use rdf_model::{HeapTerm, HeapTriple, TRIPLE_SLOTS};
use tokio::fs::File;

/// A reader for the COTTAS (Parquet) binary format.
pub struct CottasReader<T: AsyncFileReader + Send + 'static = File> {
    pub stream: ParquetRecordBatchStream<T>,
    pub batch_row_index: usize,
    pub batch: Option<arrow_array::RecordBatch>,
}

impl<T: AsyncFileReader + Unpin + Send + 'static> CottasReader<T> {
    /// Creates a COTTAS reader for an `AsyncRead + AsyncSeek` source.
    pub async fn try_from(input: T) -> Result<Self> {
        let builder = ParquetRecordBatchStreamBuilder::new(input).await?;
        let stream = builder.build()?;
        Ok(Self {
            stream,
            batch_row_index: 0,
            batch: None,
        })
    }

    pub fn into_stream(self) -> impl Stream<Item = CottasReaderResult<HeapTriple>> {
        self
    }

    fn next_in_batch(&mut self) -> Option<CottasReaderResult<HeapTriple>> {
        let Some(batch) = self.batch.as_ref() else {
            return None; // no current batch
        };

        if self.batch_row_index >= batch.num_rows() {
            self.batch = None;
            self.batch_row_index = 0;
            return None; // current batch exhausted
        }

        let schema = batch.schema_ref();
        let triple = TRIPLE_SLOTS
            .iter()
            .map(|slot| {
                let column_index = slot.index();
                let column_type = schema.field(column_index).data_type();
                let column = batch.column(column_index);
                match column_type {
                    DataType::Utf8 => {
                        let column_array = column.as_any().downcast_ref::<StringArray>().unwrap();
                        let column_str = column_array.value(self.batch_row_index);
                        HeapTerm::iri(&column_str[1..column_str.len() - 1]) // TODO
                    },
                    _ => unreachable!(),
                }
            })
            .collect::<HeapTriple>();

        self.batch_row_index += 1;
        Some(Ok(triple))
    }
}

impl<T: AsyncFileReader + Unpin + Send + 'static> Stream for CottasReader<T> {
    type Item = CottasReaderResult<HeapTriple>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Some(result) = self.next_in_batch() {
            return Poll::Ready(Some(result));
        }
        match self.stream.poll_next_unpin(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None), // EOS
            Poll::Ready(Some(Err(_err))) => Poll::Ready(Some(Err(()))), // TODO
            Poll::Ready(Some(Ok(batch))) => {
                self.batch = Some(batch);
                Poll::Ready(self.next_in_batch())
            },
        }
    }
}

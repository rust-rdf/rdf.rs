// This is free and unencumbered software released into the public domain.

use crate::{HeapStoreError, HeapTransaction, Store};
use alloc::sync::Arc;
use rdf_model::HeapQuadSet;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct HeapStore {
    pub(crate) quads: RwLock<HeapQuadSet>,
}

impl HeapStore {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

impl Store for Arc<HeapStore> {
    type Error = HeapStoreError;
    type Read = Arc<HeapTransaction>;
    type Write = Arc<HeapTransaction>;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        Ok(Arc::new(HeapTransaction::new(self.clone(), false)))
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        Ok(Arc::new(HeapTransaction::new(self.clone(), true)))
    }
}

// This is free and unencumbered software released into the public domain.

use crate::{HeapTransaction, Store};
use alloc::{boxed::Box, collections::BTreeSet, sync::Arc};
use async_trait::async_trait;
use parking_lot::RwLock;
use rdf_model::HeapQuad;

#[derive(Debug, Default)]
pub struct HeapStore {
    pub(crate) quads: RwLock<BTreeSet<HeapQuad>>,
}

impl HeapStore {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

#[async_trait]
impl Store for Arc<HeapStore> {
    type Error = ();
    type Read = Arc<HeapTransaction>;
    type Write = Arc<HeapTransaction>;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        Ok(Arc::new(HeapTransaction::new(self.clone(), false)))
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        Ok(Arc::new(HeapTransaction::new(self.clone(), true)))
    }
}

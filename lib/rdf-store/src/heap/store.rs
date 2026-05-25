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
    type Transaction = Arc<HeapTransaction>;

    async fn begin_transaction(&mut self) -> Result<Self::Transaction, Self::Error> {
        Ok(Arc::new(HeapTransaction::new(self.clone()))) // TODO
    }
}

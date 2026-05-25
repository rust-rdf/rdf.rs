// This is free and unencumbered software released into the public domain.

use crate::{HeapTransaction, Store};
use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use async_trait::async_trait;
use rdf_model::HeapQuad;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HeapStore {
    quads: BTreeSet<HeapQuad>,
}

impl HeapStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Store for HeapStore {
    type Error = ();
    type Transaction = HeapTransaction;

    async fn begin_transaction(&mut self) -> Result<Self::Transaction, Self::Error> {
        Ok(HeapTransaction::new()) // TODO
    }
}

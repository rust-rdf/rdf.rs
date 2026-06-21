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

/// # Cancel safety
///
/// - `read` / `write`: creation is immediate and cancel safe (they construct an
///   `Arc<HeapTransaction>` synchronously and return `Ok`).
/// - `ReadTransaction` methods: all read methods observe in-memory state and are
///   cancel safe. Dropping a returned stream/future must not mutate the store.
/// - `WriteTransaction` methods (`insert`, `remove`, `clear`, `delete`): these
///   operate on a per-transaction in-memory mutation map. They are effectively
///   cancel safe: if a caller cancels while a mutating method is waiting for the
///   transaction's lock, no store mutation occurs; once the method completes it
///   only updates the transaction-local mutation set.
/// - `commit`: not cancel safe. `commit` applies the transaction's mutation map
///   to the store by acquiring locks and updating the shared `quads` set. If the
///   commit future is canceled while applying changes, the store may be left in
///   a partially-applied state. Callers that require atomic application MUST
///   drive `commit` to completion.
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

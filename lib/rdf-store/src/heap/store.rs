// This is free and unencumbered software released into the public domain.

use crate::{HeapStoreError, HeapTransaction, Store};
use alloc::sync::Arc;
use rdf_model::HeapQuadSet;
use tokio::sync::RwLock;

/// An in-memory set of RDF quads, available with the `std` feature.
///
/// Use the [`Store`] implementation on [`Arc<HeapStore>`] to create transactions.
/// Reads observe current committed data plus their own staged changes, rather
/// than a snapshot taken when the transaction starts. Commits apply atomically
/// under a store-wide write lock. See [`HeapTransaction`] for isolation,
/// streaming, and cancellation semantics.
///
/// # Examples
///
/// ```rust
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use rdf_model::SAMPLE_QUAD;
/// use rdf_store::{HeapStore, ReadTransaction, Store, WriteTransaction};
///
/// let mut store = HeapStore::new();
/// let mut tx = store.write().await?;
/// tx.insert(SAMPLE_QUAD).await?;
/// assert!(tx.contains(()).await?);
/// tx.commit().await?;
/// assert_eq!(store.read().await?.count(()).await?, 1);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct HeapStore {
    pub(crate) quads: RwLock<HeapQuadSet>,
}

impl HeapStore {
    /// Creates an empty, shared heap store.
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

/// # Cancel safety
///
/// - `read` / `write`: creation is immediate and cancel safe (they construct an
///   `Arc<HeapTransaction>` synchronously and return `Ok`).
/// - Reads release their locks when their futures/streams finish or are dropped.
/// - Mutations are staged privately and never change committed data.
/// - `commit` can be canceled while waiting for locks without applying changes.
///   Once it wins transaction finalization, it applies all changes without an
///   intervening await, so future cancellation cannot leave a partial commit.
/// - A successful rollback request closes the shared transaction immediately.
///   Its future releases staged buffers after acquiring the transaction lock;
///   dropping that future cannot allow another handle to commit aborted changes.
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

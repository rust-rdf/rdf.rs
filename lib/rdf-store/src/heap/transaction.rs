// This is free and unencumbered software released into the public domain.

use crate::{HeapStore, HeapStoreError, ReadTransaction, WriteTransaction};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use core::sync::atomic::{AtomicU8, Ordering};
use futures::Stream;
use rdf_model::{HeapQuad, HeapQuadPattern, HeapQuadSet, HeapTerm, StatementPattern};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

const ACTIVE: u8 = 0;
const COMMITTED: u8 = 1;
const ROLLED_BACK: u8 = 2;

#[derive(Debug, Default)]
struct Changes {
    cleared: bool,
    mutations: BTreeMap<HeapQuad, bool>,
}

impl Changes {
    fn visible<'a>(&'a self, quads: &'a HeapQuadSet) -> impl Iterator<Item = &'a HeapQuad> {
        quads
            .iter()
            .take(if self.cleared { 0 } else { quads.len() })
            .filter(|quad| !self.mutations.contains_key(*quad))
            .chain(
                self.mutations
                    .iter()
                    .filter_map(|(quad, &insert)| insert.then_some(quad)),
            )
    }
}

/// A shared heap-store transaction with a private set of staged changes.
///
/// The transaction traits are implemented on [`Arc<HeapTransaction>`]. Cloned
/// handles share both changes and lifecycle: successful commit or rollback is
/// terminal for every handle. Subsequent operations return
/// [`HeapStoreError::Committed`] or [`HeapStoreError::RolledBack`]. Dropping the
/// last handle discards any uncommitted changes. The default value is a read-only
/// transaction on a new, empty store.
///
/// # Isolation and mutation order
///
/// Reads use current committed data overlaid with this transaction's changes;
/// they are not repeatable-read snapshots. Each quad appears at most once.
/// Insert/remove use the last operation on that quad. `delete` removes matching
/// committed quads and staged inserts visible when it runs, not future inserts
/// by other transactions. `clear` discards previous staged changes and hides all
/// committed data. At commit it clears the store, including intervening commits,
/// then applies operations staged after the last clear. Concurrent transactions
/// otherwise merge their changes, with the last commit winning for each quad.
///
/// Matching compares complete [`HeapTerm`] values. Every `None` pattern slot is
/// a wildcard, including the context slot; a quad's `None` context is the default
/// graph. Context enumeration returns distinct graph names of visible quads and
/// excludes the default graph.
///
/// # Locks and cancellation
///
/// Locks are acquired in transaction-then-store order. Matching and context
/// streams acquire read locks on first poll and hold them across yields for a
/// consistent view. Exhaust or drop those streams before awaiting mutations or
/// transaction completion that needs their locks. Native `contains` and `count`
/// inspect borrowed quads without creating a matching stream or cloning results;
/// `contains` stops at the first match.
///
/// Commit applies all changes under the store write lock, with no await after
/// claiming finalization. Canceling while waiting for locks applies no changes
/// and does not itself close the transaction. Commit and rollback race to
/// finalize the shared transaction; only one succeeds.
///
/// A successful rollback request closes the transaction immediately, even if
/// its returned future is never polled or is canceled. Awaiting that future also
/// clears the staged buffers. Streams suspended at a yield report rollback when
/// resumed and end; operations already executing may finish against their earlier
/// view.
#[derive(Debug, Default)]
pub struct HeapTransaction {
    changes: RwLock<Changes>,
    status: AtomicU8,
    store: Arc<HeapStore>,
    writable: bool,
}

impl HeapTransaction {
    /// Creates an active transaction on `store`.
    ///
    /// Set `writable` to enable staging, commit, and rollback. Wrap the result in
    /// [`Arc`] to use the transaction traits, or use [`Store`](crate::Store) to
    /// create an already-wrapped transaction. Construction takes no snapshot.
    pub fn new(store: Arc<HeapStore>, writable: bool) -> Self {
        Self {
            changes: RwLock::new(Changes::default()),
            status: AtomicU8::new(ACTIVE),
            store,
            writable,
        }
    }

    fn ensure_active(&self) -> Result<(), HeapStoreError> {
        match self.status.load(Ordering::Acquire) {
            ACTIVE => Ok(()),
            COMMITTED => Err(HeapStoreError::Committed),
            _ => Err(HeapStoreError::RolledBack),
        }
    }

    fn ensure_writable(&self) -> Result<(), HeapStoreError> {
        if !self.writable {
            return Err(HeapStoreError::ReadOnly);
        }
        self.ensure_active()
    }

    fn finalize(&self, status: u8) -> Result<(), HeapStoreError> {
        self.ensure_writable()?;
        self.status
            .compare_exchange(ACTIVE, status, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|previous| match previous {
                COMMITTED => HeapStoreError::Committed,
                _ => HeapStoreError::RolledBack,
            })
    }

    async fn write_changes(&self) -> Result<RwLockWriteGuard<'_, Changes>, HeapStoreError> {
        self.ensure_writable()?;
        let changes = self.changes.write().await;
        self.ensure_active()?;
        Ok(changes)
    }

    async fn read_view(
        &self,
    ) -> Result<
        (
            RwLockReadGuard<'_, Changes>,
            RwLockReadGuard<'_, HeapQuadSet>,
        ),
        HeapStoreError,
    > {
        self.ensure_active()?;
        let changes = self.changes.read().await;
        self.ensure_active()?;
        let quads = self.store.quads.read().await;
        self.ensure_active()?;
        Ok((changes, quads))
    }
}

// The generic StatementPattern::matches currently compares only lexical values
// and unwraps a missing context. The heap view needs complete term equality.
fn matches(quad: &HeapQuad, pattern: &HeapQuadPattern) -> bool {
    pattern.subject().is_none_or(|term| term == quad.subject())
        && pattern
            .predicate()
            .is_none_or(|term| term == quad.predicate())
        && pattern.object().is_none_or(|term| term == quad.object())
        && pattern
            .context()
            .is_none_or(|term| Some(term) == quad.context())
}

impl WriteTransaction for Arc<HeapTransaction> {
    type Error = HeapStoreError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>> {
        // Close before constructing the future: cancellation must not permit a
        // surviving Arc handle to commit the supposedly discarded mutations.
        let result = self.finalize(ROLLED_BACK);
        async move {
            result?;
            let mut changes = self.changes.write().await;
            changes.mutations.clear();
            changes.cleared = false;
            Ok(())
        }
    }

    async fn commit(self) -> Result<(), Self::Error> {
        let mut changes = self.write_changes().await?;
        let mut quads = self.store.quads.write().await;
        // Rollback can win while we wait for either lock. Once this succeeds,
        // no await may separate finalization from the complete store update.
        self.finalize(COMMITTED)?;
        if changes.cleared {
            quads.clear();
        }
        for (quad, insert) in core::mem::take(&mut changes.mutations) {
            if insert {
                quads.insert(quad);
            } else {
                quads.remove(&quad);
            }
        }
        changes.cleared = false;
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Self::Error> {
        let mut changes = self.write_changes().await?;
        changes.mutations.clear();
        changes.cleared = true;
        Ok(())
    }

    async fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        let quad = statement.into();
        self.write_changes().await?.mutations.insert(quad, true);
        Ok(())
    }

    async fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        let quad = statement.into();
        self.write_changes().await?.mutations.insert(quad, false);
        Ok(())
    }

    async fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error> {
        let pattern = pattern.into();
        let mut changes = self.write_changes().await?;
        if !changes.cleared {
            let quads = self.store.quads.read().await;
            self.ensure_active()?;
            for quad in quads.iter().filter(|quad| matches(quad, &pattern)) {
                changes.mutations.insert(quad.clone(), false);
            }
        }
        for (quad, insert) in changes.mutations.iter_mut() {
            if *insert && matches(quad, &pattern) {
                *insert = false;
            }
        }
        Ok(())
    }
}

impl ReadTransaction for Arc<HeapTransaction> {
    type Error = HeapStoreError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    async fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<bool, Self::Error> {
        let pattern = pattern.into();
        let (changes, quads) = self.read_view().await?;
        Ok(changes.visible(&quads).any(|quad| matches(quad, &pattern)))
    }

    async fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<u64, Self::Error> {
        let pattern = pattern.into();
        let (changes, quads) = self.read_view().await?;
        if pattern.is_empty() && changes.mutations.is_empty() {
            return Ok(if changes.cleared {
                0
            } else {
                quads.len() as u64
            });
        }
        Ok(changes
            .visible(&quads)
            .filter(|quad| matches(quad, &pattern))
            .count() as u64)
    }

    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> + Send {
        async_stream::try_stream! {
            let (changes, quads) = self.read_view().await?;
            let mut seen = BTreeSet::new();
            for quad in changes.visible(&quads) {
                self.ensure_active()?;
                if let Some(context) = quad.context() {
                    if seen.insert(context) {
                        yield context.clone();
                        self.ensure_active()?;
                    }
                }
            }
        }
    }

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        let pattern = pattern.into();
        async_stream::try_stream! {
            let (changes, quads) = self.read_view().await?;
            for quad in changes.visible(&quads) {
                self.ensure_active()?;
                if matches(quad, &pattern) {
                    yield quad.clone();
                    self.ensure_active()?;
                }
            }
        }
    }
}

// This is free and unencumbered software released into the public domain.

use core::{fmt::Debug, hash::Hash};
use rdf_model::{HeapTerm, QuadPattern, Statement, StatementPattern, Term};

/// A read-write (R/W) transaction on a [`Store`].
///
/// # Semantics for implementors
///
/// - Transactions present a private mutation view: mutating methods (`clear`,
///   `insert`, `remove`, `delete`) should record changes in the transaction's
///   local state and must not mutate the underlying store until `commit` is
///   driven to completion.
/// - `commit(self)` applies the transaction's recorded mutations to the store.
///   Implementations **should** document whether `commit` is atomic. If
///   atomicity cannot be guaranteed, callers must ensure the commit future is
///   completed to avoid partial application.
/// - `rollback(self)` discards any recorded mutations and releases any
///   resources held by the transaction. Dropping or canceling the future
///   returned by `rollback` must not leave the underlying store in a mutated
///   state.
/// - Implementors should take care to avoid deadlocks when acquiring internal
///   locks; prefer a consistent lock acquisition order across methods.
///
pub trait WriteTransaction {
    type Error: Debug + Send;
    type Term: Term + Clone + PartialEq + Eq + Hash + Send + From<HeapTerm>;
    type Statement: Statement<Term = Self::Term> + Send;
    type StatementPattern: StatementPattern<Term = Self::Term>
        + From<QuadPattern<Self::Term>>
        + Send;

    /// Aborts the transaction, discarding any changes.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Canceling or dropping the returned future will not
    /// apply the transaction's changes to the underlying store. Because the method
    /// consumes the transaction, dropping the future also drops the transaction value
    /// and any uncommitted changes; callers can therefore rely on cancellation to
    /// prevent changes from being persisted. Implementations should ensure any
    /// necessary cleanup or resource release also occurs if the future is dropped.
    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Commits the transaction, applying all changes.
    ///
    /// # Cancel safety
    ///
    /// This method is **not** cancel safe in the general case. If the commit
    /// future is canceled or dropped before it completes, some changes may
    /// already have been applied to the underlying store while others have
    /// not, resulting in a partially-applied commit. Callers that require the
    /// transaction to be atomically applied **must** drive the returned future
    /// to completion. Implementors that can guarantee atomicity should
    /// document that guarantee in their concrete type's documentation.
    fn commit(self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Clears all data from the store.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. It only records the clear operation in the
    /// transaction's local state (the transaction view) and does not modify the
    /// underlying store until `commit` completes. Canceling the returned future may
    /// leave the transaction either in the cleared state or in its previous state,
    /// but it will never corrupt the underlying store.
    fn clear(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Inserts a statement into the store.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The insertion is recorded in the transaction's
    /// local mutation set and is not applied to the underlying store until
    /// `commit` completes. Canceling the returned future may cause the insertion to
    /// be absent or present in the transaction's local view, but it will not affect
    /// the global store state until commit.
    fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Removes a statement from the store.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The removal is recorded in the transaction's
    /// local mutation set and does not modify the underlying store until
    /// `commit` completes. Canceling the returned future may leave the removal
    /// recorded or not in the transaction, but it will not affect the global store
    /// until commit.
    fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Deletes all statements matching the given statement pattern.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Deletions are recorded as per-transaction
    /// removals and are not applied to the underlying store until `commit`
    /// completes. Canceling the returned future may result in a partially-applied
    /// set of deletions within the transaction's local mutation set, but it will
    /// not modify the persisted store until the transaction is committed.
    fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}

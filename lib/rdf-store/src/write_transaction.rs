// This is free and unencumbered software released into the public domain.

use core::{fmt::Debug, hash::Hash};
use rdf_model::{HeapTerm, QuadPattern, Statement, StatementPattern, Term};

/// A read-write (R/W) transaction on a [`Store`].
pub trait WriteTransaction {
    type Error: Debug + Send;
    type Term: Term + Clone + PartialEq + Eq + Hash + Send + From<HeapTerm>;
    type Statement: Statement<Term = Self::Term> + Send;
    type StatementPattern: StatementPattern<Term = Self::Term>
        + From<QuadPattern<Self::Term>>
        + Send;

    /// Aborts the transaction, discarding any changes.
    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Commits the transaction, applying all changes.
    fn commit(self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Clears all data from the store.
    fn clear(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Inserts a statement into the store.
    fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Removes a statement from the store.
    fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Deletes all statements matching the given statement pattern.
    fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}

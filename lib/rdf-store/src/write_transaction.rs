// This is free and unencumbered software released into the public domain.

use core::borrow::Borrow;
use rdf_model::Statement;

/// A read-write (R/W) transaction on a [`Store`].
pub trait WriteTransaction {
    type Error;
    type Statement: Statement;

    /// Aborts the transaction, discarding any changes.
    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Commits the transaction, applying all changes.
    fn commit(self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Clears all data from the store.
    fn clear(&mut self) -> impl Future<Output = Result<(), Self::Error>>;

    /// Inserts a statement into the store.
    fn insert(
        &mut self,
        statement: impl Borrow<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    /// Removes a statement from the store.
    fn remove(
        &mut self,
        statement: impl Borrow<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    // TODO: delete(&mut self, pattern)
}

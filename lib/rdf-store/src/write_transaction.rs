// This is free and unencumbered software released into the public domain.

use core::borrow::Borrow;
use rdf_model::Statement;

pub trait WriteTransaction {
    type Error;
    type Statement: Statement;

    fn rollback(self) -> impl Future<Output = Result<(), Self::Error>>;

    fn commit(self) -> impl Future<Output = Result<(), Self::Error>>;

    // TODO: async fn clear(&mut self) -> Result<(), Self::Error>;

    fn insert(
        &mut self,
        statement: impl Borrow<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    fn remove(
        &mut self,
        statement: impl Borrow<Self::Statement> + Send,
    ) -> impl Future<Output = Result<(), Self::Error>>;

    // TODO: delete(&mut self, pattern)
}

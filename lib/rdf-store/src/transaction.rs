// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use rdf_model::Statement;

#[async_trait]
pub trait Transaction {
    type Error;

    async fn insert_statement(&mut self, statement: &dyn Statement) -> Result<(), Self::Error>;

    async fn remove_statement(&mut self, statement: &dyn Statement) -> Result<(), Self::Error>;

    async fn commit(&mut self) -> Result<(), Self::Error>;

    async fn rollback(&mut self) -> Result<(), Self::Error>;
}

impl core::fmt::Debug for dyn Transaction<Error = core::convert::Infallible> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Transaction").finish()
    }
}

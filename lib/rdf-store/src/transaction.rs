// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use rdf_model::Statement;

#[async_trait]
pub trait Transaction {
    type Error;
    type Statement: Statement;

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    async fn remove_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    async fn commit(self) -> Result<(), Self::Error>;

    async fn rollback(self) -> Result<(), Self::Error>;
}

// This is free and unencumbered software released into the public domain.

extern crate std;

use super::SqliteError;
use alloc::boxed::Box;
use async_trait::async_trait;
use rdf_model::Statement;
use rdf_store::Transaction;

pub struct SqliteTransaction {}

#[async_trait]
impl Transaction for SqliteTransaction {
    type Error = SqliteError;

    async fn insert_statement(&mut self, _statement: &dyn Statement) -> Result<(), Self::Error> {
        std::println!("SqliteTransaction#insert"); // DEBUG
        Ok(()) // TODO
    }

    async fn remove_statement(&mut self, _statement: &dyn Statement) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn commit(&mut self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn rollback(&mut self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }
}

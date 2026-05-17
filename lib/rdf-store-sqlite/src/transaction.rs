// This is free and unencumbered software released into the public domain.

use super::SqliteError;
use alloc::boxed::Box;
use async_trait::async_trait;
use rdf_model::Statement;
use rdf_store::Transaction;

pub struct SqliteTransaction<'conn> {
    #[allow(unused)]
    pub(crate) tx: turso::transaction::Transaction<'conn>,
}

#[async_trait]
impl<'conn> Transaction for SqliteTransaction<'conn> {
    type Error = SqliteError;

    async fn insert_statement(&mut self, _statement: &dyn Statement) -> Result<(), Self::Error> {
        std::println!("SqliteTransaction#insert"); // DEBUG
        Ok(()) // TODO
    }

    async fn remove_statement(&mut self, _statement: &dyn Statement) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn commit(self: Box<Self>) -> Result<(), Self::Error> {
        Ok(self.tx.commit().await?)
    }

    async fn rollback(self: Box<Self>) -> Result<(), Self::Error> {
        Ok(self.tx.rollback().await?)
    }
}

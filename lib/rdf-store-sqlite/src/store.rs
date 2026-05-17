// This is free and unencumbered software released into the public domain.

use super::{SqliteError, SqliteTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;
use core::{
    future::Future,
    pin::{pin, Pin},
};
use rdf_store::{Store, Transaction};
use turso::{Builder, Connection, Database};

#[allow(unused)]
pub struct SqliteStore {
    pub(crate) db: Database,
    pub(crate) conn: Connection,
}

impl SqliteStore {
    pub async fn new() -> Result<Self, SqliteError> {
        let db = Builder::new_local(":memory:").build().await?;
        let conn = db.connect()?;
        Ok(Self { db, conn })
    }
}

#[async_trait]
impl Store for SqliteStore {
    type Error = SqliteError;

    async fn begin_transaction(
        &mut self,
    ) -> Result<Box<dyn Transaction<Error = Self::Error>>, Self::Error> {
        let tx = SqliteTransaction {}; // TODO
        Ok(Box::new(tx) as Box<dyn Transaction<Error = Self::Error>>)
    }
}

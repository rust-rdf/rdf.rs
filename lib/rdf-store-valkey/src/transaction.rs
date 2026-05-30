// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyStore};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use rdf_model::{HeapQuad, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};
use redis::aio::MultiplexedConnection;

/// See: <https://valkey.io/topics/transactions/>
#[derive(Debug)]
pub struct ValkeyTransaction {
    #[allow(dead_code)]
    #[debug(skip)]
    pub(crate) conn: MultiplexedConnection,
    pub(crate) writable: bool,
}

impl ValkeyTransaction {
    pub async fn begin(store: &ValkeyStore, writable: bool) -> Result<Self, ValkeyError> {
        let mut conn = store.client.get_multiplexed_async_connection().await?;
        redis::cmd("MULTI").exec_async(&mut conn).await?;
        Ok(Self { conn, writable })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

#[async_trait]
impl WriteTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Statement = HeapQuad;

    async fn rollback(mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(ValkeyError::ReadOnly);
        }
        redis::cmd("DISCARD").exec_async(&mut self.conn).await?;
        Ok(())
    }

    async fn commit(mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(ValkeyError::ReadOnly);
        }
        redis::cmd("EXEC").exec_async(&mut self.conn).await?;
        Ok(())
    }

    async fn insert(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(ValkeyError::ReadOnly);
        }
        // TODO
        Ok(())
    }

    async fn remove(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(ValkeyError::ReadOnly);
        }
        // TODO
        Ok(())
    }
}

#[async_trait]
impl ReadTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Statement = HeapQuad;
    type Term = HeapTerm;
}

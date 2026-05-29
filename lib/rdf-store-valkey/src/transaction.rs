// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyStore};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use rdf_model::{HeapQuad, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};
use redis::Connection;

#[derive(Debug)]
pub struct ValkeyTransaction {
    #[allow(dead_code)]
    #[debug(skip)]
    pub(crate) conn: Connection,
    pub(crate) writable: bool,
}

impl ValkeyTransaction {
    pub fn begin(store: &ValkeyStore, writable: bool) -> Result<Self, ValkeyError> {
        let conn = store.client.get_connection()?;
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

    async fn rollback(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(ValkeyError::ReadOnly);
        }
        // TODO
        Ok(())
    }

    async fn commit(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(ValkeyError::ReadOnly);
        }
        // TODO
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

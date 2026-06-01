// This is free and unencumbered software released into the public domain.

use crate::{ValkeyDocumentSet, ValkeyError, ValkeyStore};
use alloc::{boxed::Box, string::String, vec::Vec};
use async_trait::async_trait;
use derive_more::Debug;
use rdf_model::{HeapQuad, HeapQuadSet, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};
use redis::{JsonAsyncCommands, aio::MultiplexedConnection};
use serde_json::Value;

/// See: <https://valkey.io/topics/transactions/>
#[derive(Debug)]
pub struct ValkeyTransaction {
    #[allow(dead_code)]
    #[debug(skip)]
    pub(crate) conn: MultiplexedConnection,
    pub(crate) writable: bool,
    pub(crate) inserts: HeapQuadSet,
    pub(crate) removes: HeapQuadSet,
}

impl ValkeyTransaction {
    pub async fn begin(store: &ValkeyStore, writable: bool) -> Result<Self, ValkeyError> {
        let conn = store.client.get_multiplexed_async_connection().await?;
        Ok(Self {
            conn,
            writable,
            inserts: HeapQuadSet::new(),
            removes: HeapQuadSet::new(),
        })
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
        if !self.is_writable() {
            return Err(ValkeyError::ReadOnly);
        }
        Ok(())
    }

    async fn commit(mut self) -> Result<(), Self::Error> {
        if !self.is_writable() {
            return Err(ValkeyError::ReadOnly);
        }

        let removes = ValkeyDocumentSet::from(self.removes);
        let inserts = ValkeyDocumentSet::from(self.inserts);

        for key in removes.0.keys().chain(inserts.0.keys()) {
            redis::cmd("WATCH")
                .arg(key)
                .exec_async(&mut self.conn)
                .await?;
        }

        let mut arrpops: Vec<(String, String, i32)> = Vec::new();
        for (key, doc) in removes.0 {
            for (prop, vals) in doc.0 {
                for val in vals {
                    let path = alloc::format!("['{}']", prop);
                    let index: i32 = self.conn.json_arr_index(&key, &path, &val).await?;
                    if index >= 0 {
                        arrpops.push((key.clone(), path, index));
                    }
                }
            }
        }
        arrpops.sort();
        arrpops.reverse();

        redis::cmd("MULTI").exec_async(&mut self.conn).await?;

        for (key, path, index) in arrpops {
            let _: () = self.conn.json_arr_pop(key, path, index as _).await?;
        }

        for (key, doc) in inserts.0 {
            let val: Value = doc.into();
            let _: () = self.conn.json_set(key, ".", &val).await?;
        }

        redis::cmd("EXEC").exec_async(&mut self.conn).await?;
        Ok(())
    }

    async fn insert(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.is_writable() {
            return Err(ValkeyError::ReadOnly);
        }
        self.removes.remove(statement);
        self.inserts.insert(statement.clone());
        Ok(())
    }

    async fn remove(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.is_writable() {
            return Err(ValkeyError::ReadOnly);
        }
        self.inserts.remove(statement);
        self.removes.insert(statement.clone());
        Ok(())
    }
}

#[async_trait]
impl ReadTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Statement = HeapQuad;
    type Term = HeapTerm;
}

// This is free and unencumbered software released into the public domain.

use crate::{ValkeyDocumentSet, ValkeyError, ValkeyPath, ValkeyStore};
use alloc::{boxed::Box, string::String, vec::Vec};
use async_trait::async_trait;
use derive_more::Debug;
use rdf_model::{HeapQuad, HeapQuadSet, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};
use redis::{AsyncTypedCommands, JsonAsyncCommands, RedisError, aio::MultiplexedConnection};
use serde_json::{Map, Value};
use std::eprintln;

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

        // Watch all keys involved in the transaction, to detect concurrent
        // modifications that mean the transaction must be rolled back:
        for key in removes.0.keys().chain(inserts.0.keys()) {
            redis::cmd("WATCH")
                .arg(key)
                .exec_async(&mut self.conn)
                .await?;
        }

        // Collect all JSON.ARRPOP operations to be performed when the
        // transaction is committed:
        let mut arrpops: Vec<(String, ValkeyPath, i32)> = Vec::new();
        for (key, doc) in removes.0 {
            for (prop, vals) in doc.0 {
                let path = ValkeyPath::from(prop);
                for val in vals {
                    let result: Result<i32, RedisError> =
                        self.conn.json_arr_index(&key, &path, &val).await;
                    match result {
                        Ok(index) if index >= 0 => arrpops.push((key.clone(), path.clone(), index)),
                        Ok(index) if index == -1 => {},
                        Ok(_) => unreachable!(),
                        Err(_) => {},
                    }
                }
            }
        }
        arrpops.sort();
        arrpops.reverse();

        let mut newkeys: Vec<String> = Vec::new();
        let mut newprops: Vec<(String, ValkeyPath)> = Vec::new();
        let mut arrappends: Vec<(String, ValkeyPath, Value)> = Vec::new();
        for (key, doc) in inserts.0 {
            let key_exists: bool = self.conn.exists(&key).await?;
            if !key_exists {
                newkeys.push(key.clone());
            }
            for (prop, vals) in doc.0 {
                let path = ValkeyPath::from(prop);
                for val in vals {
                    if !key_exists {
                        arrappends.push((key.clone(), path.clone(), val));
                        continue;
                    }
                    let result: Result<i32, RedisError> =
                        self.conn.json_arr_index(&key, &path, &val).await;
                    match result {
                        Ok(index) if index == -1 => {
                            arrappends.push((key.clone(), path.clone(), val))
                        },
                        Ok(_) => {},
                        Err(e) if e.code() == Some("NONEXISTENT") => {
                            // The JSON path does not exist
                            newprops.push((key.clone(), path.clone()));
                            arrappends.push((key.clone(), path.clone(), val));
                        },
                        Err(e) => return Err(e.into()),
                    }
                }
            }
        }

        redis::cmd("MULTI").exec_async(&mut self.conn).await?;

        for (key, path, index) in arrpops {
            let _: () = self.conn.json_arr_pop(key, path, index as _).await?;
        }

        let empty_object = Value::Object(Map::new());
        for key in newkeys {
            let _: () = self.conn.json_set(key, ".", &empty_object).await?;
        }

        let empty_array = Value::Array(Vec::new());
        for (key, path) in newprops {
            let _: () = self.conn.json_set(key, &path, &empty_array).await?;
        }

        for (key, path, val) in arrappends {
            let _: () = self.conn.json_arr_append(key, &path, &val).await?;
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

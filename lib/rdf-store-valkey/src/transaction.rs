// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyGraphKey, ValkeyStore, ValkeyTriple, ValkeyTripleKey};
use alloc::{borrow::Cow, boxed::Box};
use async_trait::async_trait;
use core::time::Duration;
use derive_more::Debug;
use fred::clients::Transaction;
use fred::prelude::*;
use rdf_model::{HeapQuad, HeapTerm, Statement};
use rdf_store::{ReadTransaction, WriteTransaction};
use serde_json::Value;

/// See: <https://valkey.io/topics/transactions/>
#[derive(Debug)]
pub struct ValkeyTransaction {
    #[allow(dead_code)]
    #[debug(skip)]
    pub(crate) client: Client,
    pub(crate) tx: Option<Transaction>,
    //pub(crate) inserts: HeapQuadSet,
    //pub(crate) removes: HeapQuadSet,
}

impl ValkeyTransaction {
    pub async fn begin(store: &ValkeyStore, writable: bool) -> Result<Self, ValkeyError> {
        let client = Builder::from_config(store.config.clone())
            .with_connection_config(|config| {
                config.connection_timeout = Duration::from_secs(5);
                config.tcp = TcpConfig {
                    nodelay: Some(true),
                    ..Default::default()
                };
            })
            .build()?;
        client.init().await?;

        let tx = if writable { Some(client.multi()) } else { None };

        Ok(Self {
            client,
            tx,
            //inserts: HeapQuadSet::new(),
            //removes: HeapQuadSet::new(),
        })
    }

    pub fn is_writable(&self) -> bool {
        self.tx.is_some()
    }
}

#[async_trait]
impl WriteTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Statement = HeapQuad;

    async fn rollback(mut self) -> Result<(), Self::Error> {
        let Some(tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        tx.reset();

        Ok(())
    }

    async fn commit(mut self) -> Result<(), Self::Error> {
        let Some(tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        let _: () = tx.exec(true).await?;

        Ok(())
    }

    async fn insert(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        let Some(ref tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        //self.removes.remove(statement);
        //self.inserts.insert(statement.clone());

        let graph_id: Cow<'_, str> = statement
            .context()
            .map(|g| g.value_str())
            .unwrap_or_else(|| "default".into());
        let graph_key = ValkeyGraphKey::from(&graph_id);

        let triple = ValkeyTriple::from(statement.to_triple());
        let triple_id = triple.id();
        let triple_key = ValkeyTripleKey::from(&triple_id);
        let triple_json: Value = triple.into();

        let _: () = tx.json_set(triple_key, "$", triple_json, None).await?;

        let _: () = tx.sadd(graph_key, triple_id).await?;

        if true {
            let _: () = tx.sadd("rdf:g", graph_id.as_ref()).await?;
        }

        Ok(())
    }

    async fn remove(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        let Some(ref tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        //self.inserts.remove(statement);
        //self.removes.insert(statement.clone());

        let graph_id: Cow<'_, str> = statement
            .context()
            .map(|g| g.value_str())
            .unwrap_or_else(|| "default".into());
        let graph_key = ValkeyGraphKey::from(&graph_id);

        let triple = ValkeyTriple::from(statement.to_triple());
        let triple_id = triple.id();

        let _: () = tx.srem(graph_key, &triple_id).await?;

        Ok(())
    }
}

#[async_trait]
impl ReadTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Statement = HeapQuad;
    type Term = HeapTerm;
}

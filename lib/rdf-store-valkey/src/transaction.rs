// This is free and unencumbered software released into the public domain.

use crate::{
    ValkeyError, ValkeyGraphKey, ValkeyQuad, ValkeyStore, ValkeyTerm, ValkeyTriple,
    ValkeyTripleKey, ValkeyTriplePattern,
};
use alloc::{borrow::Cow, boxed::Box, string::ToString, sync::Arc, vec::Vec};
use async_trait::async_trait;
use core::time::Duration;
use derive_more::Debug;
use fred::prelude::*;
use fred::util::NONE;
use fred::{clients::Transaction, types::scan::Scanner};
use futures::{FutureExt, Stream, StreamExt, TryStreamExt, stream};
use rdf_model::{HeapQuad, Statement, StatementPattern};
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
        let triple_id = triple.id().clone();
        let triple_key = ValkeyTripleKey::from(&triple_id);
        let triple_json: Value = triple.into();

        let _: () = tx.json_set(triple_key, "$", triple_json, None).await?;

        let _: () = tx.sadd(&graph_key, triple_id).await?;

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
        let triple_id = triple.id().clone();

        let _: () = tx.srem(&graph_key, triple_id).await?;

        Ok(())
    }
}

#[async_trait]
impl ReadTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Statement = ValkeyQuad;
    type Term = ValkeyTerm;

    fn count(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Future<Output = Result<u64, Self::Error>> {
        use futures::StreamExt;
        async move { Ok(self.r#match(pattern).count().await as _) } // TODO: optimize
    }

    fn r#match(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        let pattern = pattern.map(|p| p.to_quad_pattern()).unwrap_or_default();
        let context: Arc<Option<ValkeyTerm>> = Arc::new(pattern.context().cloned());
        let pattern: ValkeyTriplePattern = pattern.into();
        let graph_key: ValkeyGraphKey = (&*context).into();

        if pattern.is_constant() {
            return self
                .client
                .sismember::<bool, ValkeyGraphKey, ValkeyTriplePattern>(
                    graph_key.clone(),
                    pattern.clone(),
                )
                .into_stream()
                .filter_map(move |result| {
                    let pattern = pattern.clone();
                    async move {
                        match result {
                            Ok(false) => None,
                            Ok(true) => Some(Ok(pattern.try_into().unwrap())),
                            Err(err) => Some(Err(err.into())),
                        }
                    }
                })
                .boxed();
        }

        let stream = self.client.sscan(graph_key, pattern.glob, None);
        stream
            .and_then(move |mut sscan_result| {
                let context = Arc::clone(&context);
                async move {
                    let mut output: Vec<Result<Self::Statement, Self::Error>> = Vec::new();

                    let Some(page) = sscan_result.take_results() else {
                        return Ok(stream::iter(output)); // an empty page
                    };

                    let client = sscan_result.create_client();
                    for element in page {
                        let triple_key = ValkeyTripleKey::from(element.into_string().unwrap());
                        let triple_json: Value = client
                            .json_get(triple_key.to_string(), NONE, NONE, NONE, "")
                            .await?;
                        //std::eprintln!("{:?}", triple_json); // DEBUG
                        output.push(match ValkeyQuad::try_from((triple_key, triple_json)) {
                            Ok(triple) => Ok(triple.with_context((*context).clone())),
                            Err(err) => Err(err),
                        });
                    }
                    Ok(stream::iter(output))
                }
            })
            .try_flatten_unordered(1)
            .boxed()
    }
}

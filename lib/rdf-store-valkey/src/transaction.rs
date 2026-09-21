// This is free and unencumbered software released into the public domain.

use crate::{
    ValkeyError, ValkeyGraphKey, ValkeyQuad, ValkeyStore, ValkeyTerm, ValkeyTriple, ValkeyTripleId,
    ValkeyTripleKey, ValkeyTriplePattern,
};
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use async_stream::stream;
use core::time::Duration;
use derive_more::Debug;
use fred::{clients::Transaction, prelude::*, types::scan::Scanner, util::NONE};
use futures::{Stream, StreamExt, TryStreamExt, stream};
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm, StatementPattern};
use rdf_store::{ReadTransaction, WriteTransaction};
use serde_json::Value;

fn graph_id(context: Option<&HeapTerm>) -> Result<Cow<'_, str>, ValkeyError> {
    match context {
        None | Some(HeapTerm::DefaultGraph) => Ok(Cow::Borrowed("default")),
        Some(HeapTerm::Iri(name)) if name != "default" => Ok(Cow::Borrowed(name)),
        _ => Err(ValkeyError::UnsupportedGraphPattern),
    }
}

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in Valkey.
///
/// # Examples
///
/// Mutate the store in a write transaction:
///
/// ```rust,compile_fail
/// let mut tx = store.write().await?;
///
/// tx.remove(old_quad).await?;
/// tx.insert(new_quad).await?;
///
/// tx.commit().await?;
/// ```
///
/// Access the store in a read-only transaction:
///
/// ```rust,compile_fail
/// let tx = store.read().await?;
///
/// tx.r#match(quad_pattern)
///     .for_each(|quad| async move {
///         eprintln!("{:?}", quad);
///     })
///     .await;
/// ```
///
/// # Flows
///
/// ```mermaid
/// sequenceDiagram
///   participant App as Application
///   participant Client as Client (rdf-store-valkey)
///   participant Valkey as Valkey Server
///
///   rect transparent
///   Note over App,Client: Begin transaction
///   App->>Client: store.begin(writable: true)
///   Client->>Valkey: MULTI
///   Valkey-->>Client: OK
///   Client-->>App: tx
///   end
///
///   rect transparent
///   Note over App,Client: Enqueue mutations
///   App->>Client: tx.remove(statement)
///   Client->>Valkey: SREM rdf:g:{graph_id} {triple_id}
///   Valkey-->>Client: QUEUED
///   App->>Client: tx.insert(statement)
///   Client->>Valkey: JSON.SET rdf:j:{triple_id} $ {triple-json}
///   Client->>Valkey: SADD rdf:g:{graph_id} {triple_id}
///   Client->>Valkey: SADD rdf:g {graph_id}
///   Valkey-->>Client: QUEUED
///   end
///
///   rect transparent
///   Note over App,Client: Commit transaction
///   App->>Client: tx.commit()
///   Client->>Valkey: EXEC
///   Valkey-->>Client: RESP array
///   Client-->>App: drop(tx)
///   end
///
///   rect transparent
///   Note over App,Client: Rollback transaction
///   App->>Client: tx.rollback()
///   Client->>Valkey: DISCARD
///   Valkey-->>Client: OK
///   Client-->>App: drop(tx)
///   end
/// ```
///
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

impl WriteTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Term = HeapTerm; // TODO
    type Statement = HeapQuad; // TODO
    type StatementPattern = HeapQuadPattern; // TODO

    async fn rollback(self) -> Result<(), Self::Error> {
        let Some(tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        tx.reset();

        Ok(())
    }

    async fn commit(self) -> Result<(), Self::Error> {
        let Some(tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        let _: () = tx.exec(true).await?;

        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Self::Error> {
        let Some(ref _tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        Ok(()) // TODO
    }

    async fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        let Some(ref tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        let statement = statement.into();
        //self.removes.remove(statement);
        //self.inserts.insert(statement.clone());

        let graph_id = graph_id(statement.context())?;
        let graph_key = ValkeyGraphKey::from(&graph_id);

        let triple = ValkeyTriple::from(statement.to_triple());
        let triple_id = triple.id();
        let triple_key: ValkeyTripleKey = triple_id.into();
        let triple_json: Value = (&triple).into();

        let _: () = tx.json_set(triple_key, "$", triple_json, None).await?;

        let _: () = tx.sadd(&graph_key, triple_id).await?;

        if true {
            let _: () = tx.sadd("rdf:g", graph_id.as_ref()).await?;
        }

        Ok(())
    }

    async fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        let Some(ref tx) = self.tx else {
            return Err(ValkeyError::ReadOnly);
        };

        let statement = statement.into();
        //self.inserts.remove(statement);
        //self.removes.insert(statement.clone());

        let graph_id = graph_id(statement.context())?;
        let graph_key = ValkeyGraphKey::from(&graph_id);

        let triple = ValkeyTriple::from(statement.to_triple());
        let triple_id = triple.id();

        let _: () = tx.srem(&graph_key, triple_id).await?;

        Ok(())
    }

    async fn delete(
        &mut self,
        _pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }
}

impl ReadTransaction for ValkeyTransaction {
    type Error = ValkeyError;
    type Term = ValkeyTerm;
    type Statement = ValkeyQuad;
    type StatementPattern = ValkeyTriplePattern; // TODO

    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> {
        stream! {
            let graph_ids: Vec<String> = self.client.smembers("rdf:g").await.unwrap_or_default();
            for graph_id in graph_ids {
                let graph_term = match graph_id.as_str() {
                    "default" => continue, // skip the default graph
                    _ => ValkeyTerm(Value::String(graph_id.into()), false),
                };
                yield Ok(graph_term);
            }
        }
    }

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        let pattern = pattern.into();
        let (graph_key, context) = match pattern.graph_key_and_context() {
            Ok(graph) => graph,
            Err(error) => return stream::once(core::future::ready(Err(error))).boxed(),
        };
        let context = Arc::new(context);

        if pattern.is_constant() {
            return async_stream::try_stream! {
                let exists: bool = self.client.sismember(graph_key, pattern.clone()).await?;
                if exists {
                    // IDs are currently truncated hashes. Fetch and compare the
                    // actual terms rather than synthesizing a match from the query.
                    let id = pattern.glob.clone();
                    let key = ValkeyTripleKey::from(&id);
                    let json: Value = self.client.json_get(key, NONE, NONE, NONE, "").await?;
                    let quad = ValkeyTriple::try_from((id, json))?.with_context((*context).clone());
                    if pattern.matches_statement(&quad) {
                        yield quad;
                    }
                }
            }
            .boxed();
        }

        let matcher = Arc::new(pattern.clone());
        let stream = self.client.sscan(graph_key, pattern, None);
        stream
            .and_then(move |mut sscan_result| {
                let context = Arc::clone(&context);
                let matcher = Arc::clone(&matcher);
                async move {
                    let mut output: Vec<Result<Self::Statement, Self::Error>> = Vec::new();

                    let Some(page) = sscan_result.take_results() else {
                        return Ok(stream::iter(output)); // an empty page
                    };

                    let client = sscan_result.create_client();
                    for element in page {
                        let triple_id: ValkeyTripleId =
                            element.into_string().unwrap().as_str().parse().unwrap();
                        let triple_key: ValkeyTripleKey = (&triple_id).into();
                        let triple_json: Value =
                            client.json_get(triple_key, NONE, NONE, NONE, "").await?;
                        //std::eprintln!("{:?}", triple_json); // DEBUG
                        match ValkeyTriple::try_from((triple_id, triple_json)) {
                            Ok(triple) => {
                                let quad = triple.with_context((*context).clone());
                                if matcher.matches_statement(&quad) {
                                    output.push(Ok(quad));
                                }
                            },
                            Err(err) => output.push(Err(err)),
                        }
                    }
                    Ok(stream::iter(output))
                }
            })
            .try_flatten_unordered(1)
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::FutureExt;
    use rdf_model::DEFAULT_GRAPH_URN;

    #[tokio::test]
    async fn unsupported_graph_patterns_fail_without_contacting_the_server() {
        let tx = ValkeyTransaction {
            client: Builder::from_config(Config::default()).build().unwrap(),
            tx: None,
        };
        for pattern in [
            HeapQuadPattern::empty(),
            HeapQuadPattern::with_context(HeapTerm::bnode("g")),
            HeapQuadPattern::with_context(HeapTerm::string("g")),
            HeapQuadPattern::with_context(HeapTerm::iri("default")),
        ] {
            let mut stream = alloc::boxed::Box::pin(tx.r#match(pattern));
            assert!(matches!(
                stream.next().now_or_never(),
                Some(Some(Err(ValkeyError::UnsupportedGraphPattern)))
            ));
        }
    }

    #[test]
    fn write_graph_ids_do_not_alias_named_graphs_to_the_default_graph() {
        assert_eq!(graph_id(None).unwrap(), "default");
        assert_eq!(graph_id(Some(&HeapTerm::DefaultGraph)).unwrap(), "default");
        assert_eq!(
            graph_id(Some(&HeapTerm::iri(DEFAULT_GRAPH_URN))).unwrap(),
            DEFAULT_GRAPH_URN
        );
        for graph in [
            HeapTerm::bnode("default"),
            HeapTerm::iri("default"),
            HeapTerm::string("g"),
        ] {
            assert!(matches!(
                graph_id(Some(&graph)),
                Err(ValkeyError::UnsupportedGraphPattern)
            ));
        }
    }
}

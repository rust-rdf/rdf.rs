// This is free and unencumbered software released into the public domain.

use crate::{HeapStore, HeapStoreError, ReadTransaction, WriteTransaction};
use alloc::{collections::BTreeMap, sync::Arc};
use async_stream::stream;
use futures::Stream;
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm, Statement, StatementPattern};
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct HeapTransaction {
    mutations: RwLock<BTreeMap<HeapQuad, bool>>,
    store: Arc<HeapStore>,
    writable: bool,
}

impl HeapTransaction {
    pub fn new(store: Arc<HeapStore>, writable: bool) -> Self {
        Self {
            mutations: RwLock::new(BTreeMap::new()),
            store,
            writable,
        }
    }
}

impl WriteTransaction for Arc<HeapTransaction> {
    type Error = HeapStoreError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    async fn rollback(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(HeapStoreError);
        }
        Ok(())
    }

    async fn commit(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(HeapStoreError);
        }
        let mutations = self.mutations.read().await;
        let mut quads = self.store.quads.write().await;
        for (quad, &flag) in mutations.iter() {
            if flag {
                quads.insert(quad.clone());
            } else {
                quads.remove(quad);
            }
        }
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(HeapStoreError);
        }
        todo!() // TODO
    }

    async fn insert(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(HeapStoreError);
        }
        let quad = statement.into().to_quad();
        let mut mutations = self.mutations.write().await;
        mutations.insert(quad, true);
        Ok(())
    }

    async fn remove(
        &mut self,
        statement: impl Into<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(HeapStoreError);
        }
        let quad = statement.into().to_quad();
        let mut mutations = self.mutations.write().await;
        mutations.insert(quad, false);
        Ok(())
    }

    async fn delete(
        &mut self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(HeapStoreError);
        }
        let pattern = pattern.into().to_quad_pattern();
        let mut mutations = self.mutations.write().await;
        for quad in self.store.quads.read().await.iter() {
            if pattern.matches(
                quad.subject(),
                quad.predicate(),
                quad.object(),
                quad.context(),
            ) {
                mutations.insert(quad.clone(), false);
            }
        }
        Ok(())
    }
}

impl ReadTransaction for Arc<HeapTransaction> {
    type Error = HeapStoreError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        let pattern = pattern.into();
        stream! {
            let mutations = self.mutations.read().await;
            let quads = self.store.quads.read().await;
            for quad in quads.iter() {
                if pattern.matches(
                    quad.subject(),
                    quad.predicate(),
                    quad.object(),
                    quad.context(),
                ) {
                    if let Some(false) = mutations.get(quad) {
                        continue; // the quad was removed in this transaction
                    }
                    yield Ok(quad.clone());
                }
            }
            for (quad, &flag) in mutations.iter() {
                if !flag {
                    continue; // skip quads removed in this transaction
                }
                if pattern.matches(
                    quad.subject(),
                    quad.predicate(),
                    quad.object(),
                    quad.context(),
                ) {
                    yield Ok(quad.clone());
                }
            }
        }
    }
}

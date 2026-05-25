// This is free and unencumbered software released into the public domain.

use crate::{HeapStore, ReadTransaction, WriteTransaction};
use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};
use async_stream::stream;
use async_trait::async_trait;
use futures::Stream;
use parking_lot::RwLock;
use rdf_model::{HeapQuad, HeapTerm, QuadPattern, Statement, StatementPattern};

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
            store: store,
            writable,
        }
    }
}

#[async_trait]
impl WriteTransaction for Arc<HeapTransaction> {
    type Error = ();
    type Statement = HeapQuad;

    async fn rollback(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        Ok(())
    }

    async fn commit(self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        let mutations = self.mutations.read();
        let mut quads = self.store.quads.write();
        for (quad, &flag) in mutations.iter() {
            if flag {
                quads.insert(quad.clone());
            } else {
                quads.remove(quad);
            }
        }
        Ok(())
    }

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        let quad = statement.to_quad();
        let mut mutations = self.mutations.write();
        mutations.insert(quad, true);
        Ok(())
    }

    async fn remove_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        let quad = statement.to_quad();
        let mut mutations = self.mutations.write();
        mutations.insert(quad, false);
        Ok(())
    }
}

#[async_trait]
impl ReadTransaction for Arc<HeapTransaction> {
    type Error = ();
    type Term = HeapTerm;
    type Statement = HeapQuad;

    fn match_statements(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        let pattern = pattern
            .map(|s| s.to_quad_pattern())
            .unwrap_or(QuadPattern::EMPTY);
        let mutations = self.mutations.read();
        let quads = self.store.quads.read();
        stream! {
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

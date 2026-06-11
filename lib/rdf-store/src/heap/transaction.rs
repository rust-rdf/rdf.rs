// This is free and unencumbered software released into the public domain.

use crate::{HeapStore, ReadTransaction, WriteTransaction};
use alloc::{collections::BTreeMap, sync::Arc};
use async_stream::stream;
use core::borrow::Borrow;
use futures::Stream;
use parking_lot::RwLock;
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm, Statement, StatementPattern};

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
    type Error = ();
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

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

    async fn clear(&mut self) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        todo!() // TODO
    }

    async fn insert(
        &mut self,
        statement: impl Borrow<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        let quad = statement.borrow().to_quad();
        let mut mutations = self.mutations.write();
        mutations.insert(quad, true);
        Ok(())
    }

    async fn remove(
        &mut self,
        statement: impl Borrow<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        let quad = statement.borrow().to_quad();
        let mut mutations = self.mutations.write();
        mutations.insert(quad, false);
        Ok(())
    }

    async fn delete(
        &mut self,
        pattern: impl Borrow<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error> {
        if !self.writable {
            return Err(());
        }
        let pattern = pattern.borrow().to_quad_pattern();
        let mut mutations = self.mutations.write();
        for quad in self.store.quads.read().iter() {
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
    type Error = ();
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn r#match(
        &self,
        pattern: impl Borrow<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        let pattern_ = pattern.borrow().clone();
        let mutations = self.mutations.read();
        let quads = self.store.quads.read();
        stream! {
            for quad in quads.iter() {
                if pattern_.matches(
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
                if pattern_.matches(
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

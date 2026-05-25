// This is free and unencumbered software released into the public domain.

use crate::Transaction;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use async_stream::stream;
use async_trait::async_trait;
use futures::Stream;
use rdf_model::{HeapQuad, HeapTerm, StatementPattern};

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HeapTransaction {
    mutations: BTreeMap<HeapQuad, bool>,
}

impl HeapTransaction {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Transaction for HeapTransaction {
    type Error = ();
    type Term = HeapTerm;
    type Statement = HeapQuad;

    fn match_statements(
        &self,
        pattern: impl StatementPattern<Term = Self::Term>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        let _pattern = pattern.to_quad_pattern();
        stream! {
            yield Err(()); // TODO
        }
    }

    async fn insert_statement(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        Err(()) // TODO
    }

    async fn remove_statement(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        Err(()) // TODO
    }

    async fn commit(self) -> Result<(), Self::Error> {
        Err(()) // TODO
    }

    async fn rollback(self) -> Result<(), Self::Error> {
        Err(()) // TODO
    }
}

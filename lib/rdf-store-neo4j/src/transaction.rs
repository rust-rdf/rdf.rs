// This is free and unencumbered software released into the public domain.

use crate::{Neo4jError, Neo4jStore};
use core::borrow::Borrow;
use derive_more::Debug;
use futures::{Stream, stream};
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in Neo4j.
#[derive(Debug)]
pub struct Neo4jTransaction {
    pub writable: bool,
}

impl Neo4jTransaction {
    pub async fn begin(_store: &Neo4jStore, writable: bool) -> Result<Self, Neo4jError> {
        // TODO
        Ok(Self { writable })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

impl WriteTransaction for Neo4jTransaction {
    type Error = Neo4jError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    async fn rollback(self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn commit(self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn clear(&mut self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn insert(
        &mut self,
        _statement: impl Borrow<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn remove(
        &mut self,
        _statement: impl Borrow<Self::Statement> + Send,
    ) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn delete(
        &mut self,
        _pattern: impl Borrow<Self::StatementPattern> + Send,
    ) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }
}

impl ReadTransaction for Neo4jTransaction {
    type Error = Neo4jError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn r#match(
        &self,
        _pattern: impl Borrow<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}

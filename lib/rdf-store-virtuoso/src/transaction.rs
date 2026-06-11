// This is free and unencumbered software released into the public domain.

use crate::{VirtuosoError, VirtuosoStore};
use core::borrow::Borrow;
use derive_more::Debug;
use futures::{Stream, stream};
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm, StatementPattern};
use rdf_store::{ReadTransaction, WriteTransaction};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in Virtuoso.
#[derive(Debug)]
pub struct VirtuosoTransaction {
    pub writable: bool,
}

impl VirtuosoTransaction {
    pub async fn begin(_store: &VirtuosoStore, writable: bool) -> Result<Self, VirtuosoError> {
        // TODO
        Ok(Self { writable })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

impl WriteTransaction for VirtuosoTransaction {
    type Error = VirtuosoError;
    type Term = HeapTerm; // TODO
    type Statement = HeapQuad; // TODO
    type StatementPattern = HeapQuadPattern; // TODO

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

impl ReadTransaction for VirtuosoTransaction {
    type Error = VirtuosoError;
    type Term = HeapTerm; // TODO
    type Statement = HeapQuad; // TODO
    type StatementPattern = HeapQuadPattern; // TODO

    fn r#match(
        &self,
        _pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}

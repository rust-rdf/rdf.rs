// This is free and unencumbered software released into the public domain.

use crate::{OxigraphError, OxigraphStore};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use futures::{Stream, stream};
use rdf_model::{HeapQuad, HeapTerm, StatementPattern};
use rdf_store::{ReadTransaction, WriteTransaction};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in Oxigraph.
#[derive(Debug)]
pub struct OxigraphTransaction {
    pub writable: bool,
}

impl OxigraphTransaction {
    pub async fn begin(_store: &OxigraphStore, writable: bool) -> Result<Self, OxigraphError> {
        // TODO
        Ok(Self { writable })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

#[async_trait]
impl WriteTransaction for OxigraphTransaction {
    type Error = OxigraphError;
    type Statement = HeapQuad;

    async fn rollback(mut self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn commit(mut self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn insert(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn remove(&mut self, _statement: &Self::Statement) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }
}

#[async_trait]
impl ReadTransaction for OxigraphTransaction {
    type Error = OxigraphError;
    type Statement = HeapQuad;
    type Term = HeapTerm;

    fn r#match(
        &self,
        _pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}

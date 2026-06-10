// This is free and unencumbered software released into the public domain.

use crate::{PostgresError, PostgresStore};
use alloc::boxed::Box;
use async_trait::async_trait;
use core::borrow::Borrow;
use derive_more::Debug;
use futures::{Stream, stream};
use rdf_model::{HeapQuad, HeapTerm, StatementPattern};
use rdf_store::{ReadTransaction, WriteTransaction};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in PostgreSQL.
#[derive(Debug)]
pub struct PostgresTransaction {
    pub writable: bool,
}

impl PostgresTransaction {
    pub async fn begin(_store: &PostgresStore, writable: bool) -> Result<Self, PostgresError> {
        // TODO
        Ok(Self { writable })
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }
}

#[async_trait]
impl WriteTransaction for PostgresTransaction {
    type Error = PostgresError;
    type Statement = HeapQuad;

    async fn rollback(mut self) -> Result<(), Self::Error> {
        Ok(()) // TODO
    }

    async fn commit(mut self) -> Result<(), Self::Error> {
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
}

#[async_trait]
impl ReadTransaction for PostgresTransaction {
    type Error = PostgresError;
    type Statement = HeapQuad;
    type Term = HeapTerm;

    fn r#match(
        &self,
        _pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}

// This is free and unencumbered software released into the public domain.

use crate::{OxigraphError, OxigraphStore};
use alloc::boxed::Box;
use core::borrow::Borrow;
use futures::{Stream, stream};
use ouroboros::self_referencing;
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm};
use rdf_store::{ReadTransaction, WriteTransaction};

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A transaction for reading and writing statements in Oxigraph.
#[self_referencing]
pub struct OxigraphTransaction {
    writable: bool,

    store: OxigraphStore,

    #[borrows(store)]
    #[covariant]
    inner: oxigraph::store::Transaction<'this>,
}

impl OxigraphTransaction {
    pub async fn begin(store: &OxigraphStore, writable: bool) -> Result<Self, OxigraphError> {
        let store = store.clone();
        let transaction = OxigraphTransactionAsyncSendTryBuilder {
            writable,
            store,
            inner_builder: |store| Box::pin(async move { store.inner.start_transaction() }),
        }
        .try_build()
        .await?;
        Ok(transaction)
    }

    pub fn is_writable(&self) -> bool {
        *self.borrow_writable()
    }
}

impl WriteTransaction for OxigraphTransaction {
    type Error = OxigraphError;
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

impl ReadTransaction for OxigraphTransaction {
    type Error = OxigraphError;
    type Term = HeapTerm; // TODO
    type Statement = HeapQuad; // TODO
    type StatementPattern = HeapQuadPattern; // TODO

    fn r#match(
        &self,
        _pattern: impl Borrow<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty() // TODO
    }
}

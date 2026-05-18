// This is free and unencumbered software released into the public domain.

use super::SqliteError;
use alloc::boxed::Box;
use async_trait::async_trait;
use rdf_model::{HeapQuad, HeapTerm, Statement, Term, TermKind};
use rdf_store::Transaction;

pub struct SqliteTransaction<'conn> {
    #[allow(unused)]
    pub(crate) tx: turso::transaction::Transaction<'conn>,
}

impl<'conn> SqliteTransaction<'conn> {
    async fn register_term(&mut self, term: &HeapTerm) -> Result<u64, SqliteError> {
        match term.kind() {
            TermKind::Iri => {}
            TermKind::BNode => {}
            TermKind::Literal => {}
        }
        Ok(0) // TODO
    }
}

#[async_trait]
impl<'conn> Transaction for SqliteTransaction<'conn> {
    type Error = SqliteError;
    type Statement = HeapQuad;

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        let _s = self.register_term(statement.subject()).await?;
        let _p = self.register_term(statement.predicate()).await?;
        let _o = self.register_term(statement.object()).await?;
        let _g = match statement.context() {
            Some(g) => Some(self.register_term(g).await?),
            None => None,
        };
        Ok(()) // TODO
    }

    async fn remove_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error> {
        let _s = self.register_term(statement.subject()).await?;
        let _p = self.register_term(statement.predicate()).await?;
        let _o = self.register_term(statement.object()).await?;
        let _g = match statement.context() {
            Some(g) => Some(self.register_term(g).await?),
            None => None,
        };
        Ok(()) // TODO
    }

    async fn commit(self) -> Result<(), Self::Error> {
        Ok(self.tx.commit().await?)
    }

    async fn rollback(self) -> Result<(), Self::Error> {
        Ok(self.tx.rollback().await?)
    }
}

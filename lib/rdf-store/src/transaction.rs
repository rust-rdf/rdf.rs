// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use futures::Stream;
use rdf_model::{Statement, StatementPattern, Term};

#[async_trait]
pub trait Transaction {
    type Error;
    type Term: Term;
    type Statement: Statement;

    async fn rollback(self) -> Result<(), Self::Error>;

    async fn commit(self) -> Result<(), Self::Error>;

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    async fn remove_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    fn match_statements(
        &self,
        pattern: impl StatementPattern<Term = Self::Term>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>>;
}

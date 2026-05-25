// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use futures::Stream;
use rdf_model::{AnyStatement, Statement, StatementPattern, Term};

#[async_trait]
pub trait Transaction {
    type Error;

    async fn rollback(self) -> Result<(), Self::Error>;

    async fn commit(self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait ReadTransaction {
    type Error;
    type Term: Term + Clone;
    type Statement: Statement;

    fn scan_statements(&self) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        self.match_statements(AnyStatement::<Self::Term>::new())
    }

    fn match_statements(
        &self,
        pattern: impl StatementPattern<Term = Self::Term>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>>;
}

#[async_trait]
pub trait WriteTransaction {
    type Error;
    type Statement: Statement;

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    async fn remove_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;
}

// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use futures::Stream;
use rdf_model::{Statement, StatementPattern, Term};

//#[async_trait]
pub trait ReadTransaction {
    type Error;
    type Statement: Statement;
    type Term: Term + Clone;

    fn count_statements(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Future<Output = Result<u64, Self::Error>> {
        use futures::StreamExt;
        async move { Ok(self.match_statements(pattern).count().await as _) }
    }

    fn match_statements(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>>;
}

#[async_trait]
pub trait WriteTransaction {
    type Error;
    type Statement: Statement;

    async fn rollback(self) -> Result<(), Self::Error>;

    async fn commit(self) -> Result<(), Self::Error>;

    // TODO: delete_statements

    async fn insert_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    async fn remove_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;
}

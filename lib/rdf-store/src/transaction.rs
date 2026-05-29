// This is free and unencumbered software released into the public domain.

use alloc::boxed::Box;
use async_trait::async_trait;
use futures::stream::{self, Stream};
use rdf_model::{Statement, StatementPattern, Term};

//#[async_trait]
pub trait ReadTransaction {
    type Error;
    type Statement: Statement;
    type Term: Term + Clone;

    fn count(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Future<Output = Result<u64, Self::Error>> {
        use futures::StreamExt;
        async move { Ok(self.r#match(pattern).count().await as _) }
    }

    fn r#match(
        &self,
        _pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty()
    }
}

#[async_trait]
pub trait WriteTransaction {
    type Error;
    type Statement: Statement;

    async fn rollback(self) -> Result<(), Self::Error>;

    async fn commit(self) -> Result<(), Self::Error>;

    async fn insert(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    async fn remove(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;

    // TODO: delete(&mut self, pattern)
}

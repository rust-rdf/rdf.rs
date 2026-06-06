// This is free and unencumbered software released into the public domain.

use async_trait::async_trait;
use futures::stream::{self, Stream};
use rdf_model::{Statement, StatementPattern, Term};

#[async_trait]
pub trait ReadTransaction {
    type Error;
    type Statement: Statement;
    type Term: Term + Clone;

    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> {
        stream::empty()
    }

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

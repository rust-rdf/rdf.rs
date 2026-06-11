// This is free and unencumbered software released into the public domain.

use futures::stream::{self, Stream};
use rdf_model::{Statement, StatementPattern, Term};

/// A read-only (R/O) transaction on a [`Store`].
pub trait ReadTransaction {
    type Error;
    type Statement: Statement;
    type Term: Term + Clone;

    /// Returns a stream of all context terms (graph names) in the store.
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> {
        stream::empty()
    }

    /// Returns `true` if the store contains the given statement.
    fn contains(
        &self,
        statement: impl Statement<Term = Self::Term> + Send,
    ) -> impl Future<Output = Result<bool, Self::Error>> {
        use futures::future::TryFutureExt;
        let pattern = statement.to_quad_pattern();
        self.count(Some(pattern)).map_ok(|count| count > 0)
    }

    /// Returns the number of statements matching the given statement pattern.
    fn count(
        &self,
        pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Future<Output = Result<u64, Self::Error>> {
        use futures::StreamExt;
        async move { Ok(self.r#match(pattern).count().await as _) }
    }

    /// Returns a stream of all statements matching the given statement pattern.
    fn r#match(
        &self,
        _pattern: Option<impl StatementPattern<Term = Self::Term>>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty()
    }
}

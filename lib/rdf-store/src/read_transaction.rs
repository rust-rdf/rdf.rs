// This is free and unencumbered software released into the public domain.

use core::hash::Hash;
use futures::stream::{self, Stream};
use rdf_model::{QuadPattern, Statement, StatementPattern, Term};

/// A read-only (R/O) transaction on a [`Store`].
pub trait ReadTransaction {
    type Error;
    type Term: Term + Clone + PartialEq + Eq + Hash;
    type Statement: Statement<Term = Self::Term>;
    type StatementPattern: StatementPattern<Term = Self::Term> + From<QuadPattern<Self::Term>>;

    /// Returns `true` if the store is empty (contains no statements).
    fn is_empty(&self) -> impl Future<Output = Result<bool, Self::Error>> {
        use futures::future::TryFutureExt;
        self.count(QuadPattern::default()).map_ok(|count| count > 0)
    }

    /// Returns a stream of all context terms (graph names) in the store.
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> {
        stream::empty()
    }

    /// Returns `true` if the store contains the given statement (pattern).
    fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Future<Output = Result<bool, Self::Error>> {
        use futures::future::TryFutureExt;
        self.count(pattern).map_ok(|count| count > 0)
    }

    /// Returns the number of statements matching the given statement pattern.
    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern>,
    ) -> impl Future<Output = Result<u64, Self::Error>> {
        use futures::StreamExt;
        async move { Ok(self.r#match(pattern).count().await as _) }
    }

    /// Returns a stream of all statements matching the given statement pattern.
    fn r#match(
        &self,
        _pattern: impl Into<Self::StatementPattern>,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> {
        stream::empty()
    }
}

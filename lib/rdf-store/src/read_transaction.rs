// This is free and unencumbered software released into the public domain.

use core::{fmt::Debug, hash::Hash};
use futures::{
    FutureExt,
    stream::{self, Stream},
};
use rdf_model::{QuadPattern, Statement, StatementPattern, Term};

/// A read-only (R/O) transaction on a [`Store`].
pub trait ReadTransaction {
    type Error: Debug + Send;
    type Term: Term + Clone + PartialEq + Eq + Hash + Send;
    type Statement: Statement<Term = Self::Term> + Send;
    type StatementPattern: StatementPattern<Term = Self::Term>
        + From<QuadPattern<Self::Term>>
        + Send;

    /// Returns `true` if the store is empty (contains no statements).
    fn is_empty(&self) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        use futures::future::TryFutureExt;
        self.count(QuadPattern::default()).map_ok(|count| count > 0)
    }

    /// Returns a stream of all context terms (graph names) in the store.
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> + Send {
        stream::empty()
    }

    /// Returns `true` if the store contains the given statement (pattern).
    fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        use futures::future::TryFutureExt;
        self.count(pattern).map_ok(|count| count > 0)
    }

    /// Returns the number of statements matching the given statement pattern.
    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send {
        use futures::StreamExt;
        self.r#match(pattern).count().map(|count| Ok(count as u64))
    }

    /// Returns a stream of all statements matching the given statement pattern.
    fn r#match(
        &self,
        _pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        stream::empty()
    }
}

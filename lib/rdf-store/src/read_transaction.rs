// This is free and unencumbered software released into the public domain.

use core::{fmt::Debug, hash::Hash};
use futures::{
    FutureExt,
    stream::{self, Stream},
};
use rdf_model::{HeapTerm, QuadPattern, Statement, StatementPattern, Term};

/// A read-only (R/O) transaction on a [`Store`].
///
/// # Semantics for implementors
///
/// - Read transactions present a read-only view of the store. Implementations
///   **should** document whether that view is a consistent snapshot
///   (repeatable reads) or a weaker, potentially-changing view.
/// - Methods in this trait must not modify persistent store state. They may
///   acquire read locks or other resources to produce results, but they must
///   release those resources promptly when the returned future or stream is
///   dropped.
/// - Streams returned from this trait (`contexts`, `match`) may yield results
///   lazily. Implementations that hold resources while streaming **must**
///   ensure those resources are released when the stream is dropped or fully
///   consumed.
/// - Avoid holding long-lived write locks inside read transaction methods; if
///   locks are necessary, prefer acquiring them in a consistent order relative
///   to other components to reduce deadlock risk.
/// - Operations should be cancel safe: dropping the returned future or stream
///   must not mutate the underlying store and should stop further work.
///
pub trait ReadTransaction {
    type Error: Debug + Send;
    type Term: Term + Clone + PartialEq + Eq + Hash + Send + From<HeapTerm>;
    type Statement: Statement<Term = Self::Term> + Send;
    type StatementPattern: StatementPattern<Term = Self::Term>
        + From<QuadPattern<Self::Term>>
        + Send;

    /// Returns `true` if the store is empty (contains no statements).
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. It is implemented in terms of other
    /// read-only APIs and only observes store state; canceling or dropping the
    /// returned future will not modify the store. Implementations may perform
    /// asynchronous reads (locks, I/O, or iteration); dropping the future should
    /// stop any further work and release resources promptly.
    fn is_empty(&self) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        use futures::future::TryFutureExt;
        self.count(QuadPattern::default()).map_ok(|count| count > 0)
    }

    /// Returns a stream of all context terms (graph names) in the store.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The returned stream may be consumed
    /// partially; dropping the stream must not mutate the store. Implementations
    /// that enumerate contexts lazily should ensure any held resources (locks,
    /// iterators, or I/O handles) are released when the stream is dropped or when
    /// iteration completes.
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> + Send {
        stream::empty()
    }

    /// Returns `true` if the store contains the given statement (pattern).
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. It performs only read-side work and returns a
    /// future that observes whether matching statements exist. Canceling the
    /// future will not change store contents; implementations should ensure any
    /// resources acquired while computing the result are released if the future
    /// is dropped.
    fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        use futures::future::TryFutureExt;
        self.count(pattern).map_ok(|count| count > 0)
    }

    /// Returns the number of statements matching the given statement pattern.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. It consumes a read-only stream of matching
    /// statements and counts them; canceling the returned future will stop the
    /// count early and will not modify store state. Implementations should make
    /// sure any iteration resources are released on cancellation.
    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send {
        use futures::StreamExt;
        self.r#match(pattern).count().map(|count| Ok(count as u64))
    }

    /// Returns a stream of all statements matching the given statement pattern.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The returned stream may be consumed
    /// partially; dropping the stream must not mutate the store. Implementations
    /// that hold resources (locks, file handles, network cursors) while
    /// streaming results should ensure those resources are released when the
    /// stream is dropped or fully consumed.
    fn r#match(
        &self,
        _pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        stream::empty()
    }
}

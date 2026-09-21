// This is free and unencumbered software released into the public domain.

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, vec::Vec};
use core::{fmt::Debug, hash::Hash};
use futures::{Stream, TryFutureExt, TryStreamExt, future};
use rdf_model::{HeapTerm, QuadPattern, Statement, StatementPattern, Term};

/// A read-only (R/O) transaction on a [`Store`](crate::Store).
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
/// - The cost hierarchy is `contains < count < match`. Use the cheapest
///   sufficient method: [`contains`](Self::contains) for existence,
///   [`count`](Self::count) for cardinality, and [`match`](Self::match) for
///   statements. Delegate through these methods to preserve backend overrides;
///   their default fallbacks do not determine the cost of native operations.
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
    /// The default implementation negates [`Self::contains`] with an all-wildcard
    /// pattern. This preserves backend existence optimizations that can avoid
    /// both counting all statements and allocating or initializing a cursor.
    ///
    /// # Errors
    ///
    /// Propagates any error returned by [`Self::contains`].
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. It is implemented in terms of other
    /// read-only APIs and only observes store state; canceling or dropping the
    /// returned future will not modify the store. Implementations may perform
    /// asynchronous reads (locks, I/O, or iteration); dropping the future should
    /// stop any further work and release resources promptly.
    fn is_empty(&self) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        self.contains(QuadPattern::default()).map_ok(|found| !found)
    }

    /// Returns a stream of distinct context terms (graph names) in the store.
    ///
    /// The default implementation lazily scans [`match`](Self::match) with an
    /// all-wildcard pattern. It skips the default graph (`None`) and compares
    /// complete terms using [`Eq`]. It yields names in first-seen order and
    /// retains one cloned term per distinct name until the stream is dropped
    /// or exhausted. It cannot discover named graphs that contain no statements.
    /// Backends may override this method to use their graph indexes.
    /// Without `alloc`, backends must implement this method directly.
    ///
    /// # Errors
    ///
    /// The default stream yields the first matching-stream error and then ends.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The returned stream may be consumed
    /// partially; dropping the stream must not mutate the store. Implementations
    /// that enumerate contexts lazily should ensure any held resources (locks,
    /// iterators, or I/O handles) are released when the stream is dropped or when
    /// iteration completes.
    #[cfg(feature = "alloc")]
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> + Send {
        let statements = Box::pin(self.r#match(QuadPattern::default()));
        futures::stream::try_unfold(
            (statements, Vec::new()),
            |(mut statements, mut seen)| async move {
                while let Some(statement) = statements.try_next().await? {
                    let Some(context) = statement.context().cloned() else {
                        continue;
                    };
                    if !seen.contains(&context) {
                        seen.push(context.clone());
                        return Ok(Some((context, (statements, seen))));
                    }
                }
                Ok(None)
            },
        )
    }

    /// Returns a stream of distinct context terms (graph names) in the store,
    /// excluding the default graph.
    ///
    /// Without the `alloc` feature, implementors must provide this method.
    /// With `alloc`, a default implementation scans [`match`](Self::match) and
    /// retains the distinct names to deduplicate them using [`Eq`].
    ///
    /// # Errors
    ///
    /// Yield backend or unsupported-operation errors as `Err` items rather than
    /// silently returning an empty stream.
    ///
    /// # Cancel safety
    ///
    /// Dropping the stream must release its resources without mutating the store.
    #[cfg(not(feature = "alloc"))]
    fn contexts(&self) -> impl Stream<Item = Result<Self::Term, Self::Error>> + Send;

    /// Returns `true` if the store contains the given statement (pattern).
    ///
    /// Existence testing is cheaper than counting: a backend only needs to
    /// establish whether a match exists, without counting all matches or
    /// materializing statements. Existence-only operations, including
    /// [`Self::is_empty`], must delegate here to preserve native optimizations.
    /// Backends should override this method with a native existence check when
    /// available.
    ///
    /// The default fallback calls [`Self::count`] with the given pattern
    /// and checks whether the result is positive. It delegates to `count`
    /// rather than polling [`match`](Self::match), even for a single statement,
    /// so backend count optimizations can avoid cursor allocation and setup.
    ///
    /// # Errors
    ///
    /// Propagates backend existence-check errors. The default implementation
    /// propagates any error returned by [`Self::count`].
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
        self.count(pattern).map_ok(|count| count > 0)
    }

    /// Returns the number of statements matching the given statement pattern.
    ///
    /// # Cost model
    ///
    /// Counting is fundamentally cheaper than [`match`](Self::match): a backend
    /// may answer from indexes or metadata without allocating and initializing
    /// cursor state or materializing statements. Cardinality queries must use
    /// `count` rather than construct a matching stream. Existence-only queries
    /// must use [`Self::contains`], which is cheaper still and can avoid counting
    /// all matches.
    ///
    /// The default implementation is a fallback that consumes `match` and
    /// counts successful statements without buffering them. Backends should
    /// override it with a native count operation when available.
    ///
    /// # Errors
    ///
    /// Propagates backend counting errors. The default implementation returns
    /// the first matching-stream error, dropping the stream and discarding the
    /// partial count. Errors are never counted as statements.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. The default implementation consumes a
    /// read-only stream of matching statements; canceling the returned future
    /// will stop the count early and will not modify store state. Implementations
    /// must release counting resources on cancellation.
    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send {
        self.r#match(pattern)
            .try_fold(0u64, |count, _| future::ready(Ok(count + 1)))
    }

    /// Returns a stream of all statements matching the given statement pattern.
    ///
    /// Implementors must provide this method; there is no empty-stream default.
    /// An empty stream must mean that no statements match the pattern.
    /// Use [`Self::contains`] for existence checks and [`Self::count`] for counts
    /// to preserve cheaper backend operations that can avoid cursor setup.
    ///
    /// # Errors
    ///
    /// Yield backend or unsupported-operation errors as `Err` items rather than
    /// silently returning an empty stream.
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
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send;
}

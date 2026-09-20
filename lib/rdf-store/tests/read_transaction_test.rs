// This is free and unencumbered software released into the public domain.

#![cfg(feature = "alloc")]

use futures::{FutureExt, Stream, StreamExt, TryStreamExt, future, pin_mut};
use rdf_model::{HeapQuad, HeapQuadPattern, HeapTerm, SAMPLE_QUAD};
use rdf_store::{HeapStore, ReadTransaction, Store, WriteTransaction};
use std::{
    cell::Cell,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReadError;

struct DropFlag(Arc<AtomicBool>);

impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

// Deliberately !Sync. The defaults must preserve Send without requiring Sync
// transactions or Unpin matching streams.
#[derive(Default)]
struct Transaction {
    items: Vec<Result<HeapQuad, ReadError>>,
    expected_pattern: HeapQuadPattern,
    yielded: Arc<AtomicUsize>,
    dropped: Arc<AtomicBool>,
    pending_at_end: Cell<bool>,
}

impl Transaction {
    fn new(items: Vec<Result<HeapQuad, ReadError>>) -> Self {
        Self {
            items,
            ..Self::default()
        }
    }

    fn assert_finished(&self, yielded: usize) {
        assert_eq!(self.yielded.load(Ordering::SeqCst), yielded);
        assert!(self.dropped.load(Ordering::SeqCst));
    }
}

impl ReadTransaction for Transaction {
    type Error = ReadError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        assert_eq!(pattern.into(), self.expected_pattern);
        let items = self.items.clone();
        let yielded = self.yielded.clone();
        let guard = DropFlag(self.dropped.clone());
        let pending_at_end = self.pending_at_end.get();
        async_stream::stream! {
            let _guard = guard;
            for item in items {
                yielded.fetch_add(1, Ordering::SeqCst);
                yield item;
            }
            if pending_at_end {
                future::pending::<()>().await;
            }
        }
    }
}

// Models a backend whose native count does not need a matching cursor.
struct CountingTransaction {
    result: Result<u64, ReadError>,
    expected_pattern: HeapQuadPattern,
    count_calls: Cell<usize>,
    match_calls: Cell<usize>,
}

impl CountingTransaction {
    fn new(result: Result<u64, ReadError>) -> Self {
        Self {
            result,
            expected_pattern: HeapQuadPattern::default(),
            count_calls: Cell::new(0),
            match_calls: Cell::new(0),
        }
    }
}

impl ReadTransaction for CountingTransaction {
    type Error = ReadError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send {
        assert_eq!(pattern.into(), self.expected_pattern);
        self.count_calls.set(self.count_calls.get() + 1);
        future::ready(self.result.clone())
    }

    fn r#match(
        &self,
        _pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        self.match_calls.set(self.match_calls.get() + 1);
        futures::stream::empty()
    }
}

// Models a backend whose native existence check is cheaper than counting.
struct ExistenceTransaction {
    result: Result<bool, ReadError>,
    contains_calls: Cell<usize>,
    fallback: CountingTransaction,
}

impl ReadTransaction for ExistenceTransaction {
    type Error = ReadError;
    type Term = HeapTerm;
    type Statement = HeapQuad;
    type StatementPattern = HeapQuadPattern;

    fn contains(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        assert_eq!(pattern.into(), HeapQuadPattern::default());
        self.contains_calls.set(self.contains_calls.get() + 1);
        future::ready(self.result.clone())
    }

    fn count(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Future<Output = Result<u64, Self::Error>> + Send {
        self.fallback.count(pattern)
    }

    fn r#match(
        &self,
        pattern: impl Into<Self::StatementPattern> + Send,
    ) -> impl Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        self.fallback.r#match(pattern)
    }
}

fn assert_send<T: Send>(value: T) -> T {
    value
}

fn quad(context: Option<HeapTerm>) -> HeapQuad {
    HeapQuad::from(SAMPLE_QUAD).with_context(context)
}

#[tokio::test]
async fn empty_stream_has_no_statements_or_contexts() {
    let tx = Transaction::default();
    assert_eq!(assert_send(tx.is_empty()).await, Ok(true));
    assert_eq!(assert_send(tx.contains(())).await, Ok(false));
    assert_eq!(assert_send(tx.count(())).await, Ok(0));
    assert_eq!(
        assert_send(tx.contexts()).try_collect::<Vec<_>>().await,
        Ok(vec![])
    );
    tx.assert_finished(0);
}

#[tokio::test]
async fn is_empty_uses_backend_contains_without_counting_or_matching() {
    for result in [Ok(false), Ok(true), Err(ReadError)] {
        let expected = result.clone().map(|found| !found);
        let tx = ExistenceTransaction {
            result,
            contains_calls: Cell::new(0),
            fallback: CountingTransaction::new(Err(ReadError)),
        };
        assert_eq!(assert_send(tx.is_empty()).await, expected);
        assert_eq!(tx.contains_calls.get(), 1);
        assert_eq!(tx.fallback.count_calls.get(), 0);
        assert_eq!(tx.fallback.match_calls.get(), 0);
    }
}

#[tokio::test]
async fn existence_checks_use_backend_counts_without_matching() {
    for count in [0, 1, 42, u64::MAX] {
        let tx = CountingTransaction::new(Ok(count));
        assert_eq!(assert_send(tx.contains(())).await, Ok(count > 0));
        assert_eq!(assert_send(tx.is_empty()).await, Ok(count == 0));
        assert_eq!(tx.count_calls.get(), 2);
        assert_eq!(tx.match_calls.get(), 0);
    }
}

#[tokio::test]
async fn existence_checks_propagate_backend_count_errors_without_matching() {
    let tx = CountingTransaction::new(Err(ReadError));
    assert_eq!(tx.contains(()).await, Err(ReadError));
    assert_eq!(tx.is_empty().await, Err(ReadError));
    assert_eq!(tx.count_calls.get(), 2);
    assert_eq!(tx.match_calls.get(), 0);
}

#[tokio::test]
async fn existence_checks_propagate_late_count_errors() {
    let tx = Transaction::new(vec![Ok(quad(None)), Err(ReadError), Ok(quad(None))]);
    assert_eq!(assert_send(tx.contains(())).await, Err(ReadError));
    tx.assert_finished(2);

    let tx = Transaction::new(vec![Ok(quad(None)), Err(ReadError), Ok(quad(None))]);
    assert_eq!(assert_send(tx.is_empty()).await, Err(ReadError));
    tx.assert_finished(2);
}

#[tokio::test]
async fn existence_checks_propagate_first_error() {
    let tx = Transaction::new(vec![Err(ReadError), Ok(quad(None))]);
    assert_eq!(tx.contains(()).await, Err(ReadError));
    tx.assert_finished(1);

    let tx = Transaction::new(vec![Err(ReadError), Ok(quad(None))]);
    assert_eq!(tx.is_empty().await, Err(ReadError));
    tx.assert_finished(1);
}

#[tokio::test]
async fn count_consumes_successful_statements() {
    let tx = Transaction::new(vec![Ok(quad(None)), Ok(quad(None)), Ok(quad(None))]);
    assert_eq!(tx.count(()).await, Ok(3));
    tx.assert_finished(3);
}

#[tokio::test]
async fn count_stops_on_errors_without_returning_a_partial_count() {
    for items in [
        vec![Err(ReadError), Ok(quad(None))],
        vec![Ok(quad(None)), Err(ReadError), Ok(quad(None))],
    ] {
        let expected_yielded = items.len() - 1;
        let tx = Transaction::new(items);
        assert_eq!(tx.count(()).await, Err(ReadError));
        tx.assert_finished(expected_yielded);
    }
}

#[tokio::test]
async fn count_and_contains_forward_the_requested_pattern() {
    let pattern = HeapQuadPattern::with_subject(HeapTerm::iri("https://example.org/subject"));
    let tx = Transaction {
        expected_pattern: pattern.clone(),
        ..Transaction::new(vec![Ok(quad(None))])
    };
    assert_eq!(tx.contains(pattern.clone()).await, Ok(true));
    assert_eq!(tx.count(pattern.clone()).await, Ok(1));

    let tx = CountingTransaction {
        expected_pattern: pattern.clone(),
        ..CountingTransaction::new(Ok(1))
    };
    assert_eq!(tx.contains(pattern).await, Ok(true));
    assert_eq!(tx.count_calls.get(), 1);
    assert_eq!(tx.match_calls.get(), 0);
}

#[tokio::test]
async fn contexts_deduplicate_complete_terms_and_skip_the_default_graph() {
    let iri = HeapTerm::iri("graph");
    let bnode = HeapTerm::bnode("graph");
    let tx = Transaction::new(vec![
        Ok(quad(None)),
        Ok(quad(Some(iri.clone()))),
        Ok(quad(Some(iri.clone()))),
        Ok(quad(Some(bnode.clone()))),
        Ok(quad(Some(bnode.clone()))),
        Ok(quad(None)),
    ]);
    assert_eq!(
        assert_send(tx.contexts()).try_collect::<Vec<_>>().await,
        Ok(vec![iri, bnode])
    );
    tx.assert_finished(6);
}

#[tokio::test]
async fn contexts_yield_the_first_error_and_then_end() {
    for prefix in [vec![], vec![Ok(quad(Some(HeapTerm::iri("urn:graph"))))]] {
        let expected_yielded = prefix.len() + 1;
        let mut items = prefix.clone();
        items.extend([Err(ReadError), Ok(quad(Some(HeapTerm::iri("urn:other"))))]);
        let tx = Transaction::new(items);
        let contexts = tx.contexts();
        pin_mut!(contexts);
        if !prefix.is_empty() {
            assert_eq!(contexts.next().await, Some(Ok(HeapTerm::iri("urn:graph"))));
        }
        assert_eq!(contexts.next().await, Some(Err(ReadError)));
        assert_eq!(contexts.next().await, None);
        tx.assert_finished(expected_yielded);
    }
}

#[tokio::test]
async fn contexts_are_lazy_and_release_the_source_when_dropped() {
    let tx = Transaction::new(vec![
        Ok(quad(Some(HeapTerm::iri("urn:graph")))),
        Err(ReadError),
    ]);
    {
        let contexts = tx.contexts();
        assert_eq!(tx.yielded.load(Ordering::SeqCst), 0);
        pin_mut!(contexts);
        assert_eq!(contexts.next().await, Some(Ok(HeapTerm::iri("urn:graph"))));
        assert_eq!(tx.yielded.load(Ordering::SeqCst), 1);
    }
    tx.assert_finished(1);
}

#[test]
fn cancelled_reads_release_pending_sources() {
    let tx = Transaction::new(vec![Ok(quad(None))]);
    tx.pending_at_end.set(true);
    assert_eq!(tx.count(()).now_or_never(), None);
    tx.assert_finished(1);

    let tx = Transaction::default();
    tx.pending_at_end.set(true);
    assert_eq!(tx.contains(()).now_or_never(), None);
    tx.assert_finished(0);

    let tx = Transaction::default();
    tx.pending_at_end.set(true);
    assert_eq!(tx.is_empty().now_or_never(), None);
    tx.assert_finished(0);
}

#[tokio::test]
async fn heap_store_defaults_observe_committed_statements_and_graphs() {
    let mut store = HeapStore::new();
    assert!(store.read().await.unwrap().is_empty().await.unwrap());

    let graph = HeapTerm::iri("https://example.org/graph");
    let mut tx = store.write().await.unwrap();
    tx.insert(quad(None)).await.unwrap();
    tx.insert(quad(Some(graph.clone()))).await.unwrap();
    tx.commit().await.unwrap();

    let tx = store.read().await.unwrap();
    assert!(!tx.is_empty().await.unwrap());
    assert!(tx.contains(()).await.unwrap());
    assert_eq!(tx.count(()).await.unwrap(), 2);
    assert_eq!(
        tx.contexts().try_collect::<Vec<_>>().await.unwrap(),
        vec![graph]
    );
}

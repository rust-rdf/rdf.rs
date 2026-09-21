// This is free and unencumbered software released into the public domain.

#![cfg(feature = "std")]

use futures::{FutureExt, StreamExt, TryStreamExt};
use rdf_model::{BaseDirection, HeapQuad, HeapQuadPattern, HeapTerm};
use rdf_store::{
    HeapStore, HeapStoreError, HeapTransaction, ReadTransaction, Store, WriteTransaction,
};
use std::{collections::BTreeSet, slice, sync::Arc};
use tokio::sync::Barrier;

fn quad(value: &str) -> HeapQuad {
    HeapQuad::new(
        HeapTerm::iri("urn:subject"),
        HeapTerm::iri("urn:predicate"),
        HeapTerm::string(value),
        None,
    )
}

async fn store_with(quads: &[HeapQuad]) -> Arc<HeapStore> {
    let mut store = HeapStore::new();
    let mut tx = store.write().await.unwrap();
    for quad in quads {
        tx.insert(quad.clone()).await.unwrap();
    }
    tx.commit().await.unwrap();
    store
}

async fn assert_view(tx: &Arc<HeapTransaction>, expected: &[HeapQuad]) {
    let mut actual = tx.r#match(()).try_collect::<Vec<_>>().await.unwrap();
    actual.sort();
    let mut expected = expected.to_vec();
    expected.sort();
    // Comparing vectors, rather than sets, also detects duplicate results.
    assert_eq!(actual, expected);
    assert_eq!(tx.count(()).await.unwrap(), expected.len() as u64);
    assert_eq!(tx.contains(()).await.unwrap(), !expected.is_empty());
    assert_eq!(tx.is_empty().await.unwrap(), expected.is_empty());

    let expected_contexts: BTreeSet<_> = expected
        .iter()
        .filter_map(|q| q.context().cloned())
        .collect();
    let mut contexts = tx.contexts().try_collect::<Vec<_>>().await.unwrap();
    contexts.sort();
    assert_eq!(contexts, expected_contexts.into_iter().collect::<Vec<_>>());
}

async fn assert_closed(tx: &Arc<HeapTransaction>, error: HeapStoreError) {
    assert_eq!(tx.contains(()).await, Err(error));
    assert_eq!(tx.count(()).await, Err(error));
    assert_eq!(tx.is_empty().await, Err(error));
    assert_eq!(tx.r#match(()).try_collect::<Vec<_>>().await, Err(error));
    assert_eq!(tx.contexts().try_collect::<Vec<_>>().await, Err(error));
    let mut alias = tx.clone();
    assert_eq!(alias.insert(quad("late")).await, Err(error));
    assert_eq!(alias.remove(quad("late")).await, Err(error));
    assert_eq!(alias.delete(()).await, Err(error));
    assert_eq!(alias.clear().await, Err(error));
    assert_eq!(alias.clone().commit().await, Err(error));
    assert_eq!(alias.rollback().await, Err(error));
}

#[tokio::test]
async fn overlay_is_a_set_and_last_mutation_wins() {
    let a = quad("a");
    let b = quad("b").with_context(HeapTerm::iri("urn:graph"));
    let mut store = store_with(slice::from_ref(&a)).await;
    let mut tx = store.write().await.unwrap();
    let alias = tx.clone();
    tx.insert(a.clone()).await.unwrap();
    tx.insert(a.clone()).await.unwrap();
    assert_view(&alias, slice::from_ref(&a)).await;
    tx.insert(b.clone()).await.unwrap();
    assert_view(&alias, &[a.clone(), b.clone()]).await;
    assert_view(&store.read().await.unwrap(), slice::from_ref(&a)).await;

    tx.remove(quad("absent")).await.unwrap();
    tx.remove(a.clone()).await.unwrap();
    assert_view(&alias, slice::from_ref(&b)).await;
    tx.insert(a.clone()).await.unwrap();
    tx.remove(b.clone()).await.unwrap();
    assert_view(&alias, slice::from_ref(&a)).await;
    tx.commit().await.unwrap();
    assert_closed(&alias, HeapStoreError::Committed).await;
    assert_view(&store.read().await.unwrap(), &[a]).await;
}

#[tokio::test]
async fn clear_discards_previous_changes_and_allows_new_inserts() {
    let original = [
        quad("a"),
        quad("b").with_context(HeapTerm::iri("urn:graph")),
    ];
    let mut store = store_with(&original).await;
    let mut tx = store.write().await.unwrap();
    tx.remove(original[0].clone()).await.unwrap();
    tx.insert(quad("staged")).await.unwrap();
    tx.clear().await.unwrap();
    assert_view(&tx, &[]).await;
    assert_view(&store.read().await.unwrap(), &original).await;
    tx.insert(quad("discarded-by-second-clear")).await.unwrap();
    tx.clear().await.unwrap();
    let replacement = quad("replacement").with_context(HeapTerm::bnode("graph"));
    tx.insert(replacement.clone()).await.unwrap();
    assert_view(&tx, slice::from_ref(&replacement)).await;
    tx.commit().await.unwrap();
    assert_view(&store.read().await.unwrap(), &[replacement]).await;
}

#[tokio::test]
async fn rollback_discards_clear_and_closes_all_handles() {
    let original = quad("original");
    let mut store = store_with(slice::from_ref(&original)).await;
    let mut tx = store.write().await.unwrap();
    let alias = tx.clone();
    tx.clear().await.unwrap();
    tx.insert(quad("replacement")).await.unwrap();
    tx.rollback().await.unwrap();
    assert_closed(&alias, HeapStoreError::RolledBack).await;
    assert_view(&store.read().await.unwrap(), &[original]).await;
}

#[tokio::test]
async fn delete_removes_both_committed_and_staged_matches() {
    let committed = quad("selected");
    let staged = quad("selected").with_context(HeapTerm::iri("urn:graph"));
    let retained = quad("retained");
    let mut store = store_with(&[committed.clone(), retained.clone()]).await;
    let mut tx = store.write().await.unwrap();
    tx.insert(committed.clone()).await.unwrap();
    tx.insert(staged.clone()).await.unwrap();
    tx.delete(HeapQuadPattern::with_object(HeapTerm::string("selected")))
        .await
        .unwrap();
    assert_view(&tx, slice::from_ref(&retained)).await;
    assert_view(&store.read().await.unwrap(), &[committed, retained.clone()]).await;
    tx.insert(staged.clone()).await.unwrap();
    tx.commit().await.unwrap();
    assert_view(&store.read().await.unwrap(), &[retained, staged]).await;
}

#[tokio::test]
async fn delete_after_clear_only_sees_post_clear_inserts() {
    let mut store = store_with(&[quad("original")]).await;
    let mut tx = store.write().await.unwrap();
    tx.clear().await.unwrap();
    tx.insert(quad("new")).await.unwrap();
    tx.delete(()).await.unwrap();
    assert_view(&tx, &[]).await;
    tx.commit().await.unwrap();
    assert_view(&store.read().await.unwrap(), &[]).await;
}

#[tokio::test]
async fn graph_queries_and_deletes_handle_default_and_named_graphs() {
    let default = quad("value");
    let named = default.clone().with_context(HeapTerm::iri("graph"));
    let blank = default.clone().with_context(HeapTerm::bnode("graph"));
    let other = quad("other").with_context(HeapTerm::iri("graph"));
    let mut store = store_with(&[default.clone(), named.clone()]).await;
    let mut tx = store.write().await.unwrap();
    tx.insert(blank.clone()).await.unwrap();
    tx.insert(other.clone()).await.unwrap();
    assert_view(
        &tx,
        &[default.clone(), named.clone(), blank.clone(), other.clone()],
    )
    .await;

    let pattern = HeapQuadPattern::with_context(HeapTerm::iri("graph"));
    assert!(tx.contains(pattern.clone()).await.unwrap());
    assert_eq!(tx.count(pattern.clone()).await.unwrap(), 2);
    let matches = tx
        .r#match(pattern.clone())
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(
        matches.into_iter().collect::<BTreeSet<_>>(),
        BTreeSet::from([named, other])
    );
    assert!(
        !tx.contains(HeapQuadPattern::with_context(HeapTerm::iri("urn:absent")))
            .await
            .unwrap()
    );
    tx.delete(pattern).await.unwrap();
    assert_view(&tx, &[default.clone(), blank.clone()]).await;
    tx.commit().await.unwrap();
    assert_view(&store.read().await.unwrap(), &[default, blank]).await;
}

#[tokio::test]
async fn matching_and_deletion_compare_complete_terms() {
    let objects = [
        HeapTerm::iri("value"),
        HeapTerm::bnode("value"),
        HeapTerm::string("value"),
        HeapTerm::typed_literal("value", "urn:datatype:a"),
        HeapTerm::typed_literal("value", "urn:datatype:b"),
        HeapTerm::tagged_string("value", "en"),
        HeapTerm::tagged_string("value", "fr"),
        HeapTerm::tagged_string_with_dir("value", "en", BaseDirection::Ltr),
        HeapTerm::tagged_string_with_dir("value", "en", BaseDirection::Rtl),
    ];
    let quads: Vec<_> = objects
        .iter()
        .map(|term| quad("").with_object(term.clone()))
        .collect();
    let mut store = store_with(&quads[..4]).await;
    let mut tx = store.write().await.unwrap();
    for quad in &quads[4..] {
        tx.insert(quad.clone()).await.unwrap();
    }
    for quad in &quads {
        let pattern = HeapQuadPattern::with_object(quad.object().clone());
        assert!(tx.contains(pattern.clone()).await.unwrap());
        assert_eq!(tx.count(pattern.clone()).await.unwrap(), 1);
        assert_eq!(
            tx.r#match(pattern).try_collect::<Vec<_>>().await.unwrap(),
            vec![quad.clone()]
        );
    }
    for pattern in [
        HeapQuadPattern::with_subject(HeapTerm::iri("urn:absent")),
        HeapQuadPattern::with_predicate(HeapTerm::iri("urn:absent")),
    ] {
        assert!(!tx.contains(pattern.clone()).await.unwrap());
        assert_eq!(tx.count(pattern.clone()).await.unwrap(), 0);
        tx.delete(pattern).await.unwrap();
    }
    tx.delete(HeapQuadPattern::with_object(objects[3].clone()))
        .await
        .unwrap();
    let expected: Vec<_> = quads
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != 3)
        .map(|(_, q)| q.clone())
        .collect();
    assert_view(&tx, &expected).await;
    tx.commit().await.unwrap();
    assert_view(&store.read().await.unwrap(), &expected).await;
}

#[tokio::test]
async fn read_only_handles_reject_every_write_operation() {
    let original = quad("original");
    let mut store = store_with(slice::from_ref(&original)).await;
    let mut tx = store.read().await.unwrap();
    assert_eq!(tx.insert(quad("new")).await, Err(HeapStoreError::ReadOnly));
    assert_eq!(
        tx.remove(original.clone()).await,
        Err(HeapStoreError::ReadOnly)
    );
    assert_eq!(tx.delete(()).await, Err(HeapStoreError::ReadOnly));
    assert_eq!(tx.clear().await, Err(HeapStoreError::ReadOnly));
    assert_eq!(tx.clone().commit().await, Err(HeapStoreError::ReadOnly));
    assert_eq!(tx.clone().rollback().await, Err(HeapStoreError::ReadOnly));
    assert_view(&tx, slice::from_ref(&original)).await;
    assert_view(&store.read().await.unwrap(), &[original]).await;
}

#[tokio::test]
async fn dropping_the_last_handle_discards_staged_changes() {
    let mut store = HeapStore::new();
    let mut tx = store.write().await.unwrap();
    tx.insert(quad("staged")).await.unwrap();
    let alias = tx.clone();
    drop(tx);
    assert_view(&alias, &[quad("staged")]).await;
    drop(alias);
    assert_view(&store.read().await.unwrap(), &[]).await;
}

#[tokio::test]
async fn dropping_an_unpolled_rollback_future_still_aborts_all_handles() {
    let mut store = HeapStore::new();
    let mut tx = store.write().await.unwrap();
    tx.insert(quad("staged")).await.unwrap();
    drop(tx.clone().rollback());
    assert_closed(&tx, HeapStoreError::RolledBack).await;
    assert_view(&store.read().await.unwrap(), &[]).await;
}

#[tokio::test]
async fn rollback_cancellation_ends_suspended_matching_and_context_streams() {
    let original = quad("original").with_context(HeapTerm::iri("urn:graph"));
    let mut store = store_with(slice::from_ref(&original)).await;
    for contexts in [false, true] {
        let tx = store.write().await.unwrap();
        let mut stream = if contexts {
            tx.contexts().map_ok(|_| ()).boxed()
        } else {
            tx.r#match(()).map_ok(|_| ()).boxed()
        };
        assert_eq!(stream.next().await, Some(Ok(())));
        // Cleanup waits for the suspended stream's transaction lock.
        assert_eq!(tx.clone().rollback().now_or_never(), None);
        assert_eq!(stream.next().await, Some(Err(HeapStoreError::RolledBack)));
        assert_eq!(stream.next().await, None);
        drop(stream);
        assert_closed(&tx, HeapStoreError::RolledBack).await;
    }
    assert_view(&store.read().await.unwrap(), &[original]).await;
}

#[tokio::test]
async fn cancelling_commit_while_waiting_for_the_store_preserves_staged_changes() {
    let a = quad("a");
    let b = quad("b");
    let c = quad("c");
    let mut store = store_with(slice::from_ref(&a)).await;
    let reader = store.read().await.unwrap();
    let mut stream = reader.r#match(()).boxed();
    assert_eq!(stream.next().await, Some(Ok(a.clone())));
    let mut tx = store.write().await.unwrap();
    tx.insert(b.clone()).await.unwrap();
    assert_eq!(tx.clone().commit().now_or_never(), None);
    // Cancellation releases the transaction write lock as well as its waiter.
    tx.insert(c.clone()).await.unwrap();
    drop(stream);
    assert_view(&reader, slice::from_ref(&a)).await;
    assert_view(&tx, &[a.clone(), b.clone(), c.clone()]).await;
    tx.commit().await.unwrap();
    assert_view(&reader, &[a, b, c]).await;
}

#[tokio::test]
async fn rollback_prevents_a_commit_already_waiting_for_the_store() {
    let original = quad("original");
    let mut store = store_with(slice::from_ref(&original)).await;
    let reader = store.read().await.unwrap();
    let mut stream = reader.r#match(()).boxed();
    assert_eq!(stream.next().await, Some(Ok(original.clone())));
    let mut tx = store.write().await.unwrap();
    tx.clear().await.unwrap();
    tx.insert(quad("replacement")).await.unwrap();
    let mut commit = Box::pin(tx.clone().commit());
    assert_eq!(commit.as_mut().now_or_never(), None);
    let mut rollback = Box::pin(tx.clone().rollback());
    assert_eq!(rollback.as_mut().now_or_never(), None);
    drop(stream);
    assert_eq!(commit.await, Err(HeapStoreError::RolledBack));
    assert_eq!(rollback.await, Ok(()));
    assert_closed(&tx, HeapStoreError::RolledBack).await;
    assert_view(&reader, &[original]).await;
}

#[tokio::test]
async fn unpolled_streams_are_lazy_and_observe_terminal_state_on_first_poll() {
    let mut store = HeapStore::new();
    let mut tx = store.write().await.unwrap();
    tx.insert(quad("staged")).await.unwrap();
    let mut matches = tx.r#match(()).boxed();
    let mut contexts = tx.contexts().boxed();
    tx.clone().commit().await.unwrap();
    assert_eq!(matches.next().await, Some(Err(HeapStoreError::Committed)));
    assert_eq!(matches.next().await, None);
    assert_eq!(contexts.next().await, Some(Err(HeapStoreError::Committed)));
    assert_eq!(contexts.next().await, None);
}

#[tokio::test]
async fn independent_transactions_merge_changes_and_reads_are_live() {
    let mut store = HeapStore::new();
    let reader = store.read().await.unwrap();
    let mut first = store.write().await.unwrap();
    let mut second = store.write().await.unwrap();
    first.insert(quad("a")).await.unwrap();
    second.insert(quad("b")).await.unwrap();
    assert_view(&reader, &[]).await;
    first.commit().await.unwrap();
    assert_view(&reader, &[quad("a")]).await;
    assert_view(&second, &[quad("a"), quad("b")]).await;
    second.commit().await.unwrap();
    assert_view(&reader, &[quad("a"), quad("b")]).await;
}

#[tokio::test]
async fn conflicting_transactions_use_the_last_committed_mutation() {
    for insert_last in [false, true] {
        let value = quad("value");
        let mut store = store_with(slice::from_ref(&value)).await;
        let mut insert = store.write().await.unwrap();
        let mut remove = store.write().await.unwrap();
        insert.insert(value.clone()).await.unwrap();
        remove.remove(value.clone()).await.unwrap();
        if insert_last {
            remove.commit().await.unwrap();
            insert.commit().await.unwrap();
            assert_view(&store.read().await.unwrap(), &[value]).await;
        } else {
            insert.commit().await.unwrap();
            remove.commit().await.unwrap();
            assert_view(&store.read().await.unwrap(), &[]).await;
        }
    }
}

#[tokio::test]
async fn clear_at_commit_removes_intervening_commits_but_delete_does_not() {
    for clear in [false, true] {
        let mut store = store_with(&[quad("original")]).await;
        let mut first = store.write().await.unwrap();
        if clear {
            first.clear().await.unwrap();
        } else {
            first.delete(()).await.unwrap();
        }
        let mut second = store.write().await.unwrap();
        second.insert(quad("intervening")).await.unwrap();
        second.commit().await.unwrap();
        first.insert(quad("replacement")).await.unwrap();
        first.commit().await.unwrap();
        let expected = if clear {
            vec![quad("replacement")]
        } else {
            vec![quad("intervening"), quad("replacement")]
        };
        assert_view(&store.read().await.unwrap(), &expected).await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn racing_commit_and_rollback_have_exactly_one_winner() {
    let mut store = HeapStore::new();
    let mut tx = store.write().await.unwrap();
    tx.insert(quad("staged")).await.unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let commit = {
        let tx = tx.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            tx.commit().await
        })
    };
    let rollback = {
        let tx = tx.clone();
        let barrier = barrier.clone();
        tokio::spawn(async move {
            barrier.wait().await;
            tx.rollback().await
        })
    };
    barrier.wait().await;
    match (commit.await.unwrap(), rollback.await.unwrap()) {
        (Ok(()), Err(HeapStoreError::Committed)) => {
            assert_closed(&tx, HeapStoreError::Committed).await;
            assert_view(&store.read().await.unwrap(), &[quad("staged")]).await;
        },
        (Err(HeapStoreError::RolledBack), Ok(())) => {
            assert_closed(&tx, HeapStoreError::RolledBack).await;
            assert_view(&store.read().await.unwrap(), &[]).await;
        },
        results => panic!("expected exactly one finalization winner: {results:?}"),
    }
}

#[tokio::test]
async fn default_graph_selectors_do_not_match_named_graphs_or_the_reserved_iri() {
    use rdf_model::{DEFAULT_GRAPH, DEFAULT_GRAPH_URN};
    let default = quad("shared");
    let named = default
        .clone()
        .with_context(HeapTerm::iri(DEFAULT_GRAPH_URN));
    let blank = default.clone().with_context(HeapTerm::bnode("graph"));
    let mut store = store_with(&[default.clone(), named.clone(), blank.clone()]).await;
    let mut tx = store.write().await.unwrap();
    // Exercise the const constructor's explicit marker alias, not the normalizing builder.
    tx.insert(HeapQuad::new(
        default.subject().clone(),
        default.predicate().clone(),
        default.object().clone(),
        Some(DEFAULT_GRAPH.into()),
    ))
    .await
    .unwrap();
    assert_view(&tx, &[default.clone(), named.clone(), blank.clone()]).await;
    let exact_default = default.to_quad_pattern();
    assert_eq!(tx.count(exact_default.clone()).await.unwrap(), 1);
    assert!(tx.contains(exact_default.clone()).await.unwrap());
    assert_eq!(
        tx.r#match(exact_default.clone())
            .try_collect::<Vec<_>>()
            .await
            .unwrap(),
        vec![default]
    );
    tx.delete(exact_default).await.unwrap();
    assert!(
        !tx.contains(HeapQuadPattern::with_default_context())
            .await
            .unwrap()
    );
    assert_view(&tx, &[named.clone(), blank.clone()]).await;
    tx.commit().await.unwrap();
    assert_view(&store.read().await.unwrap(), &[named, blank]).await;
}

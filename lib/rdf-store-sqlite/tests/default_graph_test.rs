// This is free and unencumbered software released into the public domain.

#![cfg(feature = "std")]

use futures::TryStreamExt;
use rdf_model::{DEFAULT_GRAPH, DEFAULT_GRAPH_URN, HeapQuad, HeapQuadPattern, HeapTerm};
use rdf_store_sqlite::{ReadTransaction, SqliteError, SqliteStore, Store, WriteTransaction};

#[tokio::test]
async fn default_graph_matching_is_explicit_and_named_graph_writes_are_rejected() {
    let mut store = SqliteStore::new().await.unwrap();
    let mut tx = store.write().await.unwrap();
    let quad = HeapQuad::new(
        HeapTerm::iri("urn:s"),
        HeapTerm::iri("urn:p"),
        HeapTerm::string("o"),
        Some(DEFAULT_GRAPH.into()),
    );
    tx.insert(quad.clone()).await.unwrap();
    assert_eq!(
        tx.count(HeapQuadPattern::with_default_context())
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        tx.r#match(quad.to_quad_pattern())
            .try_collect::<Vec<_>>()
            .await
            .unwrap(),
        vec![quad.clone()]
    );
    let named = quad.with_context(HeapTerm::iri(DEFAULT_GRAPH_URN));
    assert!(!tx.contains(named.to_quad_pattern()).await.unwrap());
    assert!(matches!(
        tx.insert(named).await,
        Err(SqliteError::UnsupportedNamedGraph)
    ));
    tx.rollback().await.unwrap();
}

// This is free and unencumbered software released into the public domain.

#![cfg(feature = "std")]

use rdf_model::{
    DEFAULT_GRAPH, DEFAULT_GRAPH_URN, HeapQuad, HeapQuadPattern, HeapTerm, Statement,
    StatementPattern, Term,
};
use rdf_store_valkey::{ValkeyError, ValkeyQuad, ValkeyTerm, ValkeyTriplePattern};

fn quad(graph: Option<HeapTerm>) -> HeapQuad {
    HeapQuad::new(
        HeapTerm::iri("urn:s"),
        HeapTerm::iri("urn:p"),
        HeapTerm::string("o"),
        graph,
    )
}

#[test]
fn pattern_conversions_preserve_named_and_default_graphs() {
    for graph in [
        None,
        Some(HeapTerm::iri("urn:g")),
        Some(HeapTerm::iri(DEFAULT_GRAPH_URN)),
    ] {
        let original = quad(graph);
        let pattern = ValkeyTriplePattern::from(original.to_quad_pattern());
        assert!(pattern.is_constant());
        let statement = ValkeyQuad::try_from(pattern.clone()).unwrap();
        assert!(pattern.matches_statement(&statement));
        assert_eq!(
            statement.context().map(|g| HeapTerm::try_from(g).unwrap()),
            original.context().cloned()
        );
    }
    assert!(matches!(
        ValkeyQuad::try_from(ValkeyTriplePattern::from(())),
        Err(ValkeyError::UnboundPattern)
    ));
}

#[test]
fn marker_is_not_an_ordinary_urn_term_or_a_wildcard() {
    let marker = ValkeyTerm::from(DEFAULT_GRAPH);
    let iri = ValkeyTerm::from(HeapTerm::iri(DEFAULT_GRAPH_URN));
    assert_ne!(marker, iri);
    assert!(marker.is_default_graph());
    assert!(!iri.is_default_graph());
    assert_eq!(HeapTerm::try_from(&marker).unwrap(), HeapTerm::DefaultGraph);
    let wildcard = ValkeyTriplePattern::from(());
    let default = ValkeyTriplePattern::from(HeapQuadPattern::with_default_context());
    assert!(wildcard.context().is_none());
    assert!(default.context().unwrap().is_default_graph());
    let named = ValkeyQuad::try_from(ValkeyTriplePattern::from(
        quad(Some(HeapTerm::iri(DEFAULT_GRAPH_URN))).to_quad_pattern(),
    ))
    .unwrap();
    assert!(wildcard.matches_statement(&named));
    assert!(!default.matches_statement(&named));
}

#[test]
fn pattern_matching_checks_terms_instead_of_accepting_every_statement() {
    let pattern = ValkeyTriplePattern::from(quad(None).to_quad_pattern());
    let other = quad(None).with_object(HeapTerm::string("different"));
    let candidate =
        ValkeyQuad::try_from(ValkeyTriplePattern::from(other.to_quad_pattern())).unwrap();
    assert!(!pattern.matches_statement(&candidate));
}

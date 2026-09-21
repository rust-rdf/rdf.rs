// This is free and unencumbered software released into the public domain.

use rdf_model::{
    AnyStatement, DEFAULT_GRAPH, DEFAULT_GRAPH_URN, DefaultGraph, Quad, QuadPattern, Statement,
    StatementPattern, Term, TermKind, Triple, TriplePattern,
};
use std::{
    collections::{BTreeSet, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum StaticTerm {
    Iri(&'static str),
    Blank(&'static str),
    Literal(&'static str, &'static str),
    Default,
}

impl Term for StaticTerm {
    fn kind(&self) -> TermKind {
        match self {
            Self::Iri(_) => TermKind::Iri,
            Self::Blank(_) => TermKind::BNode,
            Self::Literal(_, _) => TermKind::Literal,
            Self::Default => TermKind::DefaultGraph,
        }
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> std::borrow::Cow<'_, str> {
        panic!("typed matching must not inspect lexical strings")
    }
}

impl From<DefaultGraph> for StaticTerm {
    fn from(_: DefaultGraph) -> Self {
        Self::Default
    }
}

// Downstream reference implementations remain valid (no conflicting blanket impl).
impl Term for &StaticTerm {
    fn kind(&self) -> TermKind {
        (**self).kind()
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> std::borrow::Cow<'_, str> {
        panic!("typed matching must not inspect lexical strings")
    }
}

fn quad(graph: Option<StaticTerm>) -> Quad<StaticTerm> {
    Quad::new(
        StaticTerm::Iri("urn:s"),
        StaticTerm::Iri("urn:p"),
        StaticTerm::Literal("value", "urn:type"),
        graph,
    )
}

#[test]
fn graph_selector_matrix() {
    let graphs = [
        None,
        Some(StaticTerm::Default),
        Some(StaticTerm::Iri("graph")),
        Some(StaticTerm::Blank("graph")),
        Some(StaticTerm::Iri(DEFAULT_GRAPH_URN)),
    ];
    let patterns = [
        QuadPattern::empty(),
        QuadPattern::with_default_context(),
        QuadPattern::with_context(StaticTerm::Iri("graph")),
        QuadPattern::with_context(StaticTerm::Blank("graph")),
        QuadPattern::with_context(StaticTerm::Iri(DEFAULT_GRAPH_URN)),
    ];
    let expected = [
        [true, true, true, true, true],
        [true, true, false, false, false],
        [false, false, true, false, false],
        [false, false, false, true, false],
        [false, false, false, false, true],
    ];
    for (pattern, expected) in patterns.iter().zip(expected) {
        for (graph, expected) in graphs.iter().zip(expected) {
            let statement = quad(graph.clone());
            assert_eq!(pattern.matches_statement(&statement), expected);
            assert_eq!(
                pattern.matches(
                    statement.subject(),
                    statement.predicate(),
                    statement.object(),
                    graph.as_ref()
                ),
                expected
            );
        }
    }
}

#[test]
fn matching_checks_each_complete_term() {
    let terms = [
        StaticTerm::Iri("same"),
        StaticTerm::Blank("same"),
        StaticTerm::Literal("same", "urn:type:a"),
        StaticTerm::Literal("same", "urn:type:b"),
    ];
    for (expected_index, expected) in terms.iter().enumerate() {
        for (actual_index, actual) in terms.iter().enumerate() {
            let statement = Quad::new(actual.clone(), actual.clone(), actual.clone(), None);
            for pattern in [
                QuadPattern::with_subject(expected.clone()),
                QuadPattern::with_predicate(expected.clone()),
                QuadPattern::with_object(expected.clone()),
            ] {
                assert_eq!(
                    pattern.matches_statement(&statement),
                    expected_index == actual_index
                );
            }
        }
    }
}

#[test]
fn exact_quad_patterns_preserve_graphs_through_all_trait_paths() {
    for graph in [
        None,
        Some(StaticTerm::Default),
        Some(StaticTerm::Iri("urn:g")),
        Some(StaticTerm::Blank("g")),
    ] {
        let statement = quad(graph);
        let pattern = statement.to_quad_pattern();
        assert!(pattern.is_constant());
        assert!(pattern.matches_statement(&statement));
        assert_eq!(pattern, Statement::to_quad_pattern(&statement));
        assert_eq!(pattern, StatementPattern::to_quad_pattern(&statement));
        assert_eq!(Quad::try_from(pattern.clone()), Ok(statement.clone()));
        for candidate in [
            quad(None),
            quad(Some(StaticTerm::Iri("urn:g"))),
            quad(Some(StaticTerm::Blank("g"))),
        ] {
            assert_eq!(
                StatementPattern::matches_statement(&statement, &candidate),
                statement == candidate
            );
        }
    }
}

#[test]
fn graphless_triple_patterns_keep_their_wildcard() {
    let triple = quad(None).to_triple();
    let pattern = triple.to_quad_pattern();
    assert!(StatementPattern::context(&pattern).is_none());
    assert_eq!(pattern, Statement::to_quad_pattern(&triple));
    assert_eq!(pattern, StatementPattern::to_quad_pattern(&triple));
    assert_eq!(pattern, triple.to_triple_pattern().to_quad_pattern());
    assert!(pattern.matches_statement(&quad(Some(StaticTerm::Iri("urn:g")))));
    assert!(Quad::try_from(pattern).is_err());
    assert!(triple.to_quad().to_quad_pattern().is_default_graph());
}

#[test]
fn legacy_constructor_and_extraction_shapes_are_preserved() {
    let wildcard: QuadPattern<StaticTerm> = (None::<StaticTerm>, None, None, None).into();
    assert!(wildcard.is_empty());
    assert_eq!(wildcard.into_inner(), (None, None, None, None));
    let default = QuadPattern::<StaticTerm>::with_context(DEFAULT_GRAPH.into());
    assert!(!default.is_empty());
    assert!(!default.is_constant());
    assert!(default.has_context());
    assert!(default.is_default_graph());
    assert_eq!(default.into_inner().3, Some(StaticTerm::Default));
}

#[test]
fn default_quad_alias_has_consistent_equality_hash_order_and_extraction() {
    let implicit = quad(None);
    let explicit = quad(Some(StaticTerm::Default));
    assert_eq!(implicit, explicit);
    assert_eq!(implicit.cmp(&explicit), std::cmp::Ordering::Equal);
    let hash = |q: &Quad<StaticTerm>| {
        let mut state = DefaultHasher::new();
        q.hash(&mut state);
        state.finish()
    };
    assert_eq!(hash(&implicit), hash(&explicit));
    assert_eq!(
        BTreeSet::from([implicit.clone(), explicit.clone()]).len(),
        1
    );
    assert!(explicit.context().is_none());
    assert!(explicit.into_inner().3.is_none());
    assert!(
        implicit
            .with_context(StaticTerm::Default)
            .context()
            .is_none()
    );
}

#[test]
fn any_statement_keeps_polymorphic_wildcard_conveniences() {
    for statement in [quad(None), quad(Some(StaticTerm::Iri("urn:g")))] {
        assert!(AnyStatement.matches_statement(&statement));
        assert!(AnyStatement.matches(
            statement.subject(),
            statement.predicate(),
            statement.object(),
            statement.context()
        ));
        let typed: QuadPattern<StaticTerm> = AnyStatement.into();
        assert!(typed.matches_statement(&statement));
        let typed: TriplePattern<StaticTerm> = AnyStatement.into();
        assert!(typed.matches_statement(&statement));
    }
}

#[test]
fn default_graph_is_an_allocation_free_marker_with_explicit_urn_decoding() {
    assert_eq!(std::mem::size_of::<DefaultGraph>(), 0);
    assert_eq!(DEFAULT_GRAPH.kind(), TermKind::DefaultGraph);
    assert!(DEFAULT_GRAPH.is_default_graph());
    assert!(!DEFAULT_GRAPH.is_iri());
    assert_eq!(DEFAULT_GRAPH.to_string(), "urn:rdf:default-graph");
    assert_eq!(
        DefaultGraph::from_urn(DEFAULT_GRAPH_URN),
        Some(DEFAULT_GRAPH)
    );
    assert_eq!(DefaultGraph::from_urn("urn:other"), None);
}

#[derive(Debug, Eq, PartialEq)]
struct BorrowOnly(&'static str);

impl Clone for BorrowOnly {
    fn clone(&self) -> Self {
        panic!("matching must not clone terms")
    }
}

impl Term for BorrowOnly {
    fn kind(&self) -> TermKind {
        TermKind::Iri
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> std::borrow::Cow<'_, str> {
        panic!("matching must not stringify terms")
    }
}

#[test]
fn matching_borrows_without_cloning_or_stringifying() {
    let pattern = TriplePattern::with_subject(BorrowOnly("s"));
    let statement = Triple::new(BorrowOnly("s"), BorrowOnly("p"), BorrowOnly("o"));
    assert!(pattern.matches_statement(&statement));
}

#[cfg(feature = "alloc")]
mod owned {
    use super::*;
    use rdf_model::{BaseDirection, CowQuad, CowTerm, HeapQuad, HeapQuadPattern, HeapTerm};

    #[test]
    fn literal_metadata_is_part_of_typed_equality() {
        let terms = [
            HeapTerm::string("value"),
            HeapTerm::typed_literal("value", "urn:a"),
            HeapTerm::typed_literal("value", "urn:b"),
            HeapTerm::tagged_string("value", "en"),
            HeapTerm::tagged_string("value", "fr"),
            HeapTerm::tagged_string_with_dir("value", "en", BaseDirection::Ltr),
            HeapTerm::tagged_string_with_dir("value", "en", BaseDirection::Rtl),
        ];
        for (i, term) in terms.iter().enumerate() {
            let pattern = HeapQuadPattern::with_object(term.clone());
            for (j, actual) in terms.iter().enumerate() {
                assert_eq!(pattern.matches(actual, actual, actual, None), i == j);
            }
        }
    }

    #[test]
    fn heap_and_cow_owned_and_borrowed_quad_roundtrips_keep_graphs() {
        for graph in [
            None,
            Some(HeapTerm::DefaultGraph),
            Some(HeapTerm::iri("urn:g")),
            Some(HeapTerm::bnode("g")),
            Some(HeapTerm::iri(DEFAULT_GRAPH_URN)),
        ] {
            let original = HeapQuad::new(
                HeapTerm::iri("urn:s"),
                HeapTerm::iri("urn:p"),
                HeapTerm::tagged_string_with_dir("text", "en", BaseDirection::Rtl),
                graph,
            );
            let moved: CowQuad<'_> = original.clone().into();
            let borrowed: CowQuad<'_> = (&original).into();
            for converted in [moved, borrowed] {
                assert_eq!(HeapQuad::from(&converted), original);
                assert_eq!(HeapQuad::from(converted), original);
            }
        }
    }

    #[test]
    fn marker_and_ordinary_urn_are_distinct_terms_and_selectors() {
        let marker: HeapTerm = DEFAULT_GRAPH.into();
        let iri = HeapTerm::iri(DEFAULT_GRAPH_URN);
        assert_eq!(marker.value_str(), iri.value_str());
        assert_ne!(marker, iri);
        assert_ne!(
            CowTerm::from(DEFAULT_GRAPH),
            CowTerm::iri(DEFAULT_GRAPH_URN)
        );
        assert_eq!(HeapTerm::from(CowTerm::from(DEFAULT_GRAPH)), marker);
        let named = HeapQuad::new(iri.clone(), iri.clone(), iri.clone(), Some(iri));
        assert!(!HeapQuadPattern::with_default_context().matches_statement(&named));
        assert!(
            HeapQuadPattern::with_context(HeapTerm::iri(DEFAULT_GRAPH_URN))
                .matches_statement(&named)
        );
        assert!(AnyStatement::try_from(HeapQuadPattern::with_default_context()).is_err());
    }

    #[test]
    fn borrowed_builtin_quads_can_create_exact_default_graph_patterns() {
        let term = HeapTerm::iri("urn:term");
        let quad = Quad::new(&term, &term, &term, None);
        assert!(quad.to_quad_pattern().is_default_graph());
        assert!(quad.to_quad_pattern().matches_statement(&quad));
        let term = CowTerm::iri("urn:term");
        let quad = Quad::new(&term, &term, &term, None);
        assert!(quad.to_quad_pattern().is_default_graph());
        assert!(quad.to_quad_pattern().matches_statement(&quad));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_retains_legacy_patterns_and_distinguishes_default_selection() {
        let legacy = serde_json::json!({"s": null, "p": null, "o": null, "g": null});
        let any: HeapQuadPattern = serde_json::from_value(legacy.clone()).unwrap();
        assert!(any.is_empty());
        assert_eq!(serde_json::to_value(&any).unwrap(), legacy);
        let default = HeapQuadPattern::with_default_context();
        let encoded = serde_json::to_value(&default).unwrap();
        assert_eq!(encoded["g"], "DefaultGraph");
        assert_eq!(
            serde_json::from_value::<HeapQuadPattern>(encoded).unwrap(),
            default
        );
        let named = HeapQuadPattern::with_context(HeapTerm::iri(DEFAULT_GRAPH_URN));
        let encoded = serde_json::to_value(&named).unwrap();
        assert_eq!(encoded["g"], serde_json::json!({"Iri": DEFAULT_GRAPH_URN}));
        assert_eq!(
            serde_json::from_value::<HeapQuadPattern>(encoded).unwrap(),
            named
        );

        assert_eq!(
            serde_json::to_value(Some(DEFAULT_GRAPH)).unwrap(),
            DEFAULT_GRAPH_URN
        );
        assert_eq!(
            serde_json::to_value(None::<DefaultGraph>).unwrap(),
            serde_json::Value::Null
        );
        assert_eq!(
            serde_json::from_str::<DefaultGraph>("\"urn:rdf:default-graph\"").unwrap(),
            DEFAULT_GRAPH
        );
        assert!(serde_json::from_str::<DefaultGraph>("\"urn:other\"").is_err());

        let quad = HeapQuad::new(
            HeapTerm::iri("urn:s"),
            HeapTerm::iri("urn:p"),
            HeapTerm::string("o"),
            Some(DEFAULT_GRAPH.into()),
        );
        let encoded = serde_json::to_value(&quad).unwrap();
        assert!(encoded["g"].is_null());
        assert_eq!(serde_json::from_value::<HeapQuad>(encoded).unwrap(), quad);
        assert_eq!(
            HeapTerm::from(DEFAULT_GRAPH).into_json(),
            serde_json::json!(DEFAULT_GRAPH_URN)
        );
        assert_eq!(
            HeapTerm::try_from(serde_json::json!(DEFAULT_GRAPH_URN)).unwrap(),
            HeapTerm::iri(DEFAULT_GRAPH_URN)
        );
    }

    #[cfg(feature = "oxrdf")]
    #[test]
    fn oxrdf_preserves_native_default_graph_and_uses_urn_only_at_term_boundary() {
        use rdf_model::interop::OxrdfTerm;
        let marker = OxrdfTerm::from(oxrdf::GraphName::DefaultGraph);
        let named = OxrdfTerm::from(oxrdf::NamedNode::new(DEFAULT_GRAPH_URN).unwrap());
        assert!(marker.is_default_graph());
        assert_ne!(marker, named);
        assert_eq!(HeapTerm::from(marker.clone()), HeapTerm::DefaultGraph);
        assert_eq!(CowTerm::from(marker.clone()), CowTerm::DefaultGraph);
        assert_eq!(marker.into_inner(), named.into_inner());
        assert_eq!(
            oxrdf::GraphName::from(DEFAULT_GRAPH),
            oxrdf::GraphName::DefaultGraph
        );
        let q = Quad::new(
            OxrdfTerm::from(HeapTerm::iri("urn:s")),
            OxrdfTerm::from(HeapTerm::iri("urn:p")),
            OxrdfTerm::from(HeapTerm::string("o")),
            None,
        );
        assert!(q.to_quad_pattern().is_default_graph());
        assert!(q.to_quad_pattern().matches_statement(&q));
    }
}

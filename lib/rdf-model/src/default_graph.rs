// This is free and unencumbered software released into the public domain.

use crate::{Term, TermKind};

/// The reserved default-graph identifier for term-only external protocols.
///
/// Prefer a protocol's native default-graph representation when available.
/// This URN is a boundary convention, not an ordinary IRI that the model
/// automatically reclassifies: an IRI containing this string is distinct from
/// [`DEFAULT_GRAPH`] internally. A protocol using this convention must reserve
/// the URN in its graph-name namespace.
pub const DEFAULT_GRAPH_URN: &str = "urn:rdf:default-graph";

/// The singleton default-graph marker.
///
/// In a pattern, `None` selects any graph, `Some(DEFAULT_GRAPH.into())` selects
/// the default graph, and another term selects that named graph. In a concrete
/// quad, `None` is the canonical default-graph representation.
pub const DEFAULT_GRAPH: DefaultGraph = DefaultGraph;

/// An allocation-free singleton denoting the default graph.
///
/// Use this marker in graph slots, not as an ordinary RDF subject, predicate,
/// or object. It has its own [`TermKind::DefaultGraph`] identity. [`Display`](core::fmt::Display)
/// and Serde use [`DEFAULT_GRAPH_URN`] for external representation; decoding an
/// IRI as this marker requires the explicit [`Self::from_urn`] convention.
/// Built-in owned/Cow terms implement `From<DefaultGraph>` without allocating.
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "alloc")] {
/// use rdf_model::{DEFAULT_GRAPH, HeapQuad, HeapQuadPattern, HeapTerm, StatementPattern};
/// let pattern = HeapQuadPattern::new(None, None, None, Some(DEFAULT_GRAPH.into()));
/// let quad = HeapQuad::new(HeapTerm::iri("urn:s"), HeapTerm::iri("urn:p"), HeapTerm::string("o"), None);
/// assert!(pattern.matches_statement(&quad));
/// assert!(!pattern.matches_statement(&quad.with_context(HeapTerm::iri("urn:g"))));
/// # }
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
pub struct DefaultGraph;

impl DefaultGraph {
    /// Recognizes the exact reserved URN at a boundary that opts into this
    /// convention. Returns `None` for any other identifier.
    pub fn from_urn(urn: &str) -> Option<Self> {
        (urn == DEFAULT_GRAPH_URN).then_some(Self)
    }
}

impl Term for DefaultGraph {
    fn kind(&self) -> TermKind {
        TermKind::DefaultGraph
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        alloc::borrow::Cow::Borrowed(DEFAULT_GRAPH_URN)
    }
}

impl core::fmt::Display for DefaultGraph {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(DEFAULT_GRAPH_URN)
    }
}

impl Term for &DefaultGraph {
    fn kind(&self) -> TermKind {
        TermKind::DefaultGraph
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        alloc::borrow::Cow::Borrowed(DEFAULT_GRAPH_URN)
    }
}

impl From<DefaultGraph> for &DefaultGraph {
    fn from(_: DefaultGraph) -> Self {
        &DEFAULT_GRAPH
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for DefaultGraph {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(DEFAULT_GRAPH_URN)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for DefaultGraph {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let urn = <alloc::string::String as serde::Deserialize>::deserialize(deserializer)?;
        Self::from_urn(&urn)
            .ok_or_else(|| serde::de::Error::custom("expected urn:rdf:default-graph"))
    }
}

#[cfg(feature = "oxrdf")]
impl From<DefaultGraph> for oxrdf::GraphName {
    fn from(_: DefaultGraph) -> Self {
        Self::DefaultGraph
    }
}

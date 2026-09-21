// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyGraphKey, ValkeyTerm, ValkeyTripleId};
use alloc::string::ToString;
use rdf_model::{HeapTerm, QuadPattern, StatementPattern, Term};

/// A triple statement pattern for matching triples in Valkey.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct ValkeyTriplePattern {
    pub(crate) glob: ValkeyTripleId,
    pub(crate) matcher: QuadPattern<ValkeyTerm>,
}

impl ValkeyTriplePattern {
    pub fn is_constant(&self) -> bool {
        self.matcher.is_constant()
    }

    pub(crate) fn graph_key_and_context(
        &self,
    ) -> Result<(ValkeyGraphKey, Option<ValkeyTerm>), ValkeyError> {
        let graph = self.context().ok_or(ValkeyError::UnsupportedGraphPattern)?;
        if graph.is_default_graph() {
            return Ok((ValkeyGraphKey::default(), None));
        }
        if graph.is_iri() {
            if let Some(name) = graph.0.as_str().filter(|name| *name != "default") {
                return Ok((ValkeyGraphKey::from(name.to_string()), Some(graph.clone())));
            }
        }
        Err(ValkeyError::UnsupportedGraphPattern)
    }
}

impl<T> From<&T> for ValkeyTriplePattern
where
    T: Clone + Into<Self>,
{
    fn from(t: &T) -> Self {
        t.clone().into()
    }
}

impl From<()> for ValkeyTriplePattern {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<Option<()>> for ValkeyTriplePattern {
    fn from(_: Option<()>) -> Self {
        Self::default()
    }
}

impl From<QuadPattern<ValkeyTerm>> for ValkeyTriplePattern {
    fn from(input: QuadPattern<ValkeyTerm>) -> Self {
        let (s, p, o, g) = input.into_inner();
        Self {
            glob: ValkeyTripleId(
                s.as_ref().and_then(|s| s.try_into().ok()),
                p.as_ref().and_then(|p| p.try_into().ok()),
                o.as_ref().and_then(|o| o.try_into().ok()),
            ),
            matcher: QuadPattern::new(
                s.map(|s| s.into()),
                p.map(|p| p.into()),
                o.map(|o| o.into()),
                g,
            ),
        }
    }
}

impl From<QuadPattern<HeapTerm>> for ValkeyTriplePattern {
    fn from(input: QuadPattern<HeapTerm>) -> Self {
        let (s, p, o, g) = input.into_inner();
        Self {
            glob: ValkeyTripleId(
                s.as_ref().map(|s| s.into()),
                p.as_ref().map(|p| p.into()),
                o.as_ref().map(|o| o.into()),
            ),
            matcher: QuadPattern::new(
                s.map(|s| s.into()),
                p.map(|p| p.into()),
                o.map(|o| o.into()),
                g.map(|graph| match graph {
                    // Retain its kind so unsupported blank-node selectors are
                    // rejected rather than silently treated as IRI graph names.
                    HeapTerm::BNode(id) => {
                        ValkeyTerm(serde_json::Value::String(alloc::format!("_:{id}")), false)
                    },
                    graph => graph.into(),
                }),
            ),
        }
    }
}

impl From<ValkeyTriplePattern> for fred::bytes_utils::Str {
    fn from(input: ValkeyTriplePattern) -> Self {
        input.glob.to_string().into()
    }
}

impl From<ValkeyTriplePattern> for fred::types::Value {
    fn from(input: ValkeyTriplePattern) -> Self {
        input.glob.into()
    }
}

impl From<&ValkeyTriplePattern> for fred::types::Value {
    fn from(input: &ValkeyTriplePattern) -> Self {
        input.glob.clone().into()
    }
}

impl StatementPattern for ValkeyTriplePattern {
    type Term = ValkeyTerm;

    fn subject(&self) -> Option<&Self::Term> {
        self.matcher.subject()
    }

    fn predicate(&self) -> Option<&Self::Term> {
        self.matcher.predicate()
    }

    fn object(&self) -> Option<&Self::Term> {
        self.matcher.object()
    }

    fn context(&self) -> Option<&Self::Term> {
        self.matcher.context()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rdf_model::{DEFAULT_GRAPH_URN, HeapQuadPattern};

    #[test]
    fn unsupported_graphs_are_errors_and_the_reserved_urn_stays_named() {
        for pattern in [
            HeapQuadPattern::empty(),
            HeapQuadPattern::with_context(HeapTerm::bnode("graph")),
            HeapQuadPattern::with_context(HeapTerm::iri("default")),
            HeapQuadPattern::with_context(HeapTerm::string("graph")),
        ] {
            assert!(matches!(
                ValkeyTriplePattern::from(pattern).graph_key_and_context(),
                Err(ValkeyError::UnsupportedGraphPattern)
            ));
        }
        let (default_key, context) =
            ValkeyTriplePattern::from(HeapQuadPattern::with_default_context())
                .graph_key_and_context()
                .unwrap();
        assert!(context.is_none());
        let (named_key, context) = ValkeyTriplePattern::from(HeapQuadPattern::with_context(
            HeapTerm::iri(DEFAULT_GRAPH_URN),
        ))
        .graph_key_and_context()
        .unwrap();
        assert!(context.is_some());
        assert_ne!(default_key, named_key);
    }
}

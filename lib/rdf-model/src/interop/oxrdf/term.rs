// This is free and unencumbered software released into the public domain.

use crate::{
    CowTerm, DEFAULT_GRAPH_URN, DefaultGraph, HeapQuad, HeapTerm, Statement, Term, TermKind,
};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use core::fmt::Debug;
use oxrdf::Quad;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// An Oxrdf term or the distinct default-graph singleton.
/// The marker is kept separate from an ordinary named node with the reserved URN.
pub struct OxrdfTerm(pub(crate) Option<oxrdf::Term>);

impl OxrdfTerm {
    /// Exports an Oxrdf term. Since this target type has no default-graph variant,
    /// the singleton becomes the reserved `urn:rdf:default-graph` named node.
    /// Importing that named node does not implicitly recreate the singleton.
    pub fn into_inner(self) -> oxrdf::Term {
        self.0
            .unwrap_or_else(|| oxrdf::NamedNode::new_unchecked(DEFAULT_GRAPH_URN).into())
    }
}

impl Term for OxrdfTerm {
    fn kind(&self) -> TermKind {
        use oxrdf::Term;
        let Some(inner) = &self.0 else {
            return TermKind::DefaultGraph;
        };
        match inner {
            Term::NamedNode(_) => TermKind::Iri,
            Term::BlankNode(_) => TermKind::BNode,
            Term::Literal(_) => TermKind::Literal,
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }

    fn value_str(&self) -> Cow<'_, str> {
        use oxrdf::Term;
        let Some(inner) = &self.0 else {
            return Cow::Borrowed(DEFAULT_GRAPH_URN);
        };
        match inner {
            Term::NamedNode(node) => Cow::Borrowed(node.as_str()),
            Term::BlankNode(node) => Cow::Borrowed(node.as_str()),
            Term::Literal(lit) => Cow::Borrowed(lit.value()), // TODO
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}

impl From<CowTerm<'_>> for OxrdfTerm {
    fn from(input: CowTerm<'_>) -> Self {
        Self(if input.is_default_graph() {
            None
        } else {
            Some(input.into())
        })
    }
}

impl From<HeapTerm> for OxrdfTerm {
    fn from(input: HeapTerm) -> Self {
        Self(if input.is_default_graph() {
            None
        } else {
            Some(input.into())
        })
    }
}

impl From<oxrdf::GraphName> for OxrdfTerm {
    fn from(input: oxrdf::GraphName) -> Self {
        use oxrdf::GraphName;
        Self(match input {
            GraphName::NamedNode(n) => Some(n.into()),
            GraphName::BlankNode(n) => Some(n.into()),
            GraphName::DefaultGraph => None,
        })
    }
}

impl From<oxrdf::NamedOrBlankNode> for OxrdfTerm {
    fn from(input: oxrdf::NamedOrBlankNode) -> Self {
        Self(Some(input.into()))
    }
}

impl From<oxrdf::NamedNode> for OxrdfTerm {
    fn from(input: oxrdf::NamedNode) -> Self {
        Self(Some(input.into()))
    }
}

impl From<oxrdf::BlankNode> for OxrdfTerm {
    fn from(input: oxrdf::BlankNode) -> Self {
        Self(Some(input.into()))
    }
}

impl From<oxrdf::Literal> for OxrdfTerm {
    fn from(input: oxrdf::Literal) -> Self {
        Self(Some(input.into()))
    }
}

impl<'a> From<OxrdfTerm> for CowTerm<'a> {
    fn from(input: OxrdfTerm) -> Self {
        input.0.map(Into::into).unwrap_or(Self::DefaultGraph)
    }
}

impl From<OxrdfTerm> for HeapTerm {
    fn from(input: OxrdfTerm) -> Self {
        input.0.map(Into::into).unwrap_or(Self::DefaultGraph)
    }
}

impl From<DefaultGraph> for OxrdfTerm {
    fn from(_: DefaultGraph) -> Self {
        Self(None)
    }
}

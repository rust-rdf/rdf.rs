// This is free and unencumbered software released into the public domain.

use crate::{CowTerm, HeapQuad, HeapTerm, Statement, Term, TermKind};
use alloc::{
    borrow::Cow,
    string::{String, ToString},
};
use core::fmt::Debug;
use oxrdf::Quad;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OxrdfTerm(pub(crate) oxrdf::Term);

impl OxrdfTerm {
    pub fn into_inner(self) -> oxrdf::Term {
        self.0
    }
}

impl Term for OxrdfTerm {
    fn kind(&self) -> TermKind {
        use oxrdf::Term;
        match &self.0 {
            Term::NamedNode(_) => TermKind::Iri,
            Term::BlankNode(_) => TermKind::BNode,
            Term::Literal(_) => TermKind::Literal,
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }

    fn value_str(&self) -> Cow<'_, str> {
        use oxrdf::Term;
        match &self.0 {
            Term::NamedNode(node) => Cow::Borrowed(node.as_str()),
            Term::BlankNode(node) => Cow::Borrowed(node.as_str()),
            Term::Literal(lit) => Cow::Borrowed(lit.value()), // TODO
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}

impl From<CowTerm<'_>> for OxrdfTerm {
    fn from(input: CowTerm<'_>) -> Self {
        Self(input.into())
    }
}

impl From<HeapTerm> for OxrdfTerm {
    fn from(input: HeapTerm) -> Self {
        Self(input.into())
    }
}

impl From<oxrdf::GraphName> for OxrdfTerm {
    fn from(input: oxrdf::GraphName) -> Self {
        use oxrdf::GraphName;
        Self(match input {
            GraphName::NamedNode(n) => n.into(),
            GraphName::BlankNode(n) => n.into(),
            GraphName::DefaultGraph => unreachable!(),
        })
    }
}

impl From<oxrdf::NamedOrBlankNode> for OxrdfTerm {
    fn from(input: oxrdf::NamedOrBlankNode) -> Self {
        Self(input.into())
    }
}

impl From<oxrdf::NamedNode> for OxrdfTerm {
    fn from(input: oxrdf::NamedNode) -> Self {
        Self(input.into())
    }
}

impl From<oxrdf::BlankNode> for OxrdfTerm {
    fn from(input: oxrdf::BlankNode) -> Self {
        Self(input.into())
    }
}

impl From<oxrdf::Literal> for OxrdfTerm {
    fn from(input: oxrdf::Literal) -> Self {
        Self(input.into())
    }
}

impl<'a> From<OxrdfTerm> for CowTerm<'a> {
    fn from(input: OxrdfTerm) -> Self {
        input.0.into()
    }
}

impl From<OxrdfTerm> for HeapTerm {
    fn from(input: OxrdfTerm) -> Self {
        input.0.into()
    }
}

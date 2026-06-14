// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use crate::{Statement, Term, TermKind};
use alloc::{borrow::Cow, string::String};
use core::fmt::Debug;
use oxrdf::Quad;

#[derive(Clone)]
pub struct OxrdfStatement {
    subject: OxrdfTerm,
    predicate: OxrdfTerm,
    object: OxrdfTerm,
    context: Option<OxrdfTerm>,
}

impl core::fmt::Debug for OxrdfStatement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OxrdfStatement")
            .field(&self.subject.value_str())
            .field(&self.predicate.value_str())
            .field(&self.object.value_str())
            .field(&self.context.as_ref().map(|c| c.value_str()))
            .finish()
    }
}

impl From<oxrdf::Quad> for OxrdfStatement {
    fn from(quad: oxrdf::Quad) -> Self {
        use oxrdf::GraphName;
        Self {
            subject: quad.subject.into(),
            predicate: quad.predicate.into(),
            object: quad.object.into(),
            context: match quad.graph_name {
                GraphName::NamedNode(node) => Some(node.into()),
                GraphName::BlankNode(node) => Some(node.into()),
                GraphName::DefaultGraph => None,
            },
        }
    }
}

impl From<oxrdf::Triple> for OxrdfStatement {
    fn from(quad: oxrdf::Triple) -> Self {
        Self {
            subject: quad.subject.into(),
            predicate: quad.predicate.into(),
            object: quad.object.into(),
            context: None,
        }
    }
}

impl Statement for OxrdfStatement {
    type Term = OxrdfTerm;

    fn subject(&self) -> &Self::Term {
        &self.subject
    }

    fn predicate(&self) -> &Self::Term {
        &self.predicate
    }

    fn object(&self) -> &Self::Term {
        &self.object
    }

    fn context(&self) -> Option<&Self::Term> {
        self.context.as_ref()
    }
}

#[derive(Clone)]
pub struct OxrdfTerm(oxrdf::Term);

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

impl From<oxrdf::Term> for OxrdfTerm {
    fn from(input: oxrdf::Term) -> Self {
        Self(input)
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

#[derive(Clone)]
pub struct OxrdfSubject(oxrdf::NamedOrBlankNode);

impl Term for OxrdfSubject {
    fn kind(&self) -> TermKind {
        use oxrdf::NamedOrBlankNode;
        match &self.0 {
            NamedOrBlankNode::NamedNode(_) => TermKind::Iri,
            NamedOrBlankNode::BlankNode(_) => TermKind::BNode,
        }
    }

    fn value_str(&self) -> Cow<'_, str> {
        use oxrdf::NamedOrBlankNode;
        match &self.0 {
            NamedOrBlankNode::NamedNode(node) => Cow::Borrowed(node.as_str()),
            NamedOrBlankNode::BlankNode(node) => Cow::Borrowed(node.as_str()),
        }
    }
}

#[derive(Clone)]
pub struct OxrdfNamedNode(oxrdf::NamedNode);

impl Term for OxrdfNamedNode {
    fn kind(&self) -> TermKind {
        TermKind::Iri
    }

    fn value_str(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.0.as_str())
    }
}

#[derive(Clone)]
pub struct OxrdfGraphName(oxrdf::GraphName);

impl Term for OxrdfGraphName {
    fn kind(&self) -> TermKind {
        use oxrdf::GraphName;
        match &self.0 {
            GraphName::NamedNode(_) => TermKind::Iri,
            GraphName::BlankNode(_) => TermKind::BNode,
            GraphName::DefaultGraph => todo!(), // TODO
        }
    }

    fn value_str(&self) -> Cow<'_, str> {
        use oxrdf::GraphName;
        match &self.0 {
            GraphName::NamedNode(node) => Cow::Borrowed(node.as_str()),
            GraphName::BlankNode(node) => Cow::Borrowed(node.as_str()),
            GraphName::DefaultGraph => Cow::Borrowed(""), // TODO
        }
    }
}

// This is free and unencumbered software released into the public domain.

use crate::{HeapQuad, HeapTerm, Statement, Term, TermKind};
use alloc::{borrow::Cow, string::String};
use core::fmt::Debug;
use oxrdf::Quad;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

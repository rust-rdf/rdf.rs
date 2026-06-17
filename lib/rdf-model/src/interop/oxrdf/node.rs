// This is free and unencumbered software released into the public domain.

use crate::{Term, TermKind};
use alloc::borrow::Cow;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OxrdfNamedNode(oxrdf::NamedNode);

impl Term for OxrdfNamedNode {
    fn kind(&self) -> TermKind {
        TermKind::Iri
    }

    fn value_str(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.0.as_str())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

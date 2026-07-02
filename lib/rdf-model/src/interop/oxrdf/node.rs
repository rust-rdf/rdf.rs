// This is free and unencumbered software released into the public domain.

use crate::{Term, TermKind};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OxrdfNamedNode(oxrdf::NamedNode);

impl Term for OxrdfNamedNode {
    fn kind(&self) -> TermKind {
        TermKind::Iri
    }

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        use alloc::borrow::Cow;
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

    #[cfg(feature = "alloc")]
    fn value_str(&self) -> alloc::borrow::Cow<'_, str> {
        use alloc::borrow::Cow;
        use oxrdf::GraphName;
        match &self.0 {
            GraphName::NamedNode(node) => Cow::Borrowed(node.as_str()),
            GraphName::BlankNode(node) => Cow::Borrowed(node.as_str()),
            GraphName::DefaultGraph => Cow::Borrowed(""), // TODO
        }
    }
}

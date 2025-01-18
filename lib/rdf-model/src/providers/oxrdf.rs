// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{Statement, Term, TermKind};
use alloc::{borrow::Cow, string::String};
use oxrdf::Quad;

pub struct OxrdfStatement {
    subject: OxrdfSubject,
    predicate: OxrdfNamedNode,
    object: OxrdfTerm,
    context: OxrdfGraphName,
}

impl OxrdfStatement {
    pub fn new(quad: oxrdf::Quad) -> OxrdfStatement {
        Self {
            subject: OxrdfSubject {
                inner: quad.subject,
            },
            predicate: OxrdfNamedNode {
                inner: quad.predicate,
            },
            object: OxrdfTerm { inner: quad.object },
            context: OxrdfGraphName {
                inner: quad.graph_name,
            },
        }
    }
}

impl Statement for OxrdfStatement {
    fn subject(&self) -> &dyn Term {
        &self.subject
    }

    fn predicate(&self) -> &dyn Term {
        &self.predicate
    }

    fn object(&self) -> &dyn Term {
        &self.object
    }

    fn context(&self) -> Option<&dyn Term> {
        Some(&self.context)
    }
}

pub struct OxrdfSubject {
    inner: oxrdf::Subject,
}

impl Term for OxrdfSubject {
    fn kind(&self) -> TermKind {
        use oxrdf::Subject;
        match &self.inner {
            Subject::NamedNode(_) => TermKind::Iri,
            Subject::BlankNode(_) => TermKind::BNode,
            #[cfg(feature = "rdf-star")]
            Subject::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }

    fn as_str(&self) -> Cow<str> {
        use oxrdf::Subject;
        match &self.inner {
            Subject::NamedNode(node) => Cow::Borrowed(node.as_str()),
            Subject::BlankNode(node) => Cow::Borrowed(node.as_str()),
            #[cfg(feature = "rdf-star")]
            Subject::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}

pub struct OxrdfNamedNode {
    inner: oxrdf::NamedNode,
}

impl Term for OxrdfNamedNode {
    fn kind(&self) -> TermKind {
        TermKind::Iri
    }

    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self.inner.as_str())
    }
}

pub struct OxrdfTerm {
    inner: oxrdf::Term,
}

impl Term for OxrdfTerm {
    fn kind(&self) -> TermKind {
        use oxrdf::Term;
        match &self.inner {
            Term::NamedNode(_) => TermKind::Iri,
            Term::BlankNode(_) => TermKind::BNode,
            Term::Literal(_) => TermKind::Literal,
            #[cfg(feature = "rdf-star")]
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }

    fn as_str(&self) -> Cow<str> {
        use oxrdf::Term;
        match &self.inner {
            Term::NamedNode(node) => Cow::Borrowed(node.as_str()),
            Term::BlankNode(node) => Cow::Borrowed(node.as_str()),
            Term::Literal(lit) => Cow::Borrowed(lit.value()), // TODO
            #[cfg(feature = "rdf-star")]
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}

pub struct OxrdfGraphName {
    inner: oxrdf::GraphName,
}

impl Term for OxrdfGraphName {
    fn kind(&self) -> TermKind {
        use oxrdf::GraphName;
        match &self.inner {
            GraphName::NamedNode(_) => TermKind::Iri,
            GraphName::BlankNode(_) => TermKind::BNode,
            GraphName::DefaultGraph => todo!(), // TODO
        }
    }

    fn as_str(&self) -> Cow<str> {
        use oxrdf::GraphName;
        match &self.inner {
            GraphName::NamedNode(node) => Cow::Borrowed(node.as_str()),
            GraphName::BlankNode(node) => Cow::Borrowed(node.as_str()),
            GraphName::DefaultGraph => Cow::Borrowed(""), // TODO
        }
    }
}

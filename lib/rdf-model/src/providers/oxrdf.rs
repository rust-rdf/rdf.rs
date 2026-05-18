// This is free and unencumbered software released into the public domain.

#![allow(unused)]

extern crate alloc;

use crate::{Statement, Term, TermKind};
use alloc::{borrow::Cow, string::String};
use oxrdf::Quad;

pub struct OxrdfStatement {
    subject: OxrdfTerm,
    predicate: OxrdfTerm,
    object: OxrdfTerm,
    context: Option<OxrdfTerm>,
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

pub struct OxrdfTerm(oxrdf::Term);

impl Term for OxrdfTerm {
    fn kind(&self) -> TermKind {
        use oxrdf::Term;
        match &self.0 {
            Term::NamedNode(_) => TermKind::Iri,
            Term::BlankNode(_) => TermKind::BNode,
            Term::Literal(_) => TermKind::Literal,
            #[cfg(feature = "rdf-star")]
            Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }

    // fn as_str(&self) -> Cow<'_, str> {
    //     use oxrdf::Term;
    //     match &self.0 {
    //         Term::NamedNode(node) => Cow::Borrowed(node.as_str()),
    //         Term::BlankNode(node) => Cow::Borrowed(node.as_str()),
    //         Term::Literal(lit) => Cow::Borrowed(lit.value()), // TODO
    //         #[cfg(feature = "rdf-star")]
    //         Term::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
    //     }
    // }

    fn as_str(&self) -> &str {
        use oxrdf::Term;
        match &self.0 {
            Term::NamedNode(node) => node.as_str(),
            Term::BlankNode(node) => node.as_str(),
            Term::Literal(lit) => lit.value(), // TODO
            #[cfg(feature = "rdf-star")]
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

impl From<oxrdf::Subject> for OxrdfTerm {
    fn from(input: oxrdf::Subject) -> Self {
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

pub struct OxrdfSubject(oxrdf::Subject);

impl Term for OxrdfSubject {
    fn kind(&self) -> TermKind {
        use oxrdf::Subject;
        match &self.0 {
            Subject::NamedNode(_) => TermKind::Iri,
            Subject::BlankNode(_) => TermKind::BNode,
            #[cfg(feature = "rdf-star")]
            Subject::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }

    // fn as_str(&self) -> Cow<'_, str> {
    //     use oxrdf::Subject;
    //     match &self.0 {
    //         Subject::NamedNode(node) => Cow::Borrowed(node.as_str()),
    //         Subject::BlankNode(node) => Cow::Borrowed(node.as_str()),
    //         #[cfg(feature = "rdf-star")]
    //         Subject::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
    //     }
    // }

    fn as_str(&self) -> &str {
        use oxrdf::Subject;
        match &self.0 {
            Subject::NamedNode(node) => node.as_str(),
            Subject::BlankNode(node) => node.as_str(),
            #[cfg(feature = "rdf-star")]
            Subject::Triple(_) => todo!("RDF-star support not implemented yet"), // TODO
        }
    }
}

pub struct OxrdfNamedNode(oxrdf::NamedNode);

impl Term for OxrdfNamedNode {
    fn kind(&self) -> TermKind {
        TermKind::Iri
    }

    // fn as_str(&self) -> Cow<'_, str> {
    //     Cow::Borrowed(self.0.as_str())
    // }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

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

    // fn as_str(&self) -> Cow<'_, str> {
    //     use oxrdf::GraphName;
    //     match &self.0 {
    //         GraphName::NamedNode(node) => Cow::Borrowed(node.as_str()),
    //         GraphName::BlankNode(node) => Cow::Borrowed(node.as_str()),
    //         GraphName::DefaultGraph => Cow::Borrowed(""), // TODO
    //     }
    // }

    fn as_str(&self) -> &str {
        use oxrdf::GraphName;
        match &self.0 {
            GraphName::NamedNode(node) => node.as_str(),
            GraphName::BlankNode(node) => node.as_str(),
            GraphName::DefaultGraph => "", // TODO?
        }
    }
}

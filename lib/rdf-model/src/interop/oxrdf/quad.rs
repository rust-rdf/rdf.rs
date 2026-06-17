// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use crate::{HeapQuad, HeapTerm, Statement, Term, TermKind, interop::OxrdfTerm};
use alloc::{borrow::Cow, string::String};
use core::fmt::Debug;
use oxrdf::Quad;

pub type OxrdfStatement = OxrdfQuad;

#[derive(Clone)]
pub struct OxrdfQuad {
    subject: OxrdfTerm,
    predicate: OxrdfTerm,
    object: OxrdfTerm,
    context: Option<OxrdfTerm>,
}

impl core::fmt::Debug for OxrdfQuad {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("OxrdfQuad")
            .field(&self.subject.value_str())
            .field(&self.predicate.value_str())
            .field(&self.object.value_str())
            .field(&self.context.as_ref().map(|c| c.value_str()))
            .finish()
    }
}

impl From<oxrdf::Quad> for OxrdfQuad {
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

impl From<oxrdf::Triple> for OxrdfQuad {
    fn from(quad: oxrdf::Triple) -> Self {
        Self {
            subject: quad.subject.into(),
            predicate: quad.predicate.into(),
            object: quad.object.into(),
            context: None,
        }
    }
}

impl Statement for OxrdfQuad {
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

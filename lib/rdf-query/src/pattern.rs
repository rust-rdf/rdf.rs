extern crate alloc;

use alloc::boxed::Box;
use rdf_model::{Statement, Term};

use crate::{matcher::Matcher, solutions::Solutions, traits::queryable::Queryable};

pub struct Pattern {
    subject: Matcher,
    predicate: Matcher,
    object: Matcher,
    graph_name: Option<Box<dyn Term>>,
}

impl Pattern {
    pub fn new(
        subject: impl Into<Matcher>,
        predicate: impl Into<Matcher>,
        object: impl Into<Matcher>,
        graph_name: Option<Box<dyn Term>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name,
        }
    }

    pub(crate) fn execute(&self, queryable: &impl Queryable) -> Solutions {
        // queryable.query_pattern(self)
        todo!()
    }
}

impl PartialEq<Box<dyn Statement>> for Pattern {
    fn eq(&self, statement: &Box<dyn Statement>) -> bool {
        self.subject == statement.subject()
            && self.predicate == statement.predicate()
            && self.object == statement.object()
    }
}

impl core::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pattern")
            .field("graph_name", &self.graph_name)
            .field("subject", &self.subject)
            .field("predicate", &self.predicate)
            .field("object", &self.object)
            .finish()
    }
}

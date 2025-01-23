extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap};
use rdf_model::{HeapTerm, Statement, Term};

use crate::{
    matcher::Matcher, solution::Solution, traits::queryable::Queryable, variable::Variable,
};

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

    pub(crate) fn execute<Q: Queryable>(&self, queryable: &Q) -> Q
    where
        Self: Sized,
    {
        queryable.query_pattern(self)
    }

    /// Returns a query solution constructed by binding any variables in this
    /// pattern with the corresponding terms in the given statement.
    pub fn solution(&self, statement: Box<dyn Statement>) -> Solution {
        let mut bindings = BTreeMap::new();

        if let Some(subject) = self
            .subject
            .as_variable()
            .map(Variable::name)
            .map(Variable::unbound)
        {
            bindings.insert(subject, HeapTerm::from(statement.subject()));
        }

        if let Some(predicate) = self
            .predicate
            .as_variable()
            .map(Variable::name)
            .map(Variable::unbound)
        {
            bindings.insert(predicate, HeapTerm::from(statement.predicate()));
        }

        if let Some(object) = self
            .object
            .as_variable()
            .map(Variable::name)
            .map(Variable::unbound)
        {
            bindings.insert(object, HeapTerm::from(statement.object()));
        }

        Solution::new(bindings)
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

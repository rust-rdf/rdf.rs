// This is free and unencumbered software released into the public domain.
extern crate alloc;

use alloc::vec;
use rdf_model::Enumerable;

use crate::{graph_pattern::GraphPattern, pattern::Pattern, query::Query, solutions::Solutions};

pub trait Queryable: Enumerable {
    fn query(&self, pattern: impl Into<GraphPattern>) -> Solutions
    where
        Self: Sized,
    {
        match pattern.into() {
            GraphPattern::BasicGraphPattern(query) => self.query_execute(query),
            GraphPattern::TriplePattern(pattern) => self.query_execute(Query::new(vec![pattern])),
        }
    }

    fn query_execute(&self, query: Query) -> Solutions
    where
        Self: Sized,
    {
        query.execute(self)
    }

    fn query_pattern(&self, pattern: &Pattern) -> Self
    where
        Self: Sized,
    {
        self.grep(pattern)
    }
}

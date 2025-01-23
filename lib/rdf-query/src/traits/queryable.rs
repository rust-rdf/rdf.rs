// This is free and unencumbered software released into the public domain.
extern crate alloc;

use rdf_model::Enumerable;

use crate::{pattern::Pattern, query::Query, solutions::Solutions};

pub trait Queryable: Enumerable {
    // fn query(&self, query: GenericQuery<T>) -> GenericSolutions<T>;

    // TODO: is this naming correct?
    fn query_execute(&self, query: Query) -> Solutions
    where
        Self: Sized,
    {
        query.execute(self)
    }

    // TODO: revisit this return type
    fn query_pattern(&self, pattern: &Pattern) -> Self
    where
        Self: Sized,
    {
        self.grep(pattern)
    }
}

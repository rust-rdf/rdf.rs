// This is free and unencumbered software released into the public domain.
extern crate alloc;

use alloc::boxed::Box;
use rdf_model::{Enumerable, Statement};

use crate::{pattern::Pattern, query::Query, solutions::Solutions};

pub trait Queryable: Enumerable + Clone {
    // fn query(&self, query: GenericQuery<T>) -> GenericSolutions<T>;

    fn query_execute(&self, query: Query) -> Solutions {
        // query.execute(self)
        todo!()
    }

    fn query_pattern(&self, pattern: &Pattern) -> impl Iterator<Item = Box<dyn Statement>>
    where
        Self: Sized,
    {
        self.clone() // TODO: check if the clone is necessary
            .filter_map(move |statement| statement.ok().filter(|statement| pattern == statement))
    }
}

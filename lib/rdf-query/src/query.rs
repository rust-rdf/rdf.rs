extern crate alloc;

use alloc::vec::Vec;

use crate::{pattern::Pattern, solutions::Solutions, traits::queryable::Queryable};

pub struct Query {
    patterns: Vec<Pattern>,
}

impl Query {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }

    /// Executes the query on the given graph.
    ///
    /// If the query nas no patterns, it returns a single empty solution as per
    /// SPARQL 1.1 Empty Group Pattern.
    pub fn execute(&self, queryable: impl Queryable) -> Solutions {
        if self.empty() {
            return Solutions::empty();
        }

        for pattern in &self.patterns {
            let _ = pattern.execute(&queryable);
        }

        Solutions::empty()
    }

    /// Returns true if the query has no patterns.
    pub fn empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

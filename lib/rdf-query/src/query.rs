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
    pub fn execute<Q: Queryable>(&self, queryable: &Q) -> Solutions {
        if self.empty() {
            return Solutions::empty();
        }

        let solutions: Vec<_> = self
            .patterns
            .iter()
            .flat_map(|pattern| {
                let statements = pattern.execute(queryable);
                statements.filter_map(|res| res.ok().map(|stmt| pattern.solution(stmt)))
            })
            .collect();

        Solutions::new(solutions.into_iter())
    }

    /// Returns true if the query has no patterns.
    pub fn empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

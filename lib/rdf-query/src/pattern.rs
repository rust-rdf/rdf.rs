extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap, vec};
use rdf_model::{Statement, Term};

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
        let mut ans = BTreeMap::new();

        let bindings = vec![
            (self.subject.as_variable(), statement.subject()),
            (self.predicate.as_variable(), statement.predicate()),
            (self.object.as_variable(), statement.object()),
        ];

        for (variable, term) in bindings {
            if let Some(variable) = variable.map(Variable::name).map(Variable::unbound) {
                ans.insert(variable, term);
            }
        }

        Solution::new(ans)
    }
}

impl PartialEq<Box<dyn Statement>> for Pattern {
    fn eq(&self, statement: &Box<dyn Statement>) -> bool {
        let (s, p, o) = (
            statement.subject(),
            statement.predicate(),
            statement.object(),
        );
        self.subject == s && self.predicate == p && self.object == o
    }
}

impl core::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pattern")
            .field("subject", &self.subject)
            .field("predicate", &self.predicate)
            .field("object", &self.object)
            .field(
                "graph_name",
                &self.graph_name.as_ref().map(|gn| gn.as_str()),
            )
            .finish()
    }
}

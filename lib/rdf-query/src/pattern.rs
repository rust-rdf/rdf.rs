use rdf_model::{HeapTerm, Statement, Term};

use crate::{matcher::GenericMatcher, variable::GenericVariable};

pub type Pattern = GenericPattern<HeapTerm>;

impl<T: Term> From<GenericVariable<T>> for GenericMatcher<T> {
    fn from(var: GenericVariable<T>) -> Self {
        Self::Variable(var)
    }
}

impl<T: Term> From<T> for GenericMatcher<T> {
    fn from(term: T) -> Self {
        Self::Term(term)
    }
}

pub struct GenericPattern<T: Term> {
    subject: GenericMatcher<T>,
    predicate: GenericMatcher<T>,
    object: GenericMatcher<T>,
    graph_name: Option<T>,
}

impl<T: Term> GenericPattern<T> {
    pub fn new(
        subject: impl Into<GenericMatcher<T>>,
        predicate: impl Into<GenericMatcher<T>>,
        object: impl Into<GenericMatcher<T>>,
        graph_name: Option<T>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name,
        }
    }

    pub fn matches(&self, statement: &dyn Statement) -> bool {
        self.subject.matches(statement.subject())
            && self.predicate.matches(statement.predicate())
            && self.object.matches(statement.object())
    }
}

impl<T: Term + core::fmt::Debug> core::fmt::Debug for GenericPattern<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pattern")
            .field("graph_name", &self.graph_name)
            .field("subject", &self.subject)
            .field("predicate", &self.predicate)
            .field("object", &self.object)
            .finish()
    }
}

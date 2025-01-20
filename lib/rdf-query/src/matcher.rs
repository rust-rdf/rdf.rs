use rdf_model::{HeapTerm, Term};

use crate::variable::GenericVariable;

pub type Matcher = GenericMatcher<HeapTerm>;

pub enum GenericMatcher<T: Term> {
    Any,
    Variable(GenericVariable<T>),
    Term(T),
}

impl<T: Term> GenericMatcher<T> {
    pub fn matches(&self, term: &dyn Term) -> bool {
        match self {
            Self::Any | Self::Variable(_) => true,
            Self::Term(t) => t.as_str() == term.as_str(),
        }
    }
}

impl<T: Term + core::fmt::Debug> core::fmt::Debug for GenericMatcher<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Any => f.write_str("Any"),
            Self::Variable(var) => write!(f, "{:?}", var),
            Self::Term(t) => f.write_str(&t.as_str()),
        }
    }
}

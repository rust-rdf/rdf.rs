extern crate alloc;

use alloc::boxed::Box;
use rdf_model::Term;

use crate::variable::Variable;

pub enum Matcher {
    Variable(Variable),
    Term(Box<dyn Term>),
}

impl Matcher {
    pub fn as_variable(&self) -> Option<&Variable> {
        match self {
            Self::Variable(var) => Some(var),
            _ => None,
        }
    }
}

impl PartialEq<&dyn Term> for Matcher {
    fn eq(&self, term: &&dyn Term) -> bool {
        match self {
            Self::Variable(_) => true,
            Self::Term(t) => t.as_str() == term.as_str(),
        }
    }
}

impl From<Variable> for Matcher {
    fn from(var: Variable) -> Self {
        Self::Variable(var)
    }
}

impl From<&str> for Matcher {
    fn from(name: &str) -> Self {
        name.into()
    }
}

impl From<Box<dyn Term>> for Matcher {
    fn from(term: Box<dyn Term>) -> Self {
        Self::Term(term)
    }
}

impl core::fmt::Debug for Matcher {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Variable(var) => write!(f, "{:?}", var),
            Self::Term(t) => f.write_str(&t.as_str()),
        }
    }
}

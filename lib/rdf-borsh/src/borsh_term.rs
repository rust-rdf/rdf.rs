// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{borrow::Cow, string::String};
use borsh::{BorshDeserialize, BorshSerialize};
use rdf_model::{HeapTerm, Term, TermKind};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, BorshSerialize, BorshDeserialize)]
pub struct BorshTerm(HeapTerm);

impl Term for BorshTerm {
    fn kind(&self) -> TermKind {
        self.0.kind()
    }

    fn as_str(&self) -> Cow<str> {
        self.0.as_str()
    }
}

impl From<&str> for BorshTerm {
    fn from(value: &str) -> Self {
        Self(HeapTerm::from(value))
    }
}

impl From<String> for BorshTerm {
    fn from(value: String) -> Self {
        Self(HeapTerm::from(value))
    }
}

impl From<HeapTerm> for BorshTerm {
    fn from(term: HeapTerm) -> Self {
        Self(term)
    }
}

impl From<&dyn Term> for BorshTerm {
    fn from(term: &dyn Term) -> Self {
        Self(HeapTerm::from(term))
    }
}

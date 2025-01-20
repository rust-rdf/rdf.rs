extern crate alloc;

use alloc::string::String;
use rdf_model::{HeapTerm, Term};

pub type Variable = GenericVariable<HeapTerm>;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericVariable<T: Term> {
    name: String,
    value: Option<T>,
}

impl<T: Term> GenericVariable<T> {
    pub fn new(name: impl Into<String>, value: Option<T>) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

impl<T: Term + core::fmt::Debug> core::fmt::Debug for GenericVariable<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Variable")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

extern crate alloc;

use alloc::{boxed::Box, string::String};
use rdf_model::Term;

pub struct Variable {
    name: String,
    value: Option<Box<dyn Term>>,
}

impl Variable {
    pub fn bound(name: impl Into<String>, value: impl Term + 'static) -> Self {
        Self {
            name: name.into(),
            value: Some(Box::new(value)),
        }
    }

    pub fn unbound(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Variable {}

impl PartialOrd for Variable {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Variable {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<&str> for Variable {
    fn from(name: &str) -> Self {
        Variable::unbound(name)
    }
}

impl core::fmt::Debug for Variable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Variable")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

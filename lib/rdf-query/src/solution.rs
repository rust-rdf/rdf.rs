extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap};
use rdf_model::Term;

use crate::variable::Variable;

type Value = Box<dyn Term>;

pub struct Solution {
    bindings: BTreeMap<Variable, Value>,
}

impl Solution {
    pub fn new(bindings: BTreeMap<Variable, Value>) -> Self {
        Self { bindings }
    }

    pub fn binding(&self, var: &Variable) -> Option<&Value> {
        self.bindings.get(var)
    }

    pub fn each_binding(&self) -> impl Iterator<Item = (&Variable, &Value)> {
        self.bindings.iter()
    }

    pub fn each_name(&self) -> impl Iterator<Item = &Variable> {
        self.bindings.keys()
    }

    pub fn each_value(&self) -> impl Iterator<Item = &Value> {
        self.bindings.values()
    }
}

impl core::fmt::Debug for Solution {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Solution").finish()
    }
}

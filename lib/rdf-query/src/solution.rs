extern crate alloc;

use alloc::collections::BTreeMap;
use rdf_model::HeapTerm;

use crate::variable::Variable;

pub struct Solution {
    bindings: BTreeMap<Variable, HeapTerm>,
}

impl Solution {
    pub fn new(bindings: BTreeMap<Variable, impl Into<HeapTerm>>) -> Self {
        let bindings = bindings
            .into_iter()
            .map(|(var, term)| (var, term.into()))
            .collect();

        Self { bindings }
    }

    pub fn binding(&self, var: &Variable) -> Option<&HeapTerm> {
        self.bindings.get(var)
    }

    pub fn each_binding(&self) -> impl Iterator<Item = (&Variable, &HeapTerm)> {
        self.bindings.iter()
    }

    pub fn each_name(&self) -> impl Iterator<Item = &Variable> {
        self.bindings.keys()
    }

    pub fn each_value(&self) -> impl Iterator<Item = &HeapTerm> {
        self.bindings.values()
    }
}

impl core::fmt::Debug for Solution {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Solution").finish()
    }
}

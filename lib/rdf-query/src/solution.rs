extern crate alloc;

use alloc::collections::BTreeMap;
use rdf_model::{HeapTerm, Term};

use crate::variable::GenericVariable;

pub type Solution = GenericSolution<HeapTerm>;

pub struct GenericSolution<T: Term> {
    bindings: BTreeMap<GenericVariable<T>, T>,
}

impl<T: Term + Ord> GenericSolution<T> {
    pub fn new(bindings: BTreeMap<GenericVariable<T>, T>) -> Self {
        Self { bindings }
    }

    pub fn binding(&self, var: &GenericVariable<T>) -> Option<&T> {
        self.bindings.get(var)
    }

    pub fn each_binding(&self) -> impl Iterator<Item = (&GenericVariable<T>, &T)> {
        self.bindings.iter()
    }

    pub fn each_name(&self) -> impl Iterator<Item = &GenericVariable<T>> {
        self.bindings.keys()
    }

    pub fn each_value(&self) -> impl Iterator<Item = &T> {
        self.bindings.values()
    }
}

impl core::fmt::Debug for Solution {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Solution").finish()
    }
}

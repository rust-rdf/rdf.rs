extern crate alloc;

use alloc::boxed::Box;
use rdf_model::Term;

use crate::solution::GenericSolution;

pub struct GenericSolutions<T: Term> {
    iter: Box<dyn Iterator<Item = GenericSolution<T>>>,
}

impl<T: Term> GenericSolutions<T> {
    pub fn new(iter: impl Iterator<Item = GenericSolution<T>> + 'static) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Term> Iterator for GenericSolutions<T> {
    type Item = GenericSolution<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<T: Term> core::fmt::Debug for GenericSolutions<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Solutions").finish()
    }
}

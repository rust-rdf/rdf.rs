extern crate alloc;

use alloc::boxed::Box;

use crate::solution::Solution;

pub struct Solutions {
    iter: Box<dyn Iterator<Item = Solution>>,
}

impl Solutions {
    pub fn new(iter: impl Iterator<Item = Solution> + 'static) -> Self {
        Self {
            iter: Box::new(iter),
        }
    }

    pub fn empty() -> Self {
        Self {
            iter: Box::new(core::iter::empty()),
        }
    }
}

impl Iterator for Solutions {
    type Item = Solution;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl core::fmt::Debug for Solutions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Solutions").finish()
    }
}

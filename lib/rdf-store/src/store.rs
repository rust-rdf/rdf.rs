// This is free and unencumbered software released into the public domain.

extern crate alloc;

use super::Transaction;
use alloc::boxed::Box;

pub trait Store {
    type Error;

    fn begin_transaction(
        &mut self,
    ) -> Result<Box<dyn Transaction<Error = Self::Error>>, Self::Error>;
}

impl core::fmt::Debug for dyn Store<Error = core::convert::Infallible> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Store").finish()
    }
}

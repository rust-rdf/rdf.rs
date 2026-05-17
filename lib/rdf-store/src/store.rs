// This is free and unencumbered software released into the public domain.

use super::Transaction;
use alloc::boxed::Box;
use async_trait::async_trait;
use core::{future::Future, pin::Pin};

#[async_trait]
pub trait Store {
    type Error;

    async fn begin_transaction(
        &mut self,
    ) -> Result<Box<dyn Transaction<Error = Self::Error>>, Self::Error>;
}

impl core::fmt::Debug for dyn Store<Error = core::convert::Infallible> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Store").finish()
    }
}

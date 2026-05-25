// This is free and unencumbered software released into the public domain.

use super::Transaction;
use alloc::boxed::Box;
use async_trait::async_trait;

#[async_trait]
pub trait Store {
    type Error;
    type Transaction: Transaction<Error = Self::Error> + Send;

    async fn begin_transaction(&mut self) -> Result<Self::Transaction, Self::Error>;
}

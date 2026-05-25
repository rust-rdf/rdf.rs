// This is free and unencumbered software released into the public domain.

use super::{ReadTransaction, WriteTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;

#[async_trait]
pub trait Store {
    type Error;
    type Read: ReadTransaction<Error = Self::Error> + Send;
    type Write: WriteTransaction<Error = Self::Error> + Send;

    async fn read(&mut self) -> Result<Self::Read, Self::Error>;

    async fn write(&mut self) -> Result<Self::Write, Self::Error>;
}

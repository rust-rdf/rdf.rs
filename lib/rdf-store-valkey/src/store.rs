// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use rdf_store::Store;
use redis::Client;

#[derive(Debug)]
pub struct ValkeyStore {
    pub(crate) client: Client,
}

impl ValkeyStore {
    pub fn open(url: impl AsRef<str>) -> Result<Self, ValkeyError> {
        let client = redis::Client::open(url.as_ref())?;
        Ok(Self { client })
    }
}

#[async_trait]
impl Store for ValkeyStore {
    type Error = ValkeyError;
    type Read = ValkeyTransaction;
    type Write = ValkeyTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        ValkeyTransaction::begin(self, false)
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        ValkeyTransaction::begin(self, true)
    }
}

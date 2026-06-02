// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use fred::prelude::*;
use rdf_store::Store;

#[derive(Debug)]
pub struct ValkeyStore {
    pub(crate) config: Config,
}

impl ValkeyStore {
    pub fn open(url: impl AsRef<str>) -> Result<Self, ValkeyError> {
        let config = Config::from_url(url.as_ref())?;
        Ok(Self { config })
    }
}

#[async_trait]
impl Store for ValkeyStore {
    type Error = ValkeyError;
    type Read = ValkeyTransaction;
    type Write = ValkeyTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        ValkeyTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        ValkeyTransaction::begin(self, true).await
    }
}

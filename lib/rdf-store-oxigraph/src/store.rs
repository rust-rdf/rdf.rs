// This is free and unencumbered software released into the public domain.

use crate::{OxigraphError, OxigraphTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use rdf_store::Store;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by an Oxigraph database.
#[derive(Debug, Default)]
pub struct OxigraphStore {}

#[async_trait]
impl Store for OxigraphStore {
    type Error = OxigraphError;
    type Read = OxigraphTransaction;
    type Write = OxigraphTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        OxigraphTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        OxigraphTransaction::begin(self, true).await
    }
}

// This is free and unencumbered software released into the public domain.

use crate::{OxigraphError, OxigraphTransaction};
use derive_more::Debug;
use rdf_store::Store;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by an Oxigraph database.
#[derive(Clone, Debug)]
pub struct OxigraphStore {
    #[debug(skip)]
    pub inner: oxigraph::store::Store,
}

impl Default for OxigraphStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OxigraphStore {
    /// Creates a new in-memory Oxigraph store.
    pub fn new() -> Self {
        let inner = oxigraph::store::Store::new().unwrap();
        Self { inner }
    }

    /// Opens an Oxigraph store from a file path.
    #[cfg(feature = "std")]
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, OxigraphError> {
        let store = oxigraph::store::Store::open(path)?;
        Ok(Self { inner: store })
    }
}

impl Store for OxigraphStore {
    type Error = OxigraphError;
    type Read = OxigraphTransaction;
    type Write = OxigraphTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        Ok(OxigraphTransaction::begin(self, false).await?)
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        Ok(OxigraphTransaction::begin(self, true).await?)
    }
}

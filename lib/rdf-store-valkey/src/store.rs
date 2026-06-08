// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use fred::prelude::*;
use rdf_store::Store;

/// The default localhost connection URL for Valkey.
///
/// See: <https://valkey.io/topics/cli/>
pub const DEFAULT_URL: &str = "redis://localhost:6379";

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a Valkey database.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_valkey::ValkeyStore;
/// let mut store = ValkeyStore::open("redis://localhost:6379")?;
/// # Ok(())
/// # }
/// ```
///
/// # Schema
///
/// ```mermaid
/// graph TD
///   RG["rdf:g — set of graph IDs"]:::accent2 --> G["rdf:g:{graph_id} — set of triple IDs"]:::accent3
///   G --> T["rdf:j:{triple_id} — JSON object with s/p/o"]:::accent4
///   T --> S["s: subject term JSON"]:::accent0
///   T --> P["p: predicate term JSON"]:::accent1
///   T --> O["o: object term JSON"]:::accent1
/// ```
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

impl Default for ValkeyStore {
    /// Connects to `redis://localhost:6379` by default.
    fn default() -> Self {
        Self::open(DEFAULT_URL).expect("should connect to redis://localhost:6379")
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

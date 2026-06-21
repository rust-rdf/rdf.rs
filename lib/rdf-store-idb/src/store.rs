// This is free and unencumbered software released into the public domain.

use crate::{IdbError, IdbTransaction};
use alloc::string::String;
use derive_more::Debug;
use idb::{Database, Factory};
use rdf_store::Store;
use send_wrapper::SendWrapper;

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by an IndexedDB database.
///
/// # Examples
///
/// ```rust,compile_fail
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_idb::IdbStore;
/// let mut store = IdbStore::open("test").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Schema
///
/// # Limitations
///
/// - Currently, only supports writing to the store, not yet matching.
/// - Currently, only supports the default graph (acts as a triple store).
///
/// See: <https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API>
#[derive(Debug)]
pub struct IdbStore {
    pub name: String,
    pub db: SendWrapper<Database>,
}

impl IdbStore {
    pub const CURRENT_VERSION: u32 = 1;

    pub async fn open(name: impl Into<String>) -> Result<Self, IdbError> {
        let name = name.into();
        let factory = Factory::new()?;
        let open_request = factory.open(&name, Some(Self::CURRENT_VERSION))?;
        let db = SendWrapper::new(open_request.await?);
        Ok(Self { name, db })
    }
}

/// # Cancel safety
///
/// - `read` / `write` (begin): the `begin` future awaits the
///   browser/JS open operation; canceling the `begin` future before it
///   completes should prevent a transaction object from being returned, but the
///   underlying JS open request may still complete.
/// - Mutating methods (`insert`, `remove`, `clear`, `delete`): these call
///   IndexedDB operations on the created object store and are asynchronous.
///   Canceling an in-flight method may stop local Rust-side work, but any
///   requests already sent to the browser may still be queued/executed there.
/// - `commit` / `rollback`: `commit` calls `tx.commit()` on the IndexedDB
///   transaction wrapper and `rollback` calls `tx.abort()`. These invoke the
///   browser-side transaction lifecycle; cancellation of the Rust future after
///   initiating the call does not necessarily undo the browser-side effect. Do
///   not rely on cancellation to prevent commits once the browser transaction
///   has been committed or aborted.
impl Store for IdbStore {
    type Error = IdbError;
    type Read = IdbTransaction;
    type Write = IdbTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        IdbTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        IdbTransaction::begin(self, true).await
    }
}

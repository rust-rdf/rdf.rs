// This is free and unencumbered software released into the public domain.

use crate::{MongoError, MongoTransaction};
use derive_more::Debug;
use futures::executor::block_on;
use mongodb::{
    Client, Database,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use rdf_store::Store;

/// The default localhost connection URL for MongoDB.
///
/// See: <https://www.mongodb.com/resources/products/compatibilities/docker>
pub const DEFAULT_URL: &str = "mongodb://localhost:27017/test";

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a MongoDB database.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_mongo::MongoStore;
/// let mut store = MongoStore::open("mongodb://localhost:27017/test").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Schema
///
/// ```mermaid
/// graph TD
///   SPO["rdf:spo — collection of triple documents"]
///   SPO --> T["_id: {triple_id} — BSON document with _id/s/p/o"]
///   T --> S["s: subject term (BSON)"]
///   T --> P["p: predicate term (BSON)"]
///   T --> O["o: object term (BSON)"]
///   G["rdf:g:default — collection of triple IDs"]
///   G --> E["_id: {triple_id} — BSON document with _id"]
///   E --> T
/// ```
///
/// # Limitations
///
/// - Currently, only supports writing to the store, not yet matching.
/// - Currently, only supports the default graph (acts as a triple store).
///
/// See: <https://www.mongodb.com/docs/manual/>
#[derive(Clone, Debug)]
pub struct MongoStore {
    #[debug(skip)]
    pub client: Client,
    pub database: Database,
}

impl MongoStore {
    pub async fn open(url: impl AsRef<str>) -> Result<Self, MongoError> {
        let mut options = ClientOptions::parse(url.as_ref()).await?;
        options.app_name = Some("rdf-store-mongo".into());
        options.retry_writes = Some(false);
        options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());

        let client = Client::with_options(options)?;
        let database = client.default_database().ok_or(MongoError::NoDatabase)?;

        Ok(Self { client, database })
    }

    pub fn database(&self) -> Database {
        self.database.clone()
    }
}

impl Default for MongoStore {
    /// Connects to `mongodb://localhost:27017/test` by default.
    fn default() -> Self {
        block_on(Self::open(DEFAULT_URL)).expect("should connect to mongodb://localhost:27017/test")
    }
}

/// # Cancel safety
///
/// - `read` / `write`: begin starts a client session and may start an
///   isolated server-side transaction (controlled by `isolated`). Canceling the
///   begin future will usually prevent a `MongoTransaction` value from being
///   returned; however, network I/O may still have occurred and implementors
///   should ensure sessions are closed if begin is dropped.
/// - `insert` / `remove` etc.: in the current implementation writes are
///   executed immediately against MongoDB (even when `isolated` is false).
///   Canceling in-flight write futures does not undo already-sent operations.
/// - `commit` / `rollback`: when `isolated` is true these call the driver's
///   `commit_transaction` / `abort_transaction`. Server-side transactions are
///   atomic, but cancellation of the client future while a commit is in
///   progress can leave the client unsure whether the transaction committed.
///   Therefore callers must drive `commit` to completion to avoid uncertainty.
impl Store for MongoStore {
    type Error = MongoError;
    type Read = MongoTransaction;
    type Write = MongoTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        MongoTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        MongoTransaction::begin(self, true).await
    }
}

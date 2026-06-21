// This is free and unencumbered software released into the public domain.

use crate::{ValkeyError, ValkeyTransaction};
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
///   RG["rdf:g — set of graph IDs"]
///   RG --> G["rdf:g:{graph_id} — set of triple IDs"]
///   G --> T["rdf:j:{triple_id} — JSON object with s/p/o"]
///   T --> S["s: subject term (JSON-LD)"]
///   T --> P["p: predicate term (JSON-LD)"]
///   T --> O["o: object term (JSON-LD)"]
/// ```
///
/// # Limitations
///
/// - Currently, does not support reading from the store in the same
///   transaction as writing.
///
/// See: <https://valkey.io/topics/>
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

/// # Cancel safety
///
/// - `read` / `write` (begin): the `begin` implementation initializes a Redis
///   client and, for writable transactions, starts a MULTI block (via
///   `client.multi()`). The begin future awaits client initialization; canceling
///   it before completion should prevent a transaction from being returned.
/// - Mutating methods (`insert`, `remove`, `clear`, `delete`) enqueue commands
///   on the Redis transaction (`MULTI`). These calls return once the command is
///   queued. Canceling an individual enqueue future may stop local work, but
///   queued commands already sent to the server will remain queued until `EXEC`
///   or `DISCARD` is called.
/// - `commit`: calls `EXEC` (via `tx.exec(true).await`) and is not cancel safe.
///   Once `EXEC` is issued the server will atomically apply queued commands. If
///   the commit future is canceled while `EXEC` is in flight the client may be
///   uncertain whether the commands were applied.
/// - `rollback`: calls `DISCARD` (`tx.reset()`) which is synchronous in the
///   client; dropping the rollback future before it runs will leave queued
///   commands present, so callers should call `rollback` or drop the transaction
///   in a controlled way.
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

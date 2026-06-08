// This is free and unencumbered software released into the public domain.

use crate::{Neo4jError, Neo4jTransaction};
use alloc::{boxed::Box, string::String};
use async_trait::async_trait;
use derive_more::Debug;
use futures::executor::block_on;
use neo4rs::Graph;
use rdf_store::Store;

/// The default localhost connection URL for Neo4j.
///
/// See: <https://neo4j.com/docs/browser/operations/dbms-connection/>
pub const DEFAULT_URL: &str = "bolt://localhost:7687";

/// The default localhost username for Neo4j. (Always "neo4j".)
///
/// See: <https://neo4j.com/docs/operations-manual/current/docker/introduction/>
pub const DEFAULT_USERNAME: &str = "neo4j";

/// The default localhost password for Neo4j. (Set with `NEO4J_AUTH`.)
///
/// See: <https://neo4j.com/docs/operations-manual/current/docker/introduction/>
pub const DEFAULT_PASSWORD: &str = "your_password";

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a Neo4j database.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_neo4j::Neo4jStore;
/// let mut store = Neo4jStore::open("bolt://localhost:7687", "neo4j", "your_password").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Neo4jStore {
    #[debug(skip)]
    pub graph: Graph,
}

impl Neo4jStore {
    pub async fn open(
        url: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self, Neo4jError> {
        let graph = Graph::new(url, username, password).await?;
        Ok(Self { graph })
    }
}

impl Default for Neo4jStore {
    /// Connects to `bolt://localhost:7687` by default.
    fn default() -> Self {
        block_on(Self::open(DEFAULT_URL, DEFAULT_USERNAME, DEFAULT_PASSWORD))
            .expect("should connect to bolt://localhost:7687")
    }
}

#[async_trait]
impl Store for Neo4jStore {
    type Error = Neo4jError;
    type Read = Neo4jTransaction;
    type Write = Neo4jTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        Neo4jTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        Neo4jTransaction::begin(self, true).await
    }
}

// This is free and unencumbered software released into the public domain.

use crate::{VirtuosoError, VirtuosoTransaction};
use alloc::boxed::Box;
use async_trait::async_trait;
use derive_more::Debug;
use odbc_api::{Connection, ConnectionOptions, Environment};
use rdf_store::Store;
use std::sync::LazyLock;

/// The default localhost ODBC connection string for Virtuoso.
pub const DEFAULT_CONNECTION_STRING: &str =
    "Driver={OpenLink Virtuoso Driver};Host=localhost:1111;UID=dba;PWD=mysecret;";

pub(crate) static ODBC_ENV: LazyLock<Environment> =
    LazyLock::new(|| Environment::new().expect("should find ODBC driver manager"));

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a Virtuoso database.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_virtuoso::VirtuosoStore;
/// let mut store = VirtuosoStore::open("Driver={OpenLink Virtuoso Driver};Host=localhost:1111;UID=dba;PWD=mysecret;")?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct VirtuosoStore {
    pub conn: Box<Connection<'static>>,
}

impl VirtuosoStore {
    pub fn open(connection_string: impl AsRef<str>) -> Result<Self, VirtuosoError> {
        Ok(Self {
            conn: Box::new(ODBC_ENV.connect_with_connection_string(
                connection_string.as_ref(),
                ConnectionOptions::default(),
            )?),
        })
    }
}

impl Default for VirtuosoStore {
    /// Connects to `tcp://localhost:1111` by default.
    fn default() -> Self {
        Self::open(DEFAULT_CONNECTION_STRING).expect("should connect to tcp://localhost:1111")
    }
}

#[async_trait]
impl Store for VirtuosoStore {
    type Error = VirtuosoError;
    type Read = VirtuosoTransaction;
    type Write = VirtuosoTransaction;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        todo!() // FIXME: VirtuosoTransaction::begin(self, false).await
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        todo!() // FIXME: VirtuosoTransaction::begin(self, true).await
    }
}

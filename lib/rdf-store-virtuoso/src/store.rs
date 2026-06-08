// This is free and unencumbered software released into the public domain.

use crate::VirtuosoError;
use alloc::boxed::Box;
use derive_more::Debug;
use odbc_api::{Connection, ConnectionOptions, Environment};
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
    fn default() -> Self {
        Self::open(DEFAULT_CONNECTION_STRING).expect("should connect to localhost:1111")
    }
}

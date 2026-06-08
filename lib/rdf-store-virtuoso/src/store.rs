// This is free and unencumbered software released into the public domain.

use crate::VirtuosoError;
use derive_more::Debug;
use odbc_api::{Connection, ConnectionOptions, Environment};
use std::sync::LazyLock;

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
/// let mut store = VirtuosoStore::open("Driver={Virtuoso};Host=127.0.0.1:1111;UID=dba;PWD=mysecret;")?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct VirtuosoStore<'c> {
    pub conn: Connection<'c>,
}

impl<'c> VirtuosoStore<'c> {
    pub fn open(connection_string: impl AsRef<str>) -> Result<Self, VirtuosoError> {
        Ok(Self {
            conn: ODBC_ENV.connect_with_connection_string(
                connection_string.as_ref(),
                ConnectionOptions::default(),
            )?,
        })
    }
}

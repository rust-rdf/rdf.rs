// This is free and unencumbered software released into the public domain.

use crate::PostgresError;
use derive_more::Debug;
use futures::executor::block_on;
use tokio_postgres::{Client, Connection, NoTls, Socket, tls::NoTlsStream};

/// The default localhost connection URL for PostgreSQL.
///
/// See: <https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-CONNSTRING-URIS>
pub const DEFAULT_URL: &str = "postgres://postgres@localhost:5432";

#[cfg_attr(doc, aquamarine::aquamarine)]
/// A quad store backed by a PostgreSQL database.
///
/// # Examples
///
/// ```rust,no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use rdf_store_postgres::PostgresStore;
/// let mut store = PostgresStore::open("postgres://postgres@localhost:5432").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct PostgresStore {
    pub client: Client,

    #[debug(skip)]
    pub connection: Connection<Socket, NoTlsStream>,
}

impl PostgresStore {
    pub async fn open(url: impl AsRef<str>) -> Result<Self, PostgresError> {
        let (client, connection) = tokio_postgres::connect(url.as_ref(), NoTls).await?;
        Ok(Self { client, connection })
    }
}

impl Default for PostgresStore {
    fn default() -> Self {
        block_on(Self::open(DEFAULT_URL))
            .expect("should connect to postgres://postgres@localhost:5432")
    }
}

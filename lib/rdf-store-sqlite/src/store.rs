// This is free and unencumbered software released into the public domain.

use super::{SCHEMA_SQL, SCHEMA_VERSION, SchemaVersion, SqliteError, SqliteTransaction};
use alloc::{boxed::Box, string::ToString};
use async_trait::async_trait;
use rdf_store::Store;
use turso::{Builder, Connection, Database, transaction::TransactionBehavior};

#[allow(unused)]
#[derive(Debug)]
pub struct SqliteStore {
    pub version: u32,
    pub(crate) db: Database,
    pub(crate) conn: Connection,
}

impl SqliteStore {
    pub async fn new() -> Result<Self, SqliteError> {
        Self::open(":memory:").await
    }

    pub async fn open(path: impl AsRef<str>) -> Result<Self, SqliteError> {
        let db = Builder::new_local(path.as_ref())
            .experimental_without_rowid(true)
            .build()
            .await?;
        let conn = db.connect()?;
        let version: SchemaVersion = match conn
            .query(
                "SELECT cast(c.val as integer) FROM rdf_config c WHERE c.key = 'schema' LIMIT 1",
                (),
            )
            .await
        {
            Err(err) => {
                if err.to_string().contains("no such table: rdf_config") {
                    conn.execute_batch(SCHEMA_SQL).await?;
                    SCHEMA_VERSION
                } else {
                    return Err(err);
                }
            }
            Ok(mut rows) => {
                if let Some(row) = rows.next().await? {
                    row.get::<SchemaVersion>(0)?
                } else {
                    // TODO: migrate the schema to the latest version
                    SCHEMA_VERSION
                }
            }
        };
        Ok(Self { version, db, conn })
    }
}

#[async_trait]
impl Store for SqliteStore {
    type Error = SqliteError;
    type Read = SqliteTransaction<'static>;
    type Write = SqliteTransaction<'static>;

    async fn read(&mut self) -> Result<Self::Read, Self::Error> {
        let conn: &'static Connection = Box::leak(Box::new(self.conn.clone())); // obtain 'static lifetime
        let tx =
            turso::transaction::Transaction::new_unchecked(conn, TransactionBehavior::Deferred)
                .await?;
        Ok(SqliteTransaction { tx })
    }

    async fn write(&mut self) -> Result<Self::Write, Self::Error> {
        let conn: &'static Connection = Box::leak(Box::new(self.conn.clone())); // obtain 'static lifetime
        let tx =
            turso::transaction::Transaction::new_unchecked(conn, TransactionBehavior::Deferred)
                .await?;
        Ok(SqliteTransaction { tx })
    }
}

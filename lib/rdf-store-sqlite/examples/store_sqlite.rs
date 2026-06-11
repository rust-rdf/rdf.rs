// This is free and unencumbered software released into the public domain.

use rdf_model::SAMPLE_QUAD;
use rdf_store::{ReadTransaction, Store, WriteTransaction};
use rdf_store_sqlite::SqliteStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = SqliteStore::new().await?;
    eprintln!("{:?}", store);

    let mut tx = store.write().await.unwrap();

    let count = tx.count(None).await.unwrap();
    eprintln!("{:?}", count);

    tx.insert(SAMPLE_QUAD).await.unwrap();

    let count = tx.count(None).await.unwrap();
    eprintln!("{:?}", count);

    Ok(())
}

// This is free and unencumbered software released into the public domain.

use rdf_store::Store;
use rdf_store_sqlite::SqliteStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = SqliteStore::new().await?;
    eprintln!("{:?}", store);
    let _tx = store.begin_transaction().await.unwrap();
    Ok(()) // TODO
}

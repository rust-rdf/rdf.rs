// This is free and unencumbered software released into the public domain.

use rdf_model::{AnyStatement, SAMPLE_QUAD};
use rdf_store::{ReadTransaction, Store, WriteTransaction};
use rdf_store_valkey::ValkeyStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = ValkeyStore::open("redis://localhost:6379")?;
    eprintln!("{:?}", store);

    let mut tx = store.write().await?;
    eprintln!("{:?}", tx);

    // let count = tx.count(AnyStatement).await.unwrap();
    // eprintln!("{:?}", count);

    tx.insert(SAMPLE_QUAD).await?;

    // let count = tx.count((None, None, None, None)).await.unwrap();
    // eprintln!("{:?}", count);

    tx.commit().await?;

    Ok(())
}

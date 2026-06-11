// This is free and unencumbered software released into the public domain.

use rdf_model::{AnyStatement, HeapQuad, HeapTerm, SAMPLE_QUAD};
use rdf_store::{HeapStore, ReadTransaction, Store, WriteTransaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = HeapStore::new();
    eprintln!("{:?}", store);

    let mut tx = store.write().await.unwrap();

    // let count = tx.count(AnyStatement<HeapTerm>).await.unwrap();
    // eprintln!("{:?}", count);

    tx.insert(HeapQuad::from(&SAMPLE_QUAD)).await.unwrap();

    // let count = tx.count(AnyStatement<HeapTerm>).await.unwrap();
    // eprintln!("{:?}", count);

    Ok(())
}

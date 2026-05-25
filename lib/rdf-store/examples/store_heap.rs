// This is free and unencumbered software released into the public domain.

use rdf_store::{HeapStore, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = HeapStore::new();
    eprintln!("{:?}", store);
    let _tx = store.begin_transaction().await.unwrap();
    Ok(()) // TODO
}

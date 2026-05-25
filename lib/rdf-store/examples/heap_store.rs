// This is free and unencumbered software released into the public domain.

use rdf_store::HeapStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = HeapStore::new();
    eprintln!("{:?}", store); // TODO
    Ok(())
}

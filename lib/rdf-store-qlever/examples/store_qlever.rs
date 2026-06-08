// This is free and unencumbered software released into the public domain.

use rdf_store_qlever::QleverStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = QleverStore::default();
    eprintln!("{:?}", store);

    Ok(()) // TODO
}

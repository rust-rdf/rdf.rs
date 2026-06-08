// This is free and unencumbered software released into the public domain.

use rdf_store_mongo::MongoStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = MongoStore::open("mongodb://localhost:27017").await?;
    eprintln!("{:?}", store);

    Ok(()) // TODO
}

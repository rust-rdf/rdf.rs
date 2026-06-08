// This is free and unencumbered software released into the public domain.

use rdf_store_neo4j::Neo4jStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = Neo4jStore::open("bolt://localhost:7687", "neo4j", "your_password").await?;
    eprintln!("{:?}", store);

    Ok(()) // TODO
}

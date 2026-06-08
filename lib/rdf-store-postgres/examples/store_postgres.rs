// This is free and unencumbered software released into the public domain.

use rdf_store_postgres::PostgresStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = PostgresStore::open("postgres://postgres@localhost:5432").await?;
    eprintln!("{:?}", store);

    Ok(()) // TODO
}

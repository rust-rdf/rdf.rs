// This is free and unencumbered software released into the public domain.

use rdf_store_virtuoso::VirtuosoStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let store = VirtuosoStore::open(
        "Driver={OpenLink Virtuoso Driver};Host=localhost:1111;UID=dba;PWD=mysecret;",
    )?;
    eprintln!("{:?}", store);

    Ok(()) // TODO
}

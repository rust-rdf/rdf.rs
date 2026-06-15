// This is free and unencumbered software released into the public domain.

use rdf_reader_jsonld::JsonldReader;
use tokio::fs::File;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.jsonld").await?;

    let reader = JsonldReader::from(file);

    spawn_blocking(move || {
        for triple in reader {
            eprintln!("{:?}", triple);
        }
    })
    .await?;

    Ok(())
}

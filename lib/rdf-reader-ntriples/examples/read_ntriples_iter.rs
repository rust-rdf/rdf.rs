// This is free and unencumbered software released into the public domain.

use rdf_reader_ntriples::NtriplesReader;
use tokio::fs::File;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.nt").await?;

    let reader = NtriplesReader::from(file);

    spawn_blocking(move || {
        for triple in reader {
            eprintln!("{:?}", triple);
        }
    })
    .await?;

    Ok(())
}

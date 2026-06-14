// This is free and unencumbered software released into the public domain.

use futures::StreamExt;
use rdf_reader_ntriples::NtriplesReader;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.nt").await?;

    let reader = NtriplesReader::open(file).await?;
    reader
        .into_stream()
        .for_each(|triple| async move {
            eprintln!("{:?}", triple);
        })
        .await;

    Ok(())
}

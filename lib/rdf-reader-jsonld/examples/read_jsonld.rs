// This is free and unencumbered software released into the public domain.

use futures::StreamExt;
use rdf_reader_jsonld::JsonldReader;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.jsonld").await?;

    let reader = JsonldReader::open(file).await?;
    reader
        .into_stream()
        .for_each(|triple| async move {
            eprintln!("{:?}", triple);
        })
        .await;

    Ok(())
}

// This is free and unencumbered software released into the public domain.

use futures::StreamExt;
use rdf_reader_rdfxml::RdfxmlReader;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.rdf").await?;

    let reader = RdfxmlReader::from(file);

    reader
        .into_stream()
        .for_each(|triple| async move {
            eprintln!("{:?}", triple);
        })
        .await;

    Ok(())
}

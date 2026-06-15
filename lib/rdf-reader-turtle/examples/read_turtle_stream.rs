// This is free and unencumbered software released into the public domain.

use futures::StreamExt;
use rdf_reader_turtle::TurtleReader;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.ttl").await?;

    let reader = TurtleReader::from(file);

    reader
        .into_stream()
        .for_each(|triple| async move {
            eprintln!("{:?}", triple);
        })
        .await;

    Ok(())
}

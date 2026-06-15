// This is free and unencumbered software released into the public domain.

use futures::StreamExt;
use rdf_reader_trig::TrigReader;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.trig").await?;

    let reader = TrigReader::from(file);

    reader
        .into_stream()
        .for_each(|quad| async move {
            eprintln!("{:?}", quad);
        })
        .await;

    Ok(())
}

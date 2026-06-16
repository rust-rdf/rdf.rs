// This is free and unencumbered software released into the public domain.

use rdf_reader_hdt::HdtReader;
use tokio::fs::File;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.hdt").await?;

    let reader = HdtReader::try_from(file).await?;

    spawn_blocking(move || {
        for triple in reader {
            eprintln!("{:?}", triple);
        }
    })
    .await?;

    Ok(())
}

// This is free and unencumbered software released into the public domain.

use rdf_reader_turtle::TurtleReader;
use tokio::fs::File;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.ttl").await?;

    let reader = TurtleReader::from(file);

    spawn_blocking(move || {
        for triple in reader.into_iter() {
            eprintln!("{:?}", triple);
        }
    })
    .await?;

    Ok(())
}

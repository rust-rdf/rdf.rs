// This is free and unencumbered software released into the public domain.

use rdf_writer_turtle::TurtleWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.ttl").await?;

    let writer = TurtleWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

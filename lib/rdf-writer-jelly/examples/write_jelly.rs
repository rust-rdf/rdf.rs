// This is free and unencumbered software released into the public domain.

use rdf_writer_jelly::JellyWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.jelly").await?;

    let writer = JellyWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

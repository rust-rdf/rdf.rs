// This is free and unencumbered software released into the public domain.

use rdf_writer_jsonld::JsonldWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.jsonld").await?;

    let writer = JsonldWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

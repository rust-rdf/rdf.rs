// This is free and unencumbered software released into the public domain.

use rdf_writer_rdfxml::RdfxmlWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.rdf").await?;

    let writer = RdfxmlWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

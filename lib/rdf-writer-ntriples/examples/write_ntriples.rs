// This is free and unencumbered software released into the public domain.

use rdf_writer_ntriples::NtriplesWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.nt").await?;

    let writer = NtriplesWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

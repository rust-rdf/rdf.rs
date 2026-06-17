// This is free and unencumbered software released into the public domain.

use rdf_writer_cottas::CottasWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.cottas").await?;

    let writer = CottasWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

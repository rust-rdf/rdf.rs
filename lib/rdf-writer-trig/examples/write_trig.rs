// This is free and unencumbered software released into the public domain.

use rdf_writer_trig::TrigWriter;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::create("output.trig").await?;

    let writer = TrigWriter::from(file);

    // TODO

    writer.finish().await?;

    Ok(())
}

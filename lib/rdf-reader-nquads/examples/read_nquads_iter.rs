// This is free and unencumbered software released into the public domain.

use rdf_reader_nquads::NquadsReader;
use tokio::fs::File;
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.nq").await?;

    let reader = NquadsReader::from(file);

    spawn_blocking(move || {
        for quad in reader {
            eprintln!("{:?}", quad);
        }
    })
    .await?;

    Ok(())
}

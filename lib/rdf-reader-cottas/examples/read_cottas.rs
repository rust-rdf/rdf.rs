// This is free and unencumbered software released into the public domain.

use futures_util::stream::TryStreamExt;
use rdf_reader_cottas::CottasReader;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let file = File::open("example.cottas").await?;

    let reader = CottasReader::open(file).await?;
    let results = reader.try_collect::<Vec<_>>().await;
    println!("{:?}", results);

    Ok(()) // TODO
}

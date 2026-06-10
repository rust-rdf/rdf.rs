// This is free and unencumbered software released into the public domain.

use rdf_model::{HeapQuad, SAMPLE_QUAD};
use rdf_store::{Store, WriteTransaction};
use rdf_store_mongo::MongoStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = MongoStore::open("mongodb://localhost:27017/test").await?;
    dbg!(&store);

    let mut tx = store.write().await?;
    dbg!(&tx);

    tx.remove(&HeapQuad::from(&SAMPLE_QUAD)).await?;
    tx.insert(&HeapQuad::from(&SAMPLE_QUAD)).await?;

    tx.commit().await?;

    Ok(())
}

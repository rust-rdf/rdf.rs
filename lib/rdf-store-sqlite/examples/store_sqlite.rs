// This is free and unencumbered software released into the public domain.

use rdf_model::{AnyStatement, HeapQuad, HeapTerm, SAMPLE_QUAD};
use rdf_store::{ReadTransaction, Store, WriteTransaction};
use rdf_store_sqlite::SqliteStore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut store = SqliteStore::new().await?;
    eprintln!("{:?}", store);

    let mut tx = store.write().await.unwrap();

    let count = tx
        .count_statements(None::<AnyStatement<HeapTerm>>)
        .await
        .unwrap();
    eprintln!("{:?}", count);

    tx.insert_statement(&HeapQuad::from(&SAMPLE_QUAD))
        .await
        .unwrap();

    let count = tx
        .count_statements(None::<AnyStatement<HeapTerm>>)
        .await
        .unwrap();
    eprintln!("{:?}", count);

    Ok(())
}

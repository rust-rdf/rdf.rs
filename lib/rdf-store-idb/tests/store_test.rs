// This is free and unencumbered software released into the public domain.

use rdf_store_idb::IdbStore;
use wasm_bindgen_test::*;

// `wasm-pack test --headless --chrome`
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_indexeddb_exists() {
    let store = IdbStore::open("test").await;
    assert!(
        store.is_ok(),
        "Store::open() should be Ok(): {}",
        store.unwrap_err()
    );
}

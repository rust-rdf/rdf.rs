// This is free and unencumbered software released into the public domain.

#![cfg(all(feature = "std", target_arch = "wasm32"))]

use rdf_store_idb::{IdbError, IdbStore};
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

#[wasm_bindgen_test]
fn driver_errors_keep_their_source_and_satisfy_send() {
    use core::error::Error;

    let factory = idb::Factory::new().unwrap();
    // IndexedDB versions must be greater than zero. This exercises a real
    // JavaScript exception rather than a thread-safe Rust-only error variant.
    let original = factory.open("rdf-rs-invalid-version", Some(0)).unwrap_err();
    let message = original.to_string();
    let error = IdbError::from(original);

    fn assert_send<T: Send>(_: &T) {}
    assert_send(&error);
    assert_eq!(error.to_string(), format!("server returned: {message}"));
    assert_eq!(error.source().unwrap().to_string(), message);
}

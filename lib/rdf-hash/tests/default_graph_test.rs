// This is free and unencumbered software released into the public domain.

#![cfg(feature = "blake3")]

use rdf_hash::TermHash;
use rdf_model::{CowTerm, DEFAULT_GRAPH, DEFAULT_GRAPH_URN, HeapTerm};

#[test]
fn default_graph_hash_is_distinct_from_its_external_iri() {
    let marker = TermHash::from(HeapTerm::from(DEFAULT_GRAPH));
    assert_eq!(marker, TermHash::from(CowTerm::from(DEFAULT_GRAPH)));
    assert_ne!(marker, TermHash::from(HeapTerm::iri(DEFAULT_GRAPH_URN)));
}

#[test]
fn existing_term_encodings_are_unchanged() {
    for (term, encoded) in [
        (HeapTerm::iri(DEFAULT_GRAPH_URN), "<urn:rdf:default-graph>"),
        (HeapTerm::bnode("node"), "_:node"),
        (HeapTerm::string("value"), "\"value\""),
    ] {
        assert_eq!(
            TermHash::from(term).as_bytes(),
            blake3::hash(encoded.as_bytes()).as_bytes()
        );
    }
}

// This is free and unencumbered software released into the public domain.

#![cfg(feature = "alloc")]

use borsh::BorshDeserialize;
use rdf_borsh::BorshTerm;
use rdf_model::{DEFAULT_GRAPH, DEFAULT_GRAPH_URN, HeapTerm, Term, TermKind};

#[test]
fn marker_is_not_silently_encoded_as_a_dictionary_iri() {
    let marker = BorshTerm::from(DEFAULT_GRAPH);
    assert!(marker.is_default_graph());
    let error = borsh::to_vec(&marker).unwrap_err();
    assert_eq!(error.kind(), borsh::io::ErrorKind::InvalidInput);
}

#[test]
fn reserved_urn_is_still_an_ordinary_iri_in_existing_dictionary_encoding() {
    let term = BorshTerm::from(HeapTerm::iri(DEFAULT_GRAPH_URN));
    let encoded = borsh::to_vec(&term).unwrap();
    assert_eq!(&encoded[..5], &[1, 21, 0, 0, 0]);
    assert_eq!(&encoded[5..], DEFAULT_GRAPH_URN.as_bytes());
    let decoded = BorshTerm::try_from_slice(&encoded).unwrap();
    assert_eq!(decoded.kind(), TermKind::Iri);
    assert!(!decoded.is_default_graph());
}

#[cfg(feature = "std")]
#[test]
fn default_graph_uses_native_zero_context_and_does_not_erase_named_urn_graphs() {
    use rdf_borsh::{BorshReader, BorshWriter};
    use rdf_model::HeapQuad;
    use rdf_writer::Writer;
    use std::io::Seek;

    let mut file = tempfile::tempfile().unwrap();
    let mut writer = BorshWriter::<u16>::new(Box::new(file.try_clone().unwrap())).unwrap();
    let default = HeapQuad::new(
        HeapTerm::iri("urn:s"),
        HeapTerm::iri("urn:p"),
        HeapTerm::string("o"),
        None,
    );
    let explicit = HeapQuad::new(
        default.subject().clone(),
        default.predicate().clone(),
        default.object().clone(),
        Some(DEFAULT_GRAPH.into()),
    );
    let named = default
        .clone()
        .with_context(HeapTerm::iri(DEFAULT_GRAPH_URN));
    writer.write_statement(&default).unwrap();
    writer.write_statement(&explicit).unwrap();
    writer.write_statement(&named).unwrap();
    assert_eq!(writer.quad_count(), 2);
    writer.finish().unwrap();
    file.rewind().unwrap();
    let decoded = BorshReader::<_, u16>::new(file)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(decoded, vec![default, named]);
}

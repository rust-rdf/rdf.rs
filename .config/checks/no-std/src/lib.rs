//! Compile-only consumer exercising usable APIs on a target without `std`.

#![no_std]
#![forbid(unsafe_code)]
#![allow(dead_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

fn minimal() -> Result<(), xsd::ParseError> {
    use rdf::{
        format::Format,
        model::{AnyTerm, Quad, QuadPattern, TermKind, Triple},
    };
    let triple = Triple::new(AnyTerm, AnyTerm, AnyTerm);
    let quad: Quad<AnyTerm> = triple.to_quad();
    let _: QuadPattern<AnyTerm> = quad.to_quad_pattern();
    let _ = (TermKind::Iri, Format::from_extension("nt"));
    let _ = xsd::parse("42", xsd::INT)?;
    Ok(())
}

#[cfg(feature = "alloc")]
fn owned_quad() -> rdf::model::HeapQuad {
    use rdf::model::{HeapQuad, HeapTerm};
    let _ = alloc::format!("{}", rdf::format::Format::NQuads);
    let _ = rdf::format::Format::NQuads.extensions();
    HeapQuad::new(
        HeapTerm::iri("urn:subject"),
        rdf::vocab::rdf::TYPE.into(),
        HeapTerm::iri("urn:Class"),
        Some(HeapTerm::iri("urn:graph")),
    )
}

#[cfg(feature = "alloc")]
struct EmptyTransaction;

#[cfg(feature = "alloc")]
impl rdf::store::ReadTransaction for EmptyTransaction {
    type Error = core::convert::Infallible;
    type Term = rdf::model::HeapTerm;
    type Statement = rdf::model::HeapQuad;
    type StatementPattern = rdf::model::HeapQuadPattern;

    fn r#match(
        &self,
        _: impl Into<Self::StatementPattern> + Send,
    ) -> impl futures::Stream<Item = Result<Self::Statement, Self::Error>> + Send {
        futures::stream::empty()
    }
}

#[cfg(feature = "alloc")]
async fn transaction_defaults() {
    use rdf::store::ReadTransaction;
    let tx = EmptyTransaction;
    let _ = tx.is_empty().await;
    let _ = tx.contains(()).await;
    let _ = tx.count(()).await;
    let _ = tx.contexts();
}

#[cfg(feature = "serde")]
fn json_roundtrip() -> Result<rdf::model::HeapQuad, serde_json::Error> {
    let encoded = serde_json::to_vec(&owned_quad())?;
    serde_json::from_slice(&encoded)
}

#[cfg(feature = "hash")]
fn hash_quad() -> rdf::hash::TripleHash {
    (&owned_quad()).into()
}

#[cfg(feature = "datetime")]
fn datetime() -> Result<xsd::Value, xsd::ParseError> {
    xsd::parse("2026-12-31T12:34:56", xsd::DATE_TIME)
}

#[cfg(feature = "borsh")]
fn encode_header() -> Result<alloc::vec::Vec<u8>, borsh::io::Error> {
    let header = rdf_borsh::BorshHeader {
        magic: *b"RDFB",
        version: b'1',
        flags: 7,
        quad_count: 0,
    };
    let mut dataset = rdf_borsh::BorshDataset::new();
    let _ = dataset.intern_term(rdf::model::HeapTerm::iri("urn:term").into());
    borsh::to_vec(&header)
}

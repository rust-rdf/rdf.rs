// This is free and unencumbered software released into the public domain.

#![cfg(feature = "alloc")]

use rdf_model::{Datatype, HeapTerm};

#[test]
fn unparsed_literals_preserve_lexical_form_and_datatype() {
    for (lexical, datatype) in [
        ("--12", Datatype::from(xsd::G_MONTH)),
        ("00ff", Datatype::from(xsd::HEX_BINARY)),
        ("+0042", Datatype::from(xsd::Type::from("unsignedInt"))),
        ("not-an-integer", Datatype::from(xsd::INT)),
        ("+00128", Datatype::from(xsd::BYTE)),
        ("not-a-decimal", Datatype::from(xsd::DECIMAL)),
        ("not-a-float", Datatype::from(xsd::FLOAT)),
        ("1e+", Datatype::from(xsd::DOUBLE)),
        ("TRUE", Datatype::from(xsd::BOOLEAN)),
        ("2", Datatype::from(xsd::BOOLEAN)),
        ("2026-02-29", Datatype::from(xsd::DATE)),
        ("2026-12-31T25:00:00", Datatype::from(xsd::DATE_TIME)),
        ("25:00:00", Datatype::from(xsd::TIME)),
        ("not-a-duration", Datatype::from(xsd::DURATION)),
        (
            "79228162514264337593543950336",
            Datatype::from(xsd::DECIMAL),
        ),
        (" custom value ", Datatype::from_iri("urn:example:datatype")),
    ] {
        let term = HeapTerm::from((lexical.to_owned(), datatype.clone()));
        assert_eq!(term, HeapTerm::TypedLiteral(lexical.to_owned(), datatype));
        #[cfg(feature = "oxrdf")]
        {
            let external = oxrdf::Term::from(term.clone());
            assert_eq!(HeapTerm::from(external), term);
        }
    }
}

#[test]
fn supported_literals_still_parse_as_values() {
    let term = HeapTerm::from(("42".to_owned(), Datatype::from(xsd::INT)));
    assert_eq!(
        term,
        HeapTerm::TypedValue(xsd::parse("42", xsd::INT).unwrap())
    );
}

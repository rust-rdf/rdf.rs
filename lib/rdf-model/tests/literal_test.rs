// This is free and unencumbered software released into the public domain.

#![cfg(feature = "alloc")]

use rdf_model::{Datatype, HeapTerm};

#[test]
fn unparsed_literals_preserve_lexical_form_and_datatype() {
    for (lexical, datatype) in [
        ("--12Z", Datatype::from(xsd::G_MONTH)),
        ("--00", Datatype::from(xsd::G_MONTH)),
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
        ("2026-12-31T12:34:56", Datatype::from(xsd::DATE)),
        ("2026-12-31T12:34:56+02:00", Datatype::from(xsd::DATE)),
        ("2026-12-31T25:00:00", Datatype::from(xsd::DATE_TIME)),
        ("2026-12-31 12:34:56", Datatype::from(xsd::DATE_TIME)),
        ("2026-12-31t12:34:56", Datatype::from(xsd::DATE_TIME)),
        ("2026-12-31T12:34:56,125", Datatype::from(xsd::DATE_TIME)),
        (
            "2026-12-31T12:34:56,125+02:00",
            Datatype::from(xsd::DATE_TIME),
        ),
        ("2026-12-31T12:34", Datatype::from(xsd::DATE_TIME)),
        ("2026-12-31T12:34+02:00", Datatype::from(xsd::DATE_TIME)),
        ("2026-12-31T12:34:56-15:00", Datatype::from(xsd::DATE_TIME)),
        ("2026-12-31T12:34:56+14:01", Datatype::from(xsd::DATE_TIME)),
        ("25:00:00", Datatype::from(xsd::TIME)),
        ("12:34:56,125", Datatype::from(xsd::TIME)),
        ("12:34:56,125+02:00", Datatype::from(xsd::TIME)),
        ("12:34", Datatype::from(xsd::TIME)),
        ("12:34+02:00", Datatype::from(xsd::TIME)),
        ("123456", Datatype::from(xsd::TIME)),
        ("2026-12-31T12:34:56", Datatype::from(xsd::TIME)),
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

#[test]
fn month_literals_preserve_datatype_and_lexical_content() {
    let term = HeapTerm::from(("--12".to_owned(), Datatype::from(xsd::G_MONTH)));
    assert_eq!(
        term,
        HeapTerm::TypedValue(xsd::parse("--12", xsd::G_MONTH).unwrap())
    );
    #[cfg(feature = "oxrdf")]
    {
        let external = oxrdf::Term::from(term.clone());
        assert_eq!(
            external,
            oxrdf::Term::Literal(oxrdf::Literal::new_typed_literal(
                "--12",
                oxrdf::NamedNode::new("http://www.w3.org/2001/XMLSchema#gMonth").unwrap()
            ))
        );
        assert_eq!(
            HeapTerm::from(external),
            HeapTerm::TypedLiteral("--12".to_owned(), Datatype::from(xsd::G_MONTH))
        );
    }
}

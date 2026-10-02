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

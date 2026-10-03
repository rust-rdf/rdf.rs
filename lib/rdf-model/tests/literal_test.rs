// This is free and unencumbered software released into the public domain.

#![cfg(feature = "alloc")]

use rdf_model::{Datatype, HeapTerm};

#[test]
fn unparsed_literals_preserve_lexical_form_and_datatype() {
    for (lexical, datatype) in [
        ("--12+14:01", Datatype::from(xsd::G_MONTH)),
        ("--00", Datatype::from(xsd::G_MONTH)),
        ("---01+14:01", Datatype::from(xsd::G_DAY)),
        ("--02-29+01:00", Datatype::from(xsd::G_MONTH_DAY)),
        ("0000Z", Datatype::from(xsd::G_YEAR)),
        ("-0001-01-14:00", Datatype::from(xsd::G_YEAR_MONTH)),
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
fn partial_calendar_literals_preserve_datatype_and_lexical_content() {
    for (lexical, datatype) in [
        ("--12", xsd::G_MONTH),
        ("--12Z", xsd::G_MONTH),
        ("--01-14:00", xsd::G_MONTH),
        ("---31", xsd::G_DAY),
        ("---31Z", xsd::G_DAY),
        ("--02-29", xsd::G_MONTH_DAY),
        ("0000", xsd::G_YEAR),
        ("-2147483648-12", xsd::G_YEAR_MONTH),
    ] {
        let term = HeapTerm::from((lexical.to_owned(), Datatype::from(datatype.clone())));
        assert_eq!(
            term,
            HeapTerm::TypedValue(xsd::parse(lexical, &datatype).unwrap())
        );
        #[cfg(feature = "oxrdf")]
        {
            let external = oxrdf::Term::from(term.clone());
            assert_eq!(
                external,
                oxrdf::Term::Literal(oxrdf::Literal::new_typed_literal(
                    lexical,
                    oxrdf::NamedNode::new(datatype.iri_string().into_owned()).unwrap()
                ))
            );
            assert_eq!(
                HeapTerm::from(external),
                HeapTerm::TypedLiteral(lexical.to_owned(), Datatype::from(datatype))
            );
        }
    }
}

#[test]
#[cfg(feature = "datetime")]
fn equivalent_temporal_values_retain_distinct_rdf_lexical_identity() {
    for (first, second, datatype) in [
        ("2026-01-02+00:00", "2026-01-02Z", xsd::DATE),
        ("24:00:00.000-00:00", "00:00:00Z", xsd::TIME),
        (
            "-0001-12-31T24:00:00+00:00",
            "0000-01-01T00:00:00Z",
            xsd::DATE_TIME,
        ),
        (
            "2026-01-02T12:34:56.100000000Z",
            "2026-01-02T12:34:56.1Z",
            xsd::DATE_TIME,
        ),
    ] {
        assert_eq!(
            xsd::parse(first, &datatype).unwrap(),
            xsd::parse(second, &datatype).unwrap()
        );
        let first = HeapTerm::typed_literal(first, datatype.clone());
        let second = HeapTerm::typed_literal(second, datatype.clone());
        assert_ne!(first, second);
        assert_eq!(
            std::collections::HashSet::from([first.clone(), second.clone()]).len(),
            2
        );
        assert_eq!(
            std::collections::BTreeSet::from([first.clone(), second.clone()]).len(),
            2
        );
        for term in [first, second] {
            let spelling = term.value_str().into_owned();
            assert_eq!(
                term,
                HeapTerm::TypedLiteral(spelling.clone(), Datatype::from(datatype.clone()))
            );
            assert_ne!(term, HeapTerm::typed_literal(spelling.clone(), xsd::STRING));
            #[cfg(feature = "oxrdf")]
            {
                let external = oxrdf::Term::from(term.clone());
                assert_eq!(
                    external,
                    oxrdf::Term::Literal(oxrdf::Literal::new_typed_literal(
                        spelling,
                        oxrdf::NamedNode::new(datatype.iri_string().into_owned()).unwrap(),
                    ))
                );
                assert_eq!(HeapTerm::from(external), term);
            }
            #[cfg(feature = "serde")]
            assert_eq!(
                serde_json::from_value::<HeapTerm>(serde_json::to_value(&term).unwrap()).unwrap(),
                term
            );
        }
    }
}

#[test]
#[cfg(feature = "datetime")]
fn equal_instants_do_not_collapse_rdf_timezone_spellings() {
    let local = "2026-01-02T01:00:00.000000001+02:00";
    let utc = "2026-01-01T23:00:00.000000001Z";
    let instant = |lexical| {
        let xsd::Value::Primitive(xsd::PrimitiveValue::DateTime(value)) =
            xsd::parse_datetime(lexical).unwrap()
        else {
            panic!("wrong datatype");
        };
        value.to_timestamp().unwrap()
    };
    assert_eq!(instant(local), instant(utc));
    assert_ne!(
        HeapTerm::typed_literal(local, xsd::DATE_TIME),
        HeapTerm::typed_literal(utc, xsd::DATE_TIME)
    );
    // Absence is not an alternative spelling of UTC.
    assert_ne!(
        xsd::parse_datetime("2026-01-02T01:00:00").unwrap(),
        xsd::parse_datetime("2026-01-02T01:00:00Z").unwrap()
    );
}

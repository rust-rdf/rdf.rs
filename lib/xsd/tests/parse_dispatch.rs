// This is free and unencumbered software released into the public domain.

use xsd::{ParseError, PrimitiveType, Type};

#[test]
fn unsupported_builtin_datatypes_return_errors() {
    for (input, datatype) in [
        ("2026-12", xsd::G_YEAR_MONTH),
        ("2026", xsd::G_YEAR),
        ("--12-31", xsd::G_MONTH_DAY),
        ("---31", xsd::G_DAY),
        ("--12", xsd::G_MONTH),
        ("00FF", xsd::HEX_BINARY),
        ("AQI=", xsd::BASE64_BINARY),
        ("urn:example:item", xsd::ANY_URI),
        ("ex:item", xsd::QNAME),
    ] {
        assert_unsupported(input, datatype);
    }
}

#[test]
fn unknown_datatypes_return_errors() {
    for datatype in [
        Type::from("unsignedInt"),
        Type::from("urn:example:datatype"),
        Type::from(PrimitiveType::Other("customPrimitive".into())),
    ] {
        assert_unsupported("42", datatype);
    }
}

#[test]
#[cfg(not(feature = "jiff"))]
fn disabled_temporal_datatypes_return_errors() {
    for (input, datatype) in [
        ("PT1S", xsd::DURATION),
        ("2026-12-31T12:34:56", xsd::DATE_TIME),
        ("12:34:56", xsd::TIME),
        ("2026-12-31", xsd::DATE),
    ] {
        assert_unsupported(input, datatype);
    }
}

#[test]
#[cfg(not(feature = "alloc"))]
fn string_dispatch_without_alloc_returns_an_error() {
    assert_unsupported("hello", xsd::STRING);
}

#[test]
fn invalid_literals_are_distinct_from_unsupported_datatypes() {
    for (input, datatype) in [("maybe", xsd::BOOLEAN), ("128", xsd::BYTE)] {
        let error = xsd::parse(input, datatype).unwrap_err();
        assert!(matches!(error, ParseError::InvalidLiteral));
        assert_eq!(error.to_string(), "invalid XSD literal");
    }
    assert!(matches!(
        xsd::parse_boolean("maybe"),
        Err(ParseError::InvalidLiteral)
    ));
}

#[test]
fn unsupported_error_identifies_the_datatype() {
    let error = xsd::parse("--12", xsd::G_MONTH).unwrap_err();
    assert_eq!(error.to_string(), "unsupported datatype: xsd:gMonth");
}

fn assert_unsupported(input: &str, datatype: Type) {
    assert!(matches!(
        xsd::parse(input, &datatype),
        Err(ParseError::UnsupportedDatatype(actual)) if actual == datatype
    ));
}

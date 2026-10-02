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
#[cfg(not(feature = "alloc"))]
fn string_parser_without_alloc_returns_an_error() {
    for input in ["", "hello", " \tΚαλημέρα <&>\r\n"] {
        let input = input.to_owned();
        assert!(matches!(
            xsd::parse_string(&input),
            Err(ParseError::UnsupportedDatatype(datatype)) if datatype == xsd::STRING
        ));
    }
}

#[test]
#[cfg(feature = "alloc")]
fn string_parser_with_alloc_preserves_owned_lexical_content() {
    for lexical in ["", "hello", " \tΚαλημέρα <&>\r\n"] {
        let result: Result<_, core::convert::Infallible> = {
            let input = lexical.to_owned();
            xsd::parse_string(&input)
        };
        let value = result.unwrap();
        assert_eq!(value.r#type(), xsd::STRING);
        assert_eq!(value.to_string(), lexical);
        assert_eq!(xsd::parse(value.to_string(), xsd::STRING).unwrap(), value);
    }
}

#[test]
fn invalid_literals_are_distinct_from_unsupported_datatypes() {
    for (input, datatype) in [("maybe", xsd::BOOLEAN), ("not-a-number", xsd::DOUBLE)] {
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
fn integer_errors_preserve_datatype_and_source() {
    use core::{error::Error, num::IntErrorKind};
    use xsd::DecimalType;

    for (datatype, below_min, above_max) in [
        (
            DecimalType::Integer,
            "-170141183460469231731687303715884105729",
            "170141183460469231731687303715884105728",
        ),
        (
            DecimalType::Long,
            "-9223372036854775809",
            "9223372036854775808",
        ),
        (DecimalType::Int, "-2147483649", "2147483648"),
        (DecimalType::Short, "-32769", "32768"),
        (DecimalType::Byte, "-129", "128"),
    ] {
        for (input, expected_kind) in [
            ("", IntErrorKind::Empty),
            ("1x", IntErrorKind::InvalidDigit),
            (below_min, IntErrorKind::NegOverflow),
            (above_max, IntErrorKind::PosOverflow),
        ] {
            let error = xsd::parse(input, datatype.clone()).unwrap_err();
            assert!(matches!(
                &error,
                ParseError::InvalidInteger { datatype: actual, .. } if actual == &datatype
            ));
            let source = error
                .source()
                .and_then(|source| source.downcast_ref::<xsd::ParseIntegerError>())
                .expect("integer parse errors must retain their cause");
            assert_eq!(source.kind(), &expected_kind);
            let message = error.to_string();
            assert!(message.contains(datatype.curie()));
            assert!(message.contains(&source.to_string()));
        }
    }
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

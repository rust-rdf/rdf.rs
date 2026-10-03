// This is free and unencumbered software released into the public domain.

use xsd::{DecimalType, ParseIntegerError, Value};

type Parser = fn(&str) -> Result<Value, ParseIntegerError>;

fn parsers() -> [(DecimalType, Parser); 5] {
    [
        (DecimalType::Integer, |s| xsd::parse_integer(s)),
        (DecimalType::Long, |s| xsd::parse_long(s)),
        (DecimalType::Int, |s| xsd::parse_int(s)),
        (DecimalType::Short, |s| xsd::parse_short(s)),
        (DecimalType::Byte, |s| xsd::parse_byte(s)),
    ]
}

#[test]
fn integer_parsers_reject_non_xsd_spellings() {
    for (datatype, parse) in parsers() {
        for input in [
            "", "+", "-", " 1", "1 ", "1\n", "1\t", "1 2", "1.0", ".1", "1e2", "1_0", "0x10",
            "++1", "--1", "+-1", "１２", "−1", "NaN", "INF",
        ] {
            assert!(parse(input).is_err(), "{datatype:?}: {input:?}");
            assert!(
                matches!(xsd::parse(input, datatype.clone()),
                Err(xsd::ParseError::InvalidInteger { datatype: actual, .. })
                if actual == datatype),
                "{datatype:?}: {input:?}"
            );
        }
    }
}

#[test]
fn integer_alternate_spellings_preserve_value_and_datatype() {
    for (datatype, parse) in parsers() {
        for (input, canonical) in [
            ("+1", "1"),
            ("0001", "1"),
            ("+001", "1"),
            ("-001", "-1"),
            ("-0", "0"),
            ("+0", "0"),
            ("000", "0"),
        ] {
            let value = parse(input).unwrap();
            assert_eq!(value.r#type(), datatype.clone().into());
            assert_eq!(value.to_string(), canonical);
            assert_eq!(parse(canonical).unwrap(), value);
            assert_eq!(xsd::parse(input, datatype.clone()).unwrap(), value);
        }
    }
}

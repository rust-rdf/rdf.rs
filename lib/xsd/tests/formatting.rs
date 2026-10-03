// This is free and unencumbered software released into the public domain.

use xsd::{PrimitiveType, PrimitiveValue, Type, Value};

#[test]
fn datatype_names_and_iris() {
    for (name, datatype) in xsd::TYPES.entries() {
        assert_eq!(datatype.to_string(), *name);
        assert_eq!(&name.parse::<Type>().unwrap(), *datatype);
        #[cfg(feature = "alloc")]
        {
            let expected = format!("{}{name}", xsd::BASE_URI);
            assert_eq!(datatype.iri_string(), expected);
            match datatype {
                Type::Primitive(t) => assert_eq!(t.iri_string(), expected),
                Type::Decimal(t) => assert_eq!(t.iri_string(), expected),
                Type::Other(_) => unreachable!(),
            }
        }
    }
}

#[test]
fn custom_datatype_names_and_iris() {
    for name in [
        "unsignedInt",
        "urn:example:datatype",
        "https://example.com/datatype",
        "http://www.w3.org/2001/XMLSchema#integer",
    ] {
        let datatype = Type::from(name);
        let primitive = PrimitiveType::Other(name.into());
        assert_eq!(datatype.to_string(), name);
        assert_eq!(primitive.to_string(), name);
        #[cfg(feature = "alloc")]
        {
            let expected = if name == "unsignedInt" {
                format!("{}{name}", xsd::BASE_URI)
            } else {
                name.to_owned()
            };
            assert_eq!(datatype.iri_string(), expected);
            assert_eq!(primitive.iri_string(), expected);
        }
    }
}

#[test]
#[cfg(feature = "oxrdf")]
fn named_node_iri_round_trips() {
    for iri in [
        "http://www.w3.org/2001/XMLSchema#string",
        "http://www.w3.org/2001/XMLSchema#int",
        "http://www.w3.org/2001/XMLSchema#unsignedInt",
        "urn:example:datatype",
    ] {
        let datatype = Type::from(oxrdf::NamedNode::new(iri).unwrap());
        assert_eq!(datatype.iri_string(), iri);
    }
}

#[test]
fn scalar_lexical_round_trips() {
    for (input, datatype, expected) in [
        ("1", xsd::BOOLEAN, "true"),
        ("false", xsd::BOOLEAN, "false"),
        ("12.5", xsd::DECIMAL, "12.5"),
        ("42", xsd::INTEGER, "42"),
        ("-9223372036854775808", xsd::LONG, "-9223372036854775808"),
        ("+42", xsd::INT, "42"),
        ("-32768", xsd::SHORT, "-32768"),
        ("127", xsd::BYTE, "127"),
        ("1.5", xsd::FLOAT, "1.5"),
        ("INF", xsd::DOUBLE, "INF"),
        ("-INF", xsd::DOUBLE, "-INF"),
        ("NaN", xsd::DOUBLE, "NaN"),
    ] {
        let value = xsd::parse(input, &datatype).unwrap();
        let rendered = value.to_string();
        assert_eq!(rendered, expected);
        assert_eq!(xsd::parse(rendered, datatype).unwrap(), value);
    }
    assert_primitive(PrimitiveValue::from("text<&>"), "text<&>");
    assert_primitive(PrimitiveValue::from(42_i32), "42");
}

#[test]
#[cfg(feature = "jiff")]
fn temporal_lexical_round_trips() {
    for (input, datatype) in [
        ("2026-12-31", xsd::DATE),
        ("2026-12-31T12:34:56", xsd::DATE_TIME),
        ("12:34:56.125", xsd::TIME),
        ("PT1.5S", xsd::DURATION),
        ("-PT1S", xsd::DURATION),
    ] {
        let value = xsd::parse(input, &datatype).unwrap();
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(value.to_string(), datatype).unwrap(), value);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_lexical_year_formatting() {
    use xsd::primitive::Date;

    for (year, expected) in [
        (-9999, "-9999-01-02"),
        (-2026, "-2026-01-02"),
        (-1, "-0001-01-02"),
        (1, "0001-01-02"),
        (9999, "9999-01-02"),
    ] {
        assert_primitive(
            PrimitiveValue::Date(Date::new(year, 1, 2).unwrap()),
            expected,
        );
    }
}

#[test]
fn partial_calendar_lexical_forms() {
    use PrimitiveValue::*;
    for (value, expected) in [
        (GYear(1), "0001"),
        (GYear(-1), "-0001"),
        (GYear(12345), "12345"),
        (GYear(i32::MIN), "-2147483648"),
        (GYearMonth((2026, 1)), "2026-01"),
        (GYearMonth((-1, 12)), "-0001-12"),
        (GMonthDay((2, 29)), "--02-29"),
        (GDay(1), "---01"),
        (GMonth(1), "--01"),
    ] {
        assert_primitive(value, expected);
    }
}

#[test]
#[cfg(feature = "alloc")]
fn binary_and_name_lexical_forms() {
    use PrimitiveValue::*;
    assert_primitive(HexBinary(vec![]), "");
    assert_primitive(HexBinary(vec![0, 1, 15, 16, 171, 255]), "00010F10ABFF");
    // RFC 4648 test vectors exercise all padding lengths and multiple groups.
    for (bytes, expected) in [
        (b"".as_slice(), ""),
        (b"f", "Zg=="),
        (b"fo", "Zm8="),
        (b"foo", "Zm9v"),
        (b"foob", "Zm9vYg=="),
        (b"fooba", "Zm9vYmE="),
        (b"foobar", "Zm9vYmFy"),
        (&[251, 255, 239], "+//v"),
    ] {
        assert_primitive(Base64Binary(bytes.to_vec()), expected);
    }
    assert_primitive(AnyUri("urn:example:item".into()), "urn:example:item");
    assert_primitive(QName("ex".into(), "item".into()), "ex:item");
    assert_primitive(QName("".into(), "item".into()), "item");
}

#[test]
fn formatting_propagates_writer_errors() {
    use core::fmt::Write;

    let mut output = heapless::String::<2>::new();
    assert!(write!(&mut output, "{}", PrimitiveValue::GMonth(1)).is_err());
    #[cfg(feature = "jiff")]
    {
        let value = PrimitiveValue::Date(xsd::primitive::Date::new(-1, 1, 2).unwrap());
        let mut short = heapless::String::<2>::new();
        assert!(write!(&mut short, "{value}").is_err());
        let mut year_only = heapless::String::<5>::new();
        assert!(write!(&mut year_only, "{value}").is_err());
    }
    #[cfg(feature = "alloc")]
    for value in [
        PrimitiveValue::HexBinary(vec![0, 255]),
        PrimitiveValue::Base64Binary(vec![0, 255]),
    ] {
        let mut output = heapless::String::<2>::new();
        assert!(write!(&mut output, "{value}").is_err());
    }
}

fn assert_primitive(value: PrimitiveValue, expected: &str) {
    assert_eq!(value.to_string(), expected);
    assert_eq!(Value::from(value).to_string(), expected);
}

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
fn boolean_errors_identify_the_datatype() {
    use core::error::Error;

    for input in ["", "maybe", "TRUE", "2", " true "] {
        for error in [
            xsd::parse_boolean(input).unwrap_err(),
            xsd::parse(input, xsd::BOOLEAN).unwrap_err(),
        ] {
            assert!(matches!(error, ParseError::InvalidBoolean));
            assert_eq!(
                error.to_string(),
                "invalid xsd:boolean literal: expected true, false, 1, or 0"
            );
            assert!(error.source().is_none());
        }
    }
}

#[test]
fn boolean_parsers_accept_all_four_lexical_forms() {
    for (input, expected) in [("true", true), ("false", false), ("1", true), ("0", false)] {
        let expected = xsd::Value::from(expected);
        assert_eq!(xsd::parse_boolean(input).unwrap(), expected);
        assert_eq!(xsd::parse(input, xsd::BOOLEAN).unwrap(), expected);
    }
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
fn decimal_errors_preserve_datatype_and_source() {
    use core::error::Error;

    for datatype in [xsd::DECIMAL, Type::from(PrimitiveType::Decimal)] {
        for input in [
            "",
            "not-a-decimal",
            "79228162514264337593543950336",
            "-79228162514264337593543950336",
        ] {
            let expected = xsd::parse_decimal(input).unwrap_err();
            let error = xsd::parse(input, &datatype).unwrap_err();
            assert!(matches!(
                &error,
                ParseError::InvalidDecimal { datatype: actual, .. } if actual == &datatype
            ));
            let source = error
                .source()
                .and_then(|source| source.downcast_ref::<xsd::ParseDecimalError>())
                .expect("decimal parse errors must retain their cause");
            assert_eq!(format!("{source:?}"), format!("{expected:?}"));
            assert_eq!(source.to_string(), expected.to_string());
            let message = error.to_string();
            assert!(message.contains(datatype.curie()));
            assert!(message.contains(&source.to_string()));
        }
    }
}

#[test]
fn floating_point_errors_preserve_datatype_and_source() {
    use core::error::Error;

    for datatype in [PrimitiveType::Float, PrimitiveType::Double] {
        for input in ["", "not-a-number", "1e+", "1.2.3"] {
            let expected = if datatype == PrimitiveType::Float {
                xsd::parse_float(input).unwrap_err()
            } else {
                xsd::parse_double(input).unwrap_err()
            };
            let error = xsd::parse(input, datatype.clone()).unwrap_err();
            assert!(matches!(
                &error,
                ParseError::InvalidFloat { datatype: actual, .. } if actual == &datatype
            ));
            let source = error
                .source()
                .and_then(|source| source.downcast_ref::<xsd::ParseFloatError>())
                .expect("floating-point parse errors must retain their cause");
            assert_eq!(source, &expected);
            let message = error.to_string();
            assert!(message.contains(datatype.curie()));
            assert!(message.contains(&source.to_string()));
        }
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_reject_bracketed_annotations() {
    for input in [
        "2026-12-31T12:34:56[Europe/Paris]",
        "2026-12-31T12:34:56[!Europe/Paris]",
        "2026-12-31T12:34:56[+02:00]",
        "2024-02-29T12:34:56.125+02:00[Europe/Paris]",
        "2026-12-31T12:34:56[u-ca=iso8601]",
        "2026-12-31T12:34:56[Europe/Paris][u-ca=iso8601]",
    ] {
        let message = "xsd:dateTime literals must not contain bracketed annotations";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_reject_leap_seconds() {
    for input in [
        "2026-12-31T23:59:60",
        "2026-12-31T12:34:60",
        "2024-02-29T23:59:60.125",
        "2026-12-31T23:59:60+02:00",
        "2026-12-31T23:59:60-02:00",
    ] {
        let message = "xsd:dateTime seconds must be less than 60";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_reject_comma_fractional_seconds() {
    for input in [
        "2026-12-31T12:34:56,125",
        "2024-02-29T00:00:00,000000001",
        "2026-12-31T12:34:56,125+02:00",
        "2026-12-31T12:34:56,125-02:00",
    ] {
        let message = "xsd:dateTime fractional seconds require a period separator";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_require_complete_clock_fields() {
    for input in [
        "2026-12-31T12:34",
        "2026-12-31T00",
        "2026-12-31T123456",
        "2024-02-29T12:34+02:00",
        "2026-12-31T12:34[Etc/UTC]",
    ] {
        let message = "xsd:dateTime literals require hours, minutes, and seconds (hh:mm:ss)";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_require_uppercase_t_separator() {
    for input in [
        "2026-12-31 12:34:56",
        "2026-12-31t12:34:56",
        "2024-02-29 00:00:00.125+02:00",
        "2026-12-31t12:34:56.125-02:00",
        "2026-12-31 12:34:56[Etc/UTC]",
        "2026-12-31t12:34:56[Etc/UTC]",
    ] {
        let message = "xsd:dateTime literals require an uppercase T separator";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_reject_zero_padded_extended_years() {
    for input in [
        "-002026-12-31T12:34:56",
        "-000001-01-01T00:00:00",
        "-009999-12-31T23:59:59.123456789",
        "-002024-02-29T12:34:56.125+02:00",
        "-002026-12-31T12:34:56-02:00",
    ] {
        let message = "xsd:dateTime years longer than four digits must not begin with zero";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_reject_positive_year_signs() {
    for input in [
        "+002026-12-31T12:34:56",
        "+000001-01-01T00:00:00",
        "+009999-12-31T23:59:59.123456789",
        "+002024-02-29T12:34:56.125+02:00",
        "+002026-12-31T12:34:56-02:00",
    ] {
        let message = "xsd:dateTime years must not have a leading plus sign";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_require_date_separators() {
    for input in [
        "20261231T12:34:56",
        "20240229T00:00:00.125",
        "20261231T12:34:56+02:00",
        "20261231T12:34:56-02:00",
        "-0020261231T12:34:56",
    ] {
        let message = "xsd:dateTime literals require hyphen-separated year, month, and day";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_accept_uppercase_t_separator() {
    for input in [
        "0001-01-01T00:00:00",
        "2024-02-29T00:00:00.125",
        "2026-12-31T12:34:56",
        "9999-12-31T23:59:59.123456789",
        "2026-12-31T23:59:59.999999999",
    ] {
        let value = xsd::parse_datetime(input).unwrap();
        assert_eq!(value.r#type(), xsd::DATE_TIME);
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(input, xsd::DATE_TIME).unwrap(), value);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_require_xsd_numeric_offset_syntax() {
    for offset in [
        "+02",
        "-02",
        "+0200",
        "-0200",
        "+02:00:00",
        "-02:00:01",
        "+02:00:00.5",
        "-02:00:00,5",
        "+14",
        "-1400",
        "+14:00:00.000000001",
        "-14:00:00.000000001",
    ] {
        for datetime in ["2026-12-31T12:34:56", "2024-02-29T12:34:56.125"] {
            let input = format!("{datetime}{offset}");
            let message = "xsd:dateTime numeric timezone offsets require +hh:mm or -hh:mm";
            assert_eq!(
                xsd::parse_datetime(&input).unwrap_err().to_string(),
                message
            );
            let ParseError::InvalidTemporal { datatype, source } =
                xsd::parse(&input, xsd::DATE_TIME).unwrap_err()
            else {
                panic!("expected a temporal parse error for {input}");
            };
            assert_eq!(datatype, PrimitiveType::DateTime);
            assert_eq!(source.to_string(), message);
        }
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_reject_out_of_range_offsets() {
    for input in [
        "2026-12-31T12:34:56-15:00",
        "2026-12-31T12:34:56+15:00",
        "2026-12-31T12:34:56-14:01",
        "2024-02-29T00:00:00.125+14:01",
        "2026-12-31T12:34:56-23:59",
        "2026-12-31T12:34:56+23:59",
        "2026-12-31T12:34:56-14:00:01",
        "2026-12-31T12:34:56+14:00:01",
        "2026-12-31T12:34:56+15",
        "2026-12-31T12:34:56-1500",
        "2026-12-31T12:34:56-15:00[Etc/UTC]",
    ] {
        let message = "xsd:dateTime timezone offsets must be between -14:00 and +14:00";
        assert_eq!(xsd::parse_datetime(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE_TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::DateTime);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn datetime_parsers_accept_in_range_offsets() {
    for offset in [
        "-14:00", "+14:00", "-13:59", "+13:59", "-00:00", "+00:00", "+02:00",
    ] {
        let input = format!("2024-02-29T12:34:56.125{offset}");
        let value = xsd::parse_datetime(&input).unwrap();
        assert_eq!(value.r#type(), xsd::DATE_TIME);
        assert_eq!(xsd::parse(&input, xsd::DATE_TIME).unwrap(), value);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_require_xsd_numeric_offset_syntax() {
    for offset in [
        "+02",
        "-02",
        "+0200",
        "-0200",
        "+02:00:00",
        "-02:00:01",
        "+02:00:00.5",
        "-02:00:00,5",
        "+14",
        "-1400",
    ] {
        for time in ["12:34:56", "12:34:56.125"] {
            let input = format!("{time}{offset}");
            let message = "xsd:time numeric timezone offsets require +hh:mm or -hh:mm";
            assert_eq!(xsd::parse_time(&input).unwrap_err().to_string(), message);
            let ParseError::InvalidTemporal { datatype, source } =
                xsd::parse(&input, xsd::TIME).unwrap_err()
            else {
                panic!("expected a temporal parse error for {input}");
            };
            assert_eq!(datatype, PrimitiveType::Time);
            assert_eq!(source.to_string(), message);
        }
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_reject_out_of_range_offsets() {
    for input in [
        "12:34:56-15:00",
        "12:34:56+15:00",
        "12:34:56-14:01",
        "00:00:00.125+14:01",
        "12:34:56-23:59",
        "12:34:56+23:59",
        "12:34:56-14:00:01",
        "12:34:56+14:00:01",
        "12:34:56-14:00:00.000000001",
        "12:34:56+14:00:00.000000001",
        "12:34:56+15",
        "12:34:56-1500",
    ] {
        let message = "xsd:time timezone offsets must be between -14:00 and +14:00";
        assert_eq!(xsd::parse_time(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Time);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_accept_in_range_offsets() {
    for offset in [
        "-14:00", "+14:00", "-13:59", "+13:59", "-00:00", "+00:00", "+02:00",
    ] {
        for time in ["00:00:00", "12:34:56.125", "23:59:59.999999999"] {
            let input = format!("{time}{offset}");
            let value = xsd::parse_time(&input).unwrap();
            assert_eq!(value.r#type(), xsd::TIME);
            assert_eq!(xsd::parse(&input, xsd::TIME).unwrap(), value);
        }
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_reject_bracketed_annotations() {
    for input in [
        "12:34:56[Europe/Paris]",
        "12:34:56[!Europe/Paris]",
        "12:34:56[+02:00]",
        "12:34:56.125+02:00[Europe/Paris]",
        "12:34:56.125-05:00[America/New_York]",
        "12:34:56[u-ca=iso8601]",
        "12:34:56[Europe/Paris][u-ca=iso8601]",
    ] {
        let message = "xsd:time literals must not contain bracketed annotations";
        assert_eq!(xsd::parse_time(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Time);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_reject_comma_fractional_seconds() {
    for input in ["12:34:56,125", "00:00:00,000000001", "12:34:56,125+02:00"] {
        let message = "xsd:time fractional seconds require a period separator";
        assert_eq!(xsd::parse_time(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Time);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_require_complete_clock_fields() {
    for input in [
        "12:34",
        "00:00",
        "23:59",
        "12",
        "1234",
        "123456",
        "12:34+02:00",
        "12:34-02:00",
        "12:34[Etc/UTC]",
        "2026-12-31T12:34:56",
    ] {
        let message = "xsd:time literals require hours, minutes, and seconds (hh:mm:ss)";
        assert_eq!(xsd::parse_time(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Time);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_reject_leap_seconds() {
    for input in [
        "23:59:60",
        "12:34:60",
        "23:59:60.125",
        "23:59:60+02:00",
        "23:59:60-02:00",
    ] {
        let message = "xsd:time seconds must be less than 60";
        assert_eq!(xsd::parse_time(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::TIME).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Time);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn time_parsers_accept_complete_clock_fields() {
    for input in [
        "00:00:00",
        "12:34:00",
        "12:34:56",
        "12:34:56.125",
        "00:00:00.000000001",
        "23:59:59.123456789",
        "23:59:59.999999999",
    ] {
        let value = xsd::parse_time(input).unwrap();
        assert_eq!(value.r#type(), xsd::TIME);
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(input, xsd::TIME).unwrap(), value);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_parsers_reject_zero_padded_extended_years() {
    for input in ["-002026-12-31", "-000001-01-01", "-009999-12-31"] {
        let message = "xsd:date years longer than four digits must not begin with zero";
        assert_eq!(xsd::parse_date(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Date);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_parsers_reject_positive_year_signs() {
    for input in ["+002026-12-31", "+000001-01-01", "+009999-12-31"] {
        let message = "xsd:date years must not have a leading plus sign";
        assert_eq!(xsd::parse_date(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Date);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_parsers_require_hyphenated_components() {
    for input in [
        "00010101",
        "20240229",
        "20261231",
        "99991231",
        "+0020261231",
        "-0000011231",
    ] {
        let message = "xsd:date literals require hyphen-separated year, month, and day";
        assert_eq!(xsd::parse_date(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Date);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_parsers_reject_bracketed_annotations() {
    for input in [
        "2026-12-31[Europe/Paris]",
        "2026-12-31[!Europe/Paris]",
        "2026-12-31[+02:00]",
        "2024-02-29[u-ca=iso8601]",
        "2026-12-31[Europe/Paris][u-ca=iso8601]",
    ] {
        let message = "xsd:date literals must not contain bracketed annotations";
        assert_eq!(xsd::parse_date(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Date);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_parsers_reject_time_components() {
    for input in [
        "2026-12-31T00:00:00",
        "2026-12-31T12:34:56",
        "2026-12-31t12:34:56",
        "2026-12-31 12:34:56",
        "2026-12-31T12:34:56.125+02:00",
        "2026-12-31T12:34:56Z",
    ] {
        let message = "xsd:date literals must not contain a time component";
        assert_eq!(xsd::parse_date(input).unwrap_err().to_string(), message);
        let ParseError::InvalidTemporal { datatype, source } =
            xsd::parse(input, xsd::DATE).unwrap_err()
        else {
            panic!("expected a temporal parse error for {input}");
        };
        assert_eq!(datatype, PrimitiveType::Date);
        assert_eq!(source.to_string(), message);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn date_parsers_accept_calendar_dates() {
    for input in ["0001-01-01", "2024-02-29", "2026-12-31", "9999-12-31"] {
        let value = xsd::parse_date(input).unwrap();
        assert_eq!(value.r#type(), xsd::DATE);
        assert_eq!(value.to_string(), input);
        assert_eq!(xsd::parse(input, xsd::DATE).unwrap(), value);
    }
}

#[test]
#[cfg(feature = "jiff")]
fn temporal_errors_preserve_datatype_and_source() {
    use core::error::Error;

    for (datatype, inputs) in [
        (PrimitiveType::Date, ["", "not-a-date", "2026-02-29"]),
        (
            PrimitiveType::DateTime,
            ["", "not-a-datetime", "2026-12-31T25:00:00"],
        ),
        (PrimitiveType::Time, ["", "not-a-time", "25:00:00"]),
        (
            PrimitiveType::Duration,
            ["", "not-a-duration", "PT999999999999999999999999999999S"],
        ),
    ] {
        for input in inputs {
            let expected = match &datatype {
                PrimitiveType::Date => xsd::parse_date(input).unwrap_err(),
                PrimitiveType::DateTime => xsd::parse_datetime(input).unwrap_err(),
                PrimitiveType::Time => xsd::parse_time(input).unwrap_err(),
                PrimitiveType::Duration => xsd::parse_duration(input).unwrap_err(),
                _ => unreachable!(),
            };
            let error = xsd::parse(input, datatype.clone()).unwrap_err();
            assert!(matches!(
                &error,
                ParseError::InvalidTemporal { datatype: actual, .. } if actual == &datatype
            ));
            let source = error
                .source()
                .and_then(|source| source.downcast_ref::<xsd::ParseTemporalError>())
                .expect("temporal parse errors must retain their cause");
            assert_eq!(format!("{:?}", source.0), format!("{expected:?}"));
            assert_eq!(source.to_string(), expected.to_string());
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

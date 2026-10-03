// This is free and unencumbered software released into the public domain.

#[cfg(any(feature = "serde", feature = "bson"))]
use xsd::{PrimitiveValue, Value};

#[cfg(feature = "serde")]
fn assert_json_string(value: PrimitiveValue, lexical: &str) {
    let expected = serde_json::Value::String(lexical.into());
    assert_eq!(value.to_string(), lexical);
    assert_eq!(value.clone().into_json(), expected);
    assert_eq!(value.to_json(), Some(expected.clone()));
    assert_eq!(serde_json::Value::from(value.clone()), expected);
    assert_eq!(serde_json::Value::from(&value), expected);
    let wrapped = Value::from(value);
    assert_eq!(wrapped.to_json(), Some(expected.clone()));
    assert_eq!(wrapped.into_json(), expected);
}

#[test]
#[cfg(feature = "serde")]
fn hex_binary_json_uses_uppercase_lexical_encoding() {
    for (bytes, lexical) in [
        (&b""[..], ""),
        (&[0][..], "00"),
        (&[0, 1, 15, 16, 127, 128, 254, 255][..], "00010F107F80FEFF"),
    ] {
        assert_json_string(PrimitiveValue::HexBinary(bytes.to_vec()), lexical);
    }
}

#[test]
#[cfg(feature = "bson")]
fn hex_binary_bson_preserves_bytes() {
    let bytes = vec![0, 1, 15, 16, 127, 128, 254, 255];
    let value = PrimitiveValue::HexBinary(bytes.clone());
    let expected = bson::Bson::Binary(bson::Binary {
        bytes,
        subtype: bson::spec::BinarySubtype::Generic,
    });
    assert_eq!(value.clone().into_bson(), expected);
    assert_eq!(value.to_bson(), Some(expected.clone()));
    assert_eq!(bson::Bson::from(value.clone()), expected);
    assert_eq!(Value::from(value).into_bson(), expected);
}

#[test]
#[cfg(feature = "serde")]
fn base64_binary_json_uses_padded_lexical_encoding() {
    for (bytes, lexical) in [
        (&b""[..], ""),
        (&b"f"[..], "Zg=="),
        (&b"fo"[..], "Zm8="),
        (&b"foo"[..], "Zm9v"),
        (&b"foobar"[..], "Zm9vYmFy"),
        (&[0, 251, 255][..], "APv/"),
        (&[251, 255][..], "+/8="),
    ] {
        assert_json_string(PrimitiveValue::Base64Binary(bytes.to_vec()), lexical);
    }
}

#[test]
#[cfg(feature = "bson")]
fn base64_binary_bson_preserves_bytes() {
    for bytes in [vec![], vec![0], vec![0, 251, 255]] {
        let value = PrimitiveValue::Base64Binary(bytes.clone());
        let expected = bson::Bson::Binary(bson::Binary {
            bytes,
            subtype: bson::spec::BinarySubtype::Generic,
        });
        assert_eq!(value.clone().into_bson(), expected);
        assert_eq!(value.to_bson(), Some(expected.clone()));
        assert_eq!(bson::Bson::from(value.clone()), expected);
        assert_eq!(Value::from(value).into_bson(), expected);
    }
}

#[cfg(feature = "bson")]
fn assert_bson_string(value: PrimitiveValue, lexical: &str) {
    let expected = bson::Bson::String(lexical.into());
    assert_eq!(value.to_string(), lexical);
    assert_eq!(value.clone().into_bson(), expected);
    assert_eq!(value.to_bson(), Some(expected.clone()));
    assert_eq!(bson::Bson::from(value.clone()), expected);
    let wrapped = Value::from(value);
    assert_eq!(wrapped.to_bson(), Some(expected.clone()));
    assert_eq!(wrapped.into_bson(), expected);
}

#[test]
#[cfg(all(feature = "jiff", any(feature = "serde", feature = "bson")))]
fn date_conversions_use_xsd_year_formatting() {
    for (year, lexical) in [
        (-9999, "-9999-01-02"),
        (-1, "-0001-01-02"),
        (0, "0000-01-02"),
        (1, "0001-01-02"),
        (2026, "2026-01-02"),
        (9999, "9999-01-02"),
    ] {
        let value = PrimitiveValue::Date(xsd::primitive::Date::new(year, 1, 2).unwrap());
        #[cfg(feature = "serde")]
        assert_json_string(value.clone(), lexical);
        #[cfg(feature = "bson")]
        assert_bson_string(value.clone(), lexical);
        assert_eq!(xsd::parse(lexical, xsd::DATE).unwrap(), Value::from(value));
    }
}

#[test]
#[cfg(all(feature = "jiff", any(feature = "serde", feature = "bson")))]
fn datetime_conversions_preserve_xsd_years_and_fractional_seconds() {
    for (year, date) in [
        (-9999, "-9999-01-02"),
        (-1, "-0001-01-02"),
        (2026, "2026-01-02"),
    ] {
        for (nanosecond, fraction) in [
            (0, ""),
            (1, ".000000001"),
            (125_000_000, ".125"),
            (999_999_999, ".999999999"),
        ] {
            let lexical = format!("{date}T12:34:56{fraction}");
            let value = PrimitiveValue::DateTime(
                xsd::primitive::DateTime::new(year, 1, 2, 12, 34, 56, nanosecond).unwrap(),
            );
            #[cfg(feature = "serde")]
            assert_json_string(value.clone(), &lexical);
            #[cfg(feature = "bson")]
            assert_bson_string(value.clone(), &lexical);
            assert_eq!(
                xsd::parse(&lexical, xsd::DATE_TIME).unwrap(),
                Value::from(value)
            );
        }
    }
}

#[test]
#[cfg(any(feature = "serde", feature = "bson"))]
fn partial_calendar_conversions_preserve_lexical_structure() {
    use PrimitiveValue::*;
    for (value, lexical) in [
        (GYearMonth((1, 2)), "0001-02"),
        (GYearMonth((-1, 12)), "-0001-12"),
        (GYearMonth((10000, 1)), "10000-01"),
        (GYear(1), "0001"),
        (GYear(-1), "-0001"),
        (GYear(i32::MIN), "-2147483648"),
        (GYear(i32::MAX), "2147483647"),
        (GMonthDay((1, 2)), "--01-02"),
        (GMonthDay((2, 29)), "--02-29"),
        (GMonthDay((12, 31)), "--12-31"),
        (GDay(1), "---01"),
        (GDay(31), "---31"),
        (GMonth(1), "--01"),
        (GMonth(12), "--12"),
    ] {
        #[cfg(feature = "serde")]
        assert_json_string(value.clone(), lexical);
        #[cfg(feature = "bson")]
        assert_bson_string(value, lexical);
    }
}

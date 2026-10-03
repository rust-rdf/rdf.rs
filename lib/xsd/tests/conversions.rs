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

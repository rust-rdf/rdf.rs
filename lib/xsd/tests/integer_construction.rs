// This is free and unencumbered software released into the public domain.

use xsd::{DecimalValue, Value};

#[test]
fn signed_rust_integers_preserve_range_and_datatype() {
    macro_rules! check {
        ($rust:ty, $datatype:expr) => {
            for input in [<$rust>::MIN, -1, 0, 1, <$rust>::MAX] {
                let value = Value::from(input);
                assert_eq!(value.r#type(), $datatype);
                assert_eq!(value, Value::from(DecimalValue::from(input)));
                assert_eq!(value.to_string(), input.to_string());
                assert_eq!(xsd::parse(input.to_string(), $datatype).unwrap(), value);
                assert_eq!(Value::from(&input), value);
            }
        };
    }
    check!(i128, xsd::INTEGER);
    check!(i8, xsd::BYTE);
    check!(i16, xsd::SHORT);
    check!(i32, xsd::INT);
    check!(i64, xsd::LONG);
    check!(isize, xsd::INTEGER);
}

#[cfg(feature = "serde")]
#[test]
fn signed_construction_uses_existing_serde_variants() {
    let value = Value::from(42_i32);
    let encoded = serde_json::to_string(&value).unwrap();
    assert_eq!(encoded, r#"{"Decimal":{"Int":42}}"#);
    assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(), value);

    // Previously constructed primitive decimals remain deserializable as such.
    let legacy = Value::from(xsd::PrimitiveValue::from(42_i32));
    let encoded = serde_json::to_string(&legacy).unwrap();
    assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(), legacy);
    assert_ne!(legacy, value);
}

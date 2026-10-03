// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

#[test]
fn decimal_json_preserves_stored_precision_across_both_variants() {
    for input in [
        "0",
        "-1.25",
        "1.2300",
        "0.1234567890123456789012345678",
        "0.0000000000000000000000000001",
        "79228162514264337593543950335",
    ] {
        let number: xsd::primitive::Decimal = input.parse().unwrap();
        let decimal = xsd::DecimalValue::Decimal(number.clone());
        let primitive = xsd::PrimitiveValue::Decimal(number.clone());
        let expected = serde_json::Value::String(number.to_string());
        for json in [
            decimal.clone().into_json(),
            decimal.to_json().unwrap(),
            serde_json::Value::from(decimal.clone()),
            serde_json::Value::from(&decimal),
            primitive.clone().into_json(),
            primitive.to_json().unwrap(),
            serde_json::Value::from(primitive.clone()),
            serde_json::Value::from(&primitive),
        ] {
            assert_eq!(json, expected, "{input}");
            let encoded = serde_json::to_string(&json).unwrap();
            let lexical: String = serde_json::from_str(&encoded).unwrap();
            assert_eq!(lexical.parse::<xsd::primitive::Decimal>().unwrap(), number);
        }
        for value in [xsd::Value::from(decimal), xsd::Value::from(primitive)] {
            assert_eq!(value.to_json(), Some(expected.clone()));
            assert_eq!(serde_json::Value::from(&value), expected);
            assert_eq!(serde_json::Value::from(value.clone()), expected);
            assert_eq!(value.into_json(), expected);
        }
    }
}

// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

#[test]
fn integer_json_preserves_full_range_across_conversion_routes() {
    for input in [
        i128::MIN,
        i64::MIN as i128 - 1,
        i64::MIN as i128,
        -1,
        0,
        1,
        9_007_199_254_740_993,
        i64::MAX as i128,
        i64::MAX as i128 + 1,
        i128::MAX,
    ] {
        let decimal = xsd::DecimalValue::from(input);
        let value = xsd::parse(input.to_string(), xsd::INTEGER).unwrap();
        let expected = match i64::try_from(input) {
            Ok(n) => serde_json::Value::from(n),
            Err(_) => serde_json::Value::String(input.to_string()),
        };
        for json in [
            decimal.clone().into_json(),
            decimal.to_json().unwrap(),
            serde_json::Value::from(decimal.clone()),
            serde_json::Value::from(&decimal),
            value.clone().into_json(),
            value.to_json().unwrap(),
            serde_json::Value::from(value.clone()),
            serde_json::Value::from(&value),
        ] {
            assert_eq!(json, expected, "{input}");
            let encoded = serde_json::to_string(&json).unwrap();
            let decoded: serde_json::Value = serde_json::from_str(&encoded).unwrap();
            let lexical = match decoded {
                serde_json::Value::String(s) => s,
                other => other.to_string(),
            };
            assert_eq!(xsd::parse(lexical, xsd::INTEGER).unwrap(), value);
        }
    }
}

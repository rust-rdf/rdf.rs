// This is free and unencumbered software released into the public domain.

#![cfg(feature = "serde")]

#[test]
fn non_finite_floats_use_xsd_strings_across_conversion_routes() {
    for datatype in [xsd::FLOAT, xsd::DOUBLE] {
        for (input, lexical) in [
            ("INF", "INF"),
            ("+INF", "INF"),
            ("-INF", "-INF"),
            ("NaN", "NaN"),
            ("1e9999", "INF"),
            ("-1e9999", "-INF"),
        ] {
            let value = xsd::parse(input, &datatype).unwrap();
            let primitive = value.as_primitive();
            let expected = serde_json::Value::String(lexical.into());
            for json in [
                value.clone().into_json(),
                value.to_json().unwrap(),
                serde_json::Value::from(value.clone()),
                serde_json::Value::from(&value),
                primitive.clone().into_json(),
                primitive.to_json().unwrap(),
                serde_json::Value::from(primitive.clone()),
                serde_json::Value::from(primitive),
            ] {
                assert_eq!(json, expected);
                let encoded = serde_json::to_string(&json).unwrap();
                let lexical: String = serde_json::from_str(&encoded).unwrap();
                assert_eq!(xsd::parse(lexical, &datatype).unwrap(), value);
            }
        }
    }
}

#[test]
fn finite_floats_remain_json_numbers_including_signed_zero() {
    for datatype in [xsd::FLOAT, xsd::DOUBLE] {
        for (input, expected) in [("1.25", 1.25_f64), ("-0", -0.0_f64), ("0", 0.0_f64)] {
            let json = xsd::parse(input, &datatype).unwrap().into_json();
            let number = json.as_f64().unwrap();
            assert_eq!(number.to_bits(), expected.to_bits());
        }
    }
}

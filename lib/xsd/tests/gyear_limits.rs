use xsd::PrimitiveValue;

const CASES: &[(i32, &str)] = &[
    (i32::MIN, "-2147483648"),
    (-10000, "-10000"),
    (-1, "-0001"),
    (0, "0000"),
    (1, "0001"),
    (10000, "10000"),
    (i32::MAX, "2147483647"),
];

#[test]
fn gyear_formatting_preserves_the_entire_signed_range() {
    for &(year, lexical) in CASES {
        let value = PrimitiveValue::GYear(year);
        assert_eq!(value.to_string(), lexical);
        assert_eq!(value.r#type(), xsd::PrimitiveType::GYear);
        let value = xsd::Value::from(value);
        assert_eq!(value.to_string(), lexical);
        assert_eq!(xsd::parse(lexical, xsd::G_YEAR).unwrap(), value);
        assert_eq!(xsd::parse_g_year(lexical).unwrap(), value);
    }
}

#[cfg(feature = "serde")]
#[test]
fn gyear_serde_and_explicit_json_have_distinct_representations() {
    for &(year, lexical) in CASES {
        let value = PrimitiveValue::GYear(year);
        let encoded = serde_json::to_value(&value).unwrap();
        assert_eq!(encoded, serde_json::json!({"GYear": year}));
        assert_eq!(
            serde_json::from_value::<PrimitiveValue>(encoded).unwrap(),
            value
        );
        assert_eq!(value.into_json(), serde_json::Value::String(lexical.into()));
    }
}

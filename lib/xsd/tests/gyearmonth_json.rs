#![cfg(feature = "serde")]

#[test]
fn explicit_json_preserves_signed_years_and_offsets_without_tags() {
    for input in [
        "-2147483648-01-14:00",
        "2147483647-12+14:00",
        "-0001-12",
        "0000-01Z",
        "10000-02+05:30",
        "0000-01-00:00",
    ] {
        let value = xsd::parse_g_year_month(input).unwrap();
        let primitive = value.as_primitive();
        let expected = serde_json::Value::String(value.to_string());
        assert_eq!(primitive.to_json(), Some(expected.clone()));
        assert_eq!(primitive.clone().into_json(), expected);
        assert_eq!(serde_json::Value::from(primitive), expected);
        assert_eq!(serde_json::Value::from(primitive.clone()), expected);
        assert_eq!(value.to_json(), Some(expected.clone()));
        assert_eq!(value.clone().into_json(), expected);
        let lexical = expected.as_str().unwrap();
        assert_eq!(xsd::parse(lexical, xsd::G_YEAR_MONTH).unwrap(), value);
        assert_ne!(serde_json::to_value(&value).unwrap(), expected);
        assert!(serde_json::from_value::<xsd::Value>(expected.clone()).is_err());
        assert_eq!(
            xsd::parse(lexical, xsd::STRING).unwrap().into_json(),
            expected
        );
    }
}

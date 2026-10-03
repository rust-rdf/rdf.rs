#![cfg(feature = "serde")]

#[test]
fn explicit_json_preserves_fields_and_offset_without_tags() {
    for input in [
        "--01-01",
        "--02-29Z",
        "--12-31-14:00",
        "--04-30+14:00",
        "--02-28+05:30",
        "--02-29-00:00",
    ] {
        let value = xsd::parse_g_month_day(input).unwrap();
        let primitive = value.as_primitive();
        let expected = serde_json::Value::String(value.to_string());
        assert_eq!(primitive.to_json(), Some(expected.clone()));
        assert_eq!(primitive.clone().into_json(), expected);
        assert_eq!(serde_json::Value::from(primitive), expected);
        assert_eq!(serde_json::Value::from(primitive.clone()), expected);
        assert_eq!(value.to_json(), Some(expected.clone()));
        assert_eq!(value.clone().into_json(), expected);
        let lexical = expected.as_str().unwrap();
        assert_eq!(xsd::parse(lexical, xsd::G_MONTH_DAY).unwrap(), value);
        assert_ne!(serde_json::to_value(&value).unwrap(), expected);
        assert!(serde_json::from_value::<xsd::Value>(expected.clone()).is_err());
        assert_eq!(
            xsd::parse(lexical, xsd::STRING).unwrap().into_json(),
            expected
        );
    }
}

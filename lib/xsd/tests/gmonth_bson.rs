#![cfg(feature = "bson")]

#[test]
fn bson_strings_preserve_month_and_offset_without_tags() {
    for input in [
        "--01",
        "--12Z",
        "--02-14:00",
        "--03+14:00",
        "--04+05:30",
        "--05+00:00",
    ] {
        let value = xsd::parse_g_month(input).unwrap();
        let primitive = value.as_primitive();
        let expected = bson::Bson::String(value.to_string());
        assert_eq!(primitive.to_bson(), Some(expected.clone()));
        assert_eq!(primitive.clone().into_bson(), expected);
        assert_eq!(bson::Bson::from(primitive.clone()), expected);
        assert_eq!(value.to_bson(), Some(expected.clone()));
        assert_eq!(value.clone().into_bson(), expected);
        let bson::Bson::String(lexical) = &expected else {
            panic!("expected string")
        };
        assert_eq!(xsd::parse(lexical, xsd::G_MONTH).unwrap(), value);
        assert_eq!(
            xsd::parse(lexical, xsd::STRING).unwrap().into_bson(),
            expected
        );
    }
}

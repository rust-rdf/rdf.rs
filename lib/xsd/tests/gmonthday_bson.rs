#![cfg(feature = "bson")]

#[test]
fn bson_strings_preserve_fields_and_offset_without_tags() {
    for input in [
        "--01-01",
        "--02-29Z",
        "--12-31-14:00",
        "--04-30+14:00",
        "--02-28+05:30",
        "--02-29+00:00",
    ] {
        let value = xsd::parse_g_month_day(input).unwrap();
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
        assert_eq!(xsd::parse(lexical, xsd::G_MONTH_DAY).unwrap(), value);
        assert_eq!(
            xsd::parse(lexical, xsd::STRING).unwrap().into_bson(),
            expected
        );
    }
}

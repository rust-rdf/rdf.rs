#![cfg(feature = "bson")]

#[test]
fn bson_strings_preserve_signed_years_and_offsets_without_tags() {
    for input in [
        "-2147483648-01-14:00",
        "2147483647-12+14:00",
        "-0001-12",
        "0000-01Z",
        "10000-02+05:30",
        "0000-01+00:00",
    ] {
        let value = xsd::parse_g_year_month(input).unwrap();
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
        assert_eq!(xsd::parse(lexical, xsd::G_YEAR_MONTH).unwrap(), value);
        assert_eq!(
            xsd::parse(lexical, xsd::STRING).unwrap().into_bson(),
            expected
        );
    }
}

// This is free and unencumbered software released into the public domain.

#![cfg(feature = "bson")]

#[test]
fn bson_routes_agree_on_numeric_value_and_representation() {
    use bson::Bson;
    use xsd::DecimalValue as D;
    for (value, expected) in [
        (D::Byte(i8::MIN), Bson::Int32(i8::MIN as i32)),
        (D::Short(i16::MAX), Bson::Int32(i16::MAX as i32)),
        (D::Int(i32::MAX), Bson::Int32(i32::MAX)),
        (D::Long(i64::MAX), Bson::Int64(i64::MAX)),
        (
            D::from(9_007_199_254_740_993_i128),
            Bson::Int64(9_007_199_254_740_993),
        ),
        (
            D::from(i64::MAX as i128 + 1),
            Bson::Decimal128("9223372036854775808".parse().unwrap()),
        ),
        (
            D::Decimal("0.1234567890123456789012345678".parse().unwrap()),
            Bson::String("0.1234567890123456789012345678".into()),
        ),
    ] {
        assert_eq!(Bson::from(value.clone()), expected);
        assert_eq!(value.to_bson(), Some(expected.clone()));
        assert_eq!(value.clone().into_bson(), expected);
        let wrapped = xsd::Value::from(value);
        assert_eq!(Bson::from(wrapped.clone()), expected);
        assert_eq!(wrapped.to_bson(), Some(expected.clone()));
        assert_eq!(wrapped.into_bson(), expected);
    }
}

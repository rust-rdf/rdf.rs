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

#[test]
fn bson_integer_limits_use_exact_strings_instead_of_panicking() {
    for input in [
        i128::MIN,
        i128::MAX,
        10_i128.pow(34) + 1,
        -(10_i128.pow(34) + 1),
    ] {
        let value = xsd::DecimalValue::from(input);
        let expected = bson::Bson::String(input.to_string());
        for bson in [
            value.clone().into_bson(),
            value.to_bson().unwrap(),
            bson::Bson::from(value.clone()),
            xsd::Value::from(value.clone()).into_bson(),
            xsd::Value::from(value.clone()).to_bson().unwrap(),
            bson::Bson::from(xsd::Value::from(value.clone())),
        ] {
            assert_eq!(bson, expected);
            let document = bson::doc! { "value": bson.clone() };
            let bytes = document.to_vec().unwrap();
            let decoded = bson::Document::from_reader(bytes.as_slice()).unwrap();
            assert_eq!(decoded.get("value"), Some(&expected));
            let bson::Bson::String(lexical) = bson else {
                unreachable!()
            };
            assert_eq!(
                xsd::parse(lexical, xsd::INTEGER).unwrap(),
                xsd::Value::from(value.clone())
            );
        }
    }
}

#[test]
fn bson_keeps_exact_decimal128_integers_at_precision_boundary() {
    // Trailing zeros allow some integers with more than 34 digits to remain exact.
    for input in [
        10_i128.pow(34) - 1,
        10_i128.pow(34),
        10_i128.pow(38),
        -(10_i128.pow(34) - 1),
        -10_i128.pow(38),
    ] {
        let expected: bson::Decimal128 = input.to_string().parse().unwrap();
        assert_eq!(
            xsd::DecimalValue::from(input).into_bson(),
            bson::Bson::Decimal128(expected)
        );
    }
}

// This is free and unencumbered software released into the public domain.

use xsd::DecimalValue;

#[test]
fn integer_widening_reports_decimal_range_limits() {
    const MAX: i128 = 79_228_162_514_264_337_593_543_950_335;
    for input in [i128::MIN, -MAX - 1, MAX + 1, i128::MAX] {
        assert!(DecimalValue::from(input).widen().is_none(), "{input}");
    }
    for input in [-MAX, -1, 0, 1, 9_007_199_254_740_993, MAX] {
        let original = DecimalValue::from(input);
        let widened = original.widen().unwrap();
        assert!(matches!(widened, DecimalValue::Decimal(_)));
        assert_eq!(widened.to_string(), input.to_string());
        assert_eq!(widened.narrow(), Some(original));
        assert_eq!(widened.widen(), None);
    }
}

#[test]
fn bounded_integer_widening_preserves_extremes() {
    for original in [
        DecimalValue::Byte(i8::MIN),
        DecimalValue::Byte(i8::MAX),
        DecimalValue::Short(i16::MIN),
        DecimalValue::Short(i16::MAX),
        DecimalValue::Int(i32::MIN),
        DecimalValue::Int(i32::MAX),
        DecimalValue::Long(i64::MIN),
        DecimalValue::Long(i64::MAX),
    ] {
        let widened = original.widen().unwrap();
        assert_eq!(widened.to_string(), original.to_string());
        assert_eq!(widened.narrow(), Some(original));
    }
}

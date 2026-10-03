use xsd::{PrimitiveType, PrimitiveValue};

#[test]
fn raw_month_variant_does_not_validate_fields() {
    assert_eq!(PrimitiveValue::GMonth(255).to_string(), "--255");
}

#[test]
fn checked_month_validates_every_u8() {
    const FEBRUARY: Option<PrimitiveValue> = PrimitiveValue::g_month(2);
    assert_eq!(FEBRUARY.unwrap().to_string(), "--02");
    for month in u8::MIN..=u8::MAX {
        let value = PrimitiveValue::g_month(month);
        assert_eq!(value.is_some(), (1..=12).contains(&month), "{month}");
        if let Some(value) = value {
            assert_eq!(value.r#type(), PrimitiveType::GMonth);
            assert_eq!(value, PrimitiveValue::GMonth(month));
            assert_eq!(value.to_string(), format!("--{month:02}"));
        }
    }
}

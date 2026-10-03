use xsd::{PrimitiveType, PrimitiveValue};

#[test]
fn checked_year_month_preserves_year_boundaries() {
    const YEAR_ZERO: Option<PrimitiveValue> = PrimitiveValue::g_year_month(0, 1);
    assert_eq!(YEAR_ZERO.unwrap().to_string(), "0000-01");
    for (year, lexical) in [
        (i32::MIN, "-2147483648"),
        (-10000, "-10000"),
        (-1, "-0001"),
        (0, "0000"),
        (1, "0001"),
        (10000, "10000"),
        (i32::MAX, "2147483647"),
    ] {
        for month in u8::MIN..=u8::MAX {
            let value = PrimitiveValue::g_year_month(year, month);
            assert_eq!(value.is_some(), (1..=12).contains(&month));
            if let Some(value) = value {
                assert_eq!(value.r#type(), PrimitiveType::GYearMonth);
                assert_eq!(value, PrimitiveValue::GYearMonth((year, month)));
                assert_eq!(value.to_string(), format!("{lexical}-{month:02}"));
            }
        }
    }
}

#[test]
fn checked_month_day_validates_all_field_combinations() {
    const LEAP_DAY: Option<PrimitiveValue> = PrimitiveValue::g_month_day(2, 29);
    assert_eq!(LEAP_DAY.unwrap().to_string(), "--02-29");
    let days_per_month = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for month in u8::MIN..=u8::MAX {
        for day in u8::MIN..=u8::MAX {
            let expected = month
                .checked_sub(1)
                .and_then(|index| days_per_month.get(usize::from(index)))
                .is_some_and(|&last| day >= 1 && day <= last);
            let value = PrimitiveValue::g_month_day(month, day);
            assert_eq!(value.is_some(), expected, "{month}/{day}");
            if let Some(value) = value {
                assert_eq!(value.r#type(), PrimitiveType::GMonthDay);
                assert_eq!(value, PrimitiveValue::GMonthDay((month, day)));
                assert_eq!(value.to_string(), format!("--{month:02}-{day:02}"));
            }
        }
    }
}

#[test]
fn checked_day_validates_every_u8() {
    const FIRST: Option<PrimitiveValue> = PrimitiveValue::g_day(1);
    assert_eq!(FIRST.unwrap().to_string(), "---01");
    for day in u8::MIN..=u8::MAX {
        let value = PrimitiveValue::g_day(day);
        assert_eq!(value.is_some(), (1..=31).contains(&day), "{day}");
        if let Some(value) = value {
            assert_eq!(value.r#type(), PrimitiveType::GDay);
            assert_eq!(value, PrimitiveValue::GDay(day));
            assert_eq!(value.to_string(), format!("---{day:02}"));
        }
    }
}

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

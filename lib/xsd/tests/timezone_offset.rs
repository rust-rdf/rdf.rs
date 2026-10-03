use xsd::TimezoneOffset;

#[test]
fn offset_constructor_enforces_the_entire_i16_domain() {
    for minutes in i16::MIN..=i16::MAX {
        let offset = TimezoneOffset::from_minutes(minutes);
        assert_eq!(offset.is_some(), (-840..=840).contains(&minutes));
        if let Some(offset) = offset {
            assert_eq!(offset.minutes(), minutes);
        }
    }
    assert_ne!(None, Some(TimezoneOffset::UTC));
}

#[test]
fn offset_formats_xsd_timezone_syntax() {
    for (minutes, lexical) in [
        (-840, "-14:00"),
        (-1, "-00:01"),
        (0, "Z"),
        (1, "+00:01"),
        (330, "+05:30"),
        (840, "+14:00"),
    ] {
        assert_eq!(
            TimezoneOffset::from_minutes(minutes).unwrap().to_string(),
            lexical
        );
    }
}

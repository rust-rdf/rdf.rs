use xsd::TimezoneOffset;

#[test]
fn every_offset_round_trips_through_xsd_syntax() {
    for minutes in -840..=840 {
        let offset = TimezoneOffset::from_minutes(minutes).unwrap();
        assert_eq!(
            offset.to_string().parse::<TimezoneOffset>().unwrap(),
            offset
        );
    }
    for input in ["Z", "+00:00", "-00:00"] {
        assert_eq!(
            input.parse::<TimezoneOffset>().unwrap(),
            TimezoneOffset::UTC
        );
    }
}

#[test]
fn offset_parser_rejects_invalid_syntax_and_ranges() {
    use xsd::TimezoneOffsetError::{InvalidLexical, OutOfRange};
    for input in [
        "",
        "z",
        " Z",
        "Z ",
        "+1:00",
        "+0100",
        "+01",
        "+01:00:00",
        "01:00",
        "+ab:cd",
        "+０:00",
        "+01:00[UTC]",
    ] {
        assert_eq!(
            input.parse::<TimezoneOffset>(),
            Err(InvalidLexical),
            "{input}"
        );
    }
    for input in ["+14:01", "-14:01", "+15:00", "-99:59", "+00:60", "+13:99"] {
        assert_eq!(input.parse::<TimezoneOffset>(), Err(OutOfRange), "{input}");
    }
}

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

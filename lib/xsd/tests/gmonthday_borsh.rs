#![cfg(feature = "borsh")]

use xsd::{TimezoneOffset, primitive::GMonthDay};

#[test]
fn versioned_month_day_encoding_round_trips() {
    for (month, day) in [(1, 1), (2, 29), (4, 30), (12, 31)] {
        for minutes in [None, Some(-840i16), Some(0), Some(330), Some(840)] {
            let value = GMonthDay::new(month, day)
                .unwrap()
                .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
            let mut expected = vec![1, month, day, u8::from(minutes.is_some())];
            if let Some(minutes) = minutes {
                expected.extend(minutes.to_le_bytes());
            }
            assert_eq!(borsh::to_vec(&value).unwrap(), expected);
            assert_eq!(borsh::from_slice::<GMonthDay>(&expected).unwrap(), value);
            for end in 0..expected.len() {
                assert!(borsh::from_slice::<GMonthDay>(&expected[..end]).is_err());
            }
            expected.push(0);
            assert!(borsh::from_slice::<GMonthDay>(&expected).is_err());
        }
    }
}

#[test]
fn invalid_encodings_and_legacy_pairs_are_rejected() {
    for bytes in [
        &[0, 1, 1, 0][..],
        &[2, 1, 1, 0],
        &[1, 0, 1, 0],
        &[1, 2, 30, 0],
        &[1, 4, 31, 0],
        &[1, 1, 0, 0],
        &[1, 1, 1, 2],
        &[1, 1, 1, 1, 73, 3],
        &[1, 1, 1, 1, 183, 252],
    ] {
        assert_eq!(
            borsh::from_slice::<GMonthDay>(bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for month in 0..=13 {
        for day in 0..=32 {
            let legacy = [month, day];
            assert!(borsh::from_slice::<GMonthDay>(&legacy).is_err());
            let (m, d) = borsh::from_slice::<(u8, u8)>(&legacy).unwrap();
            let migrated = GMonthDay::new(m, d);
            if let Some(value) = migrated {
                assert_eq!(
                    borsh::from_slice::<GMonthDay>(&borsh::to_vec(&value).unwrap()).unwrap(),
                    value
                );
            }
            assert_eq!(
                borsh::from_slice::<GMonthDay>(&[1, month, day, 0]).ok(),
                migrated
            );
        }
    }
}

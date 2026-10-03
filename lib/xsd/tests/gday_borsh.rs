#![cfg(feature = "borsh")]

use xsd::{TimezoneOffset, primitive::GDay};

#[test]
fn versioned_day_encoding_round_trips() {
    for day in 1..=31 {
        for minutes in [None, Some(-840i16), Some(0), Some(330), Some(840)] {
            let value = GDay::new(day)
                .unwrap()
                .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
            let mut expected = vec![1, day, u8::from(minutes.is_some())];
            if let Some(minutes) = minutes {
                expected.extend(minutes.to_le_bytes());
            }
            assert_eq!(borsh::to_vec(&value).unwrap(), expected);
            assert_eq!(borsh::from_slice::<GDay>(&expected).unwrap(), value);
            for end in 0..expected.len() {
                assert!(borsh::from_slice::<GDay>(&expected[..end]).is_err());
            }
            expected.push(0);
            assert!(borsh::from_slice::<GDay>(&expected).is_err());
        }
    }
}

#[test]
fn invalid_and_legacy_encodings_are_rejected() {
    for bytes in [
        &[0, 1, 0][..],
        &[2, 1, 0],
        &[1, 0, 0],
        &[1, 32, 0],
        &[1, 255, 0],
        &[1, 1, 2],
        &[1, 1, 1, 73, 3],
        &[1, 1, 1, 183, 252],
    ] {
        assert_eq!(
            borsh::from_slice::<GDay>(bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for legacy in 0..=255u8 {
        assert!(borsh::from_slice::<GDay>(&[legacy]).is_err());
        let day = borsh::from_slice::<u8>(&[legacy]).unwrap();
        assert_eq!(GDay::new(day).is_some(), (1..=31).contains(&legacy));
    }
}

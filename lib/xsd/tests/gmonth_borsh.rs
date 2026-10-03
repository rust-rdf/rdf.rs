#![cfg(feature = "borsh")]

use xsd::{TimezoneOffset, primitive::GMonth};

#[test]
fn versioned_month_encoding_round_trips() {
    for month in 1..=12 {
        for minutes in [None, Some(-840i16), Some(0), Some(330), Some(840)] {
            let value = GMonth::new(month)
                .unwrap()
                .with_timezone(minutes.map(|m| TimezoneOffset::from_minutes(m).unwrap()));
            let mut expected = vec![1, month, u8::from(minutes.is_some())];
            if let Some(minutes) = minutes {
                expected.extend(minutes.to_le_bytes());
            }
            assert_eq!(borsh::to_vec(&value).unwrap(), expected);
            assert_eq!(borsh::from_slice::<GMonth>(&expected).unwrap(), value);
            for end in 0..expected.len() {
                assert!(borsh::from_slice::<GMonth>(&expected[..end]).is_err());
            }
            expected.push(0);
            assert!(borsh::from_slice::<GMonth>(&expected).is_err());
        }
    }
}

#[test]
fn invalid_and_legacy_encodings_are_rejected() {
    for bytes in [
        &[0, 1, 0][..],
        &[2, 1, 0],
        &[1, 0, 0],
        &[1, 13, 0],
        &[1, 255, 0],
        &[1, 1, 2],
        &[1, 1, 1, 73, 3],
        &[1, 1, 1, 183, 252],
    ] {
        assert_eq!(
            borsh::from_slice::<GMonth>(bytes).unwrap_err().kind(),
            borsh::io::ErrorKind::InvalidData
        );
    }
    for legacy in 0..=255u8 {
        assert!(borsh::from_slice::<GMonth>(&[legacy]).is_err());
        let month = borsh::from_slice::<u8>(&[legacy]).unwrap();
        assert_eq!(GMonth::new(month).is_some(), (1..=12).contains(&legacy));
    }
}

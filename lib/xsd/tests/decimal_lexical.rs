// This is free and unencumbered software released into the public domain.

#[test]
fn rejects_non_xsd_decimal_spellings() {
    for input in [
        "1e2", "1E-2", "1_000", "NaN", "INF", "", "+", "-", ".", "+.", " 1", "1 ", "1\n", "1.2.3",
        "--1", "１２", "1٫5",
    ] {
        assert!(
            matches!(
                xsd::parse_decimal(input),
                Err(xsd::ParseDecimalError::InvalidLexical)
            ),
            "{input:?}"
        );
        for datatype in [xsd::DECIMAL, xsd::PrimitiveType::Decimal.into()] {
            assert!(
                matches!(xsd::parse(input, &datatype),
                Err(xsd::ParseError::InvalidDecimal { datatype: actual, .. })
                if actual == datatype),
                "{input:?}"
            );
        }
    }
}

#[test]
fn distinguishes_lexical_errors_from_backend_limits() {
    use core::error::Error;

    let lexical = xsd::parse_decimal("1e2").unwrap_err();
    assert!(lexical.source().is_none());
    assert_eq!(lexical.to_string(), "invalid XSD decimal lexical form");
    let range = xsd::parse_decimal("79228162514264337593543950336").unwrap_err();
    assert!(matches!(range, xsd::ParseDecimalError::Backend(_)));
    assert!(range.source().is_some());
}

#[test]
fn accepts_xsd_decimal_spellings() {
    for input in ["0", "-0", "+0", "01", "1.", ".5", "+.5", "-.5", "001.250"] {
        assert!(xsd::parse_decimal(input).is_ok(), "{input:?}");
    }
}

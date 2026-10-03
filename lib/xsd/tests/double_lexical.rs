// This is free and unencumbered software released into the public domain.

#[test]
fn double_accepts_xsd_11_lexical_forms() {
    for input in [
        "0", "-0", "+0", "01", "1.", ".5", "-.5", "+.5", "1.25", "1e2", "1.E+2", ".5e-2", "INF",
        "+INF", "-INF", "NaN", "1e9999", "1e-9999",
    ] {
        let value = xsd::parse_double(input).unwrap();
        assert_eq!(value.r#type(), xsd::DOUBLE);
        assert_eq!(xsd::parse(input, xsd::DOUBLE).unwrap(), value);
    }
    assert_eq!(
        xsd::parse_double("9007199254740991").unwrap(),
        xsd::Value::from(9007199254740991_f64)
    );
    assert_eq!(xsd::parse_double(".5e2").unwrap(), xsd::Value::from(50_f64));
}

#[test]
fn double_rejects_non_xsd_lexical_forms() {
    for input in [
        "inf",
        "infinity",
        "Infinity",
        "INFINITY",
        "nan",
        "NAN",
        "+NaN",
        "-NaN",
        "+inf",
        "-infinity",
        "",
        "+",
        "-",
        ".",
        "1e",
        "1e+",
        "1e2e3",
        "1.2.3",
        "1_000",
        "0x1p0",
        " 1",
        "1 ",
        "1\n",
        "１",
        "1,5",
    ] {
        assert!(
            matches!(
                xsd::parse_double(input),
                Err(xsd::ParseDoubleError::InvalidLexical)
            ),
            "{input:?}"
        );
        assert!(
            matches!(
                xsd::parse(input, xsd::DOUBLE),
                Err(xsd::ParseError::InvalidFloat {
                    datatype: xsd::PrimitiveType::Double,
                    ..
                })
            ),
            "{input:?}"
        );
    }
}

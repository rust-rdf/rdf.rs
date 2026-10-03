// This is free and unencumbered software released into the public domain.

#[test]
fn float_accepts_xsd_11_lexical_forms() {
    for input in [
        "0", "-0", "+0", "01", "1.", ".5", "-.5", "+.5", "1.25", "1e2", "1.E+2", ".5e-2", "INF",
        "+INF", "-INF", "NaN", "1e9999", "1e-9999",
    ] {
        let value = xsd::parse_float(input).unwrap();
        assert_eq!(value.r#type(), xsd::FLOAT);
        assert_eq!(xsd::parse(input, xsd::FLOAT).unwrap(), value);
    }
    assert_eq!(xsd::parse_float(".5e2").unwrap(), xsd::Value::from(50_f32));
}

#[test]
fn float_rejects_non_xsd_lexical_forms() {
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
                xsd::parse_float(input),
                Err(xsd::ParseFloatError::InvalidLexical)
            ),
            "{input:?}"
        );
        assert!(
            matches!(
                xsd::parse(input, xsd::FLOAT),
                Err(xsd::ParseError::InvalidFloat {
                    datatype: xsd::PrimitiveType::Float,
                    ..
                })
            ),
            "{input:?}"
        );
    }
}

#[test]
fn floating_point_error_chains_distinguish_lexical_and_backend_failures() {
    use core::error::Error;
    let error = xsd::parse("nan", xsd::FLOAT).unwrap_err();
    let source = error.source().unwrap();
    assert_eq!(
        source.downcast_ref::<xsd::ParseFloatError>(),
        Some(&xsd::ParseFloatError::InvalidLexical)
    );
    assert!(source.source().is_none());
    let native = "not-a-number".parse::<f32>().unwrap_err();
    let wrapped = xsd::ParseFloatError::from(native.clone());
    assert_eq!(
        wrapped
            .source()
            .unwrap()
            .downcast_ref::<core::num::ParseFloatError>(),
        Some(&native)
    );
    assert_eq!(wrapped.to_string(), native.to_string());
}

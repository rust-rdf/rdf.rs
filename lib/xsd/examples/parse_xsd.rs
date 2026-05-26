// This is free and unencumbered software released into the public domain.

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let _value = xsd::parse("Hello, world!", xsd::STRING)?;
    let _value = xsd::parse("true", xsd::BOOLEAN)?;
    let _value = xsd::parse("3.1415", xsd::DOUBLE)?;
    let _value = xsd::parse("42", xsd::INT)?;
    let _value = xsd::parse("2026-12-31", xsd::DATE)?;
    let _value = xsd::parse("2026-12-31T12:34:56", xsd::DATE_TIME)?;

    let _value: xsd::Value = "Hello, world!".into();
    let _value: xsd::Value = true.into();
    let _value: xsd::Value = 3.1415.into();
    let _value: xsd::Value = 42.into();

    Ok(()) // TODO
}

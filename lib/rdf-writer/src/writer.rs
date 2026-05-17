// This is free and unencumbered software released into the public domain.

pub use rdf_format::Format;
pub use rdf_model::Statement;

pub trait Writer {
    type Error;
    type Statement: Statement;

    fn format(&self) -> Format;

    fn write_statement(&mut self, statement: &Self::Statement) -> Result<(), Self::Error>;
}

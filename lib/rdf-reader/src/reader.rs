// This is free and unencumbered software released into the public domain.

pub use rdf_format::Format;
pub use rdf_model::Source;

pub trait Reader: Source {
    fn format(&self) -> Format;
}

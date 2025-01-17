// This is free and unencumbered software released into the public domain.

//! ```rust
//! # use rdf_borsh::*;
//! let reader = BorshReader::new(Box::new(std::io::stdin()));
//! let writer = BorshWriter::new(Box::new(std::io::stdout()));
//! ```

#![no_std]
#![deny(unsafe_code)]

mod borsh_dataset;
pub use borsh_dataset::*;

mod borsh_quad;
pub use borsh_quad::*;

mod borsh_reader;
pub use borsh_reader::*;

mod borsh_statement;
pub use borsh_statement::*;

mod borsh_term;
pub use borsh_term::*;

mod borsh_term_id;
pub use borsh_term_id::*;

mod borsh_triple;
pub use borsh_triple::*;

mod borsh_writer;
pub use borsh_writer::*;

mod parse;
pub use parse::parse_dataset;
pub(crate) use parse::*;

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use super::*;
    use alloc::boxed::Box;
    use rdf_model::HeapTerm;
    use rdf_writer::Writer;

    #[test]
    fn borsh_roundtrip() -> Result<(), std::io::Error> {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::println!("{:?}", temp_file);
        let mut w = BorshWriter::new(Box::new(temp_file.reopen().unwrap()))?;

        w.write_statement(&BorshStatement::from((
            HeapTerm::from("foo-1").into(),
            HeapTerm::from("bar-1").into(),
            HeapTerm::from("baz-1").into(),
            HeapTerm::from("qux-1").into(),
        )))
        .unwrap();

        w.write_statement(&BorshStatement::from((
            HeapTerm::from("foo-2").into(),
            HeapTerm::from("bar-2").into(),
            HeapTerm::from("baz-2").into(),
            HeapTerm::from("qux-2").into(),
        )))
        .unwrap();

        std::println!("{:?}", w);

        w.finish().unwrap();

        let mut r = BorshReader::new(temp_file).unwrap();

        assert_eq!(r.len(), 2);

        let stmt = r.next().unwrap().unwrap();
        assert_eq!(stmt.subject().as_str(), "foo-1");
        assert_eq!(stmt.predicate().as_str(), "bar-1");
        assert_eq!(stmt.object().as_str(), "baz-1");
        assert_eq!(stmt.context().unwrap().as_str(), "qux-1");
        std::println!("{stmt:?}");

        let stmt = r.next().unwrap().unwrap();
        assert_eq!(stmt.subject().as_str(), "foo-2");
        assert_eq!(stmt.predicate().as_str(), "bar-2");
        assert_eq!(stmt.object().as_str(), "baz-2");
        assert_eq!(stmt.context().unwrap().as_str(), "qux-2");
        std::println!("{stmt:?}");

        Ok(())
    }
}

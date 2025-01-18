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
    use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};
    use borsh::io::Read;
    use rdf_model::{HeapQuad, HeapTerm};
    use rdf_writer::Writer;

    #[test]
    fn borsh_roundtrip() -> Result<(), std::io::Error> {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        std::println!("{:?}", temp_file);
        let mut w = BorshWriter::new(Box::new(temp_file.reopen().unwrap()))?;

        let orig_stmts = vec![
            HeapQuad::from((
                HeapTerm::from("foo-1").into(),
                HeapTerm::from("bar-1").into(),
                HeapTerm::from("baz-1").into(),
                HeapTerm::from("qux-1").into(),
            )),
            HeapQuad::from((
                HeapTerm::from("foo-2").into(),
                HeapTerm::from("bar-2").into(),
                HeapTerm::from("baz-2").into(),
                HeapTerm::from("qux-2").into(),
            )),
        ];

        orig_stmts
            .iter()
            .try_for_each(|stmt| w.write_statement(stmt))
            .unwrap();

        std::println!("{:?}", w);

        w.finish().unwrap();

        let mut buf = Vec::new();
        temp_file.reopen().unwrap().read_to_end(&mut buf).unwrap();
        std::println!("{buf:?}");
        let parser_stmts: Vec<HeapQuad> = {
            let (terms, quads) = parse_dataset(&mut buf.as_slice()).unwrap();

            let dict = terms.into_iter().fold(BTreeMap::new(), |mut acc, term| {
                let id = BorshTermId::from(acc.len() as u16 + 1);
                acc.insert(id, term);
                acc
            });

            std::println!("{dict:?} ; {quads:?}");

            quads
                .into_iter()
                .map(
                    |BorshQuad {
                         context,
                         subject,
                         predicate,
                         object,
                     }| {
                        HeapQuad::from((
                            dict.get(&subject).unwrap().clone(),
                            dict.get(&predicate).unwrap().clone(),
                            dict.get(&object).unwrap().clone(),
                            dict.get(&context).unwrap().clone(),
                        ))
                    },
                )
                .collect()
        };

        let reader_stmts: Vec<HeapQuad> = {
            BorshReader::new(temp_file)
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
                .into_iter()
                .map(|stmt| {
                    HeapQuad::from((
                        stmt.subject().into(),
                        stmt.predicate().into(),
                        stmt.object().into(),
                        stmt.context().unwrap().into(),
                    ))
                })
                .collect()
        };

        assert_eq!(orig_stmts, parser_stmts);
        assert_eq!(orig_stmts, reader_stmts);

        Ok(())
    }
}

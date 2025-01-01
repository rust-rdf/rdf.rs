// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{BorshDataset, BorshTermId};
use alloc::{boxed::Box, vec::Vec};
use borsh::{
    io::{Result, Write},
    BorshSerialize,
};
use rdf_model::{Statement, Term};
use rdf_writer::{Format, Writer};

pub struct BorshWriter {
    #[allow(unused)]
    sink: Box<dyn Write>,
    #[allow(unused)]
    dataset: BorshDataset,
}

impl BorshWriter {
    pub fn new(sink: Box<dyn Write>) -> Self {
        Self {
            sink,
            dataset: BorshDataset::new(),
        }
    }

    pub fn intern_term(&mut self, term: &dyn Term) -> Result<BorshTermId> {
        Ok(self.dataset.intern_term(term.into()))
    }

    pub fn finish(mut self) -> Result<()> {
        let mut buffer = Vec::new();
        self.dataset.serialize(&mut buffer)?;
        self.sink.write_all(&buffer)?;
        Ok(())
    }
}

impl Writer for BorshWriter {
    type Error = borsh::io::Error;

    fn format(&self) -> Format {
        todo!() // TODO
    }

    fn write_statement(&mut self, statement: &dyn Statement) -> Result<()> {
        let s = self.dataset.intern_term(statement.subject().into());
        let p = self.dataset.intern_term(statement.predicate().into());
        let o = self.dataset.intern_term(statement.object().into());
        let c = statement
            .context()
            .map(|c| self.dataset.intern_term(c.into()));
        self.dataset.insert_quad((s, p, o, c).into());
        Ok(())
    }
}

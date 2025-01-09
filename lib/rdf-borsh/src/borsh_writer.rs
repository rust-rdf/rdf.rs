// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{BorshDataset, BorshTermId};
use alloc::boxed::Box;
use borsh::{
    io::{Result, Write},
    BorshSerialize,
};
use lz4_flex::frame::FrameEncoder;
use rdf_model::{Statement, Term};
use rdf_writer::{Format, Writer};

pub struct BorshWriter {
    #[allow(unused)]
    sink: Box<dyn Write>,
    #[allow(unused)]
    dataset: BorshDataset,
}

impl BorshWriter {
    pub fn new(sink: Box<dyn Write>) -> Result<Self> {
        Ok(Self {
            sink,
            dataset: BorshDataset::new(),
        })
    }

    pub fn quad_count(&self) -> usize {
        self.dataset.quads_set.len()
    }

    pub fn intern_term(&mut self, term: &dyn Term) -> Result<BorshTermId<u16>> {
        self.dataset
            .intern_term(term.into())
            .map_err(|_| borsh::io::Error::other("term dictionary overflow"))
    }

    #[allow(unused_mut)]
    pub fn finish(mut self) -> Result<()> {
        let version = 0u8;
        self.sink.write_all(&[version])?;

        let quad_count = self.dataset.quad_count();
        self.sink.write_all(quad_count.to_le_bytes().as_ref())?;

        let mut compressor = FrameEncoder::new(self.sink);
        self.dataset.serialize(&mut compressor)?;
        compressor.finish()?;

        Ok(())
    }
}

impl Writer for BorshWriter {
    type Error = borsh::io::Error;

    fn format(&self) -> Format {
        todo!() // TODO
    }

    fn write_statement(&mut self, statement: &dyn Statement) -> Result<()> {
        let s = self.intern_term(statement.subject().into())?;
        let p = self.intern_term(statement.predicate().into())?;
        let o = self.intern_term(statement.object().into())?;
        let c = match statement.context() {
            Some(c) => self.intern_term(c.into())?,
            None => BorshTermId::default(),
        };
        self.dataset.insert_quad((s, p, o, c).into());
        Ok(())
    }
}

// This is free and unencumbered software released into the public domain.

use crate::{BorshDataset, BorshTermId, FLAGS, MAGIC_NUMBER, VERSION_NUMBER};
use alloc::boxed::Box;
use borsh::{
    BorshSerialize,
    io::{Result, Write},
};
use core::marker::PhantomData;
use lz4_flex::frame::FrameEncoder;
use num_integer::Integer;
use rdf_model::{HeapQuad, Term};
use rdf_writer::{Format, Writer};

pub struct BorshWriter<T: Integer> {
    #[allow(unused)]
    sink: Box<dyn Write>,
    #[allow(unused)]
    dataset: BorshDataset,
    _marker: PhantomData<T>,
}

impl<T: Integer> core::fmt::Debug for BorshWriter<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BorshWriter")
            .field("dataset", &self.dataset)
            .finish()
    }
}

impl<T: Integer> BorshWriter<T> {
    pub fn new(sink: Box<dyn Write>) -> Result<Self> {
        Ok(Self {
            sink,
            dataset: BorshDataset::new(),
            _marker: PhantomData,
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
        self.sink.write_all(&MAGIC_NUMBER)?;
        self.sink.write_all(&[VERSION_NUMBER])?;
        self.sink.write_all(&[FLAGS])?;

        let quad_count = self.dataset.quad_count() as u32;
        self.sink.write_all(quad_count.to_le_bytes().as_ref())?;

        let mut compressor = FrameEncoder::new(self.sink);
        self.dataset.serialize(&mut compressor)?;
        compressor.finish()?;

        Ok(())
    }
}

impl<T: Integer> Writer for BorshWriter<T> {
    type Error = borsh::io::Error;
    type Statement = HeapQuad;

    fn format(&self) -> Format {
        todo!() // TODO
    }

    fn write_statement(&mut self, statement: &Self::Statement) -> Result<()> {
        let s = self.intern_term(statement.subject())?;
        let p = self.intern_term(statement.predicate())?;
        let o = self.intern_term(statement.object())?;
        let c = match statement.context() {
            Some(c) => self.intern_term(c)?,
            None => BorshTermId::default(),
        };
        self.dataset.insert_quad((s, p, o, c).into());
        Ok(())
    }
}

// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use borsh::io::{Read, Result};
use core::error::Error;
use lz4_flex::frame::FrameDecoder;
use rdf_model::{
    Countable, Enumerable, MaybeDurable, MaybeIndexed, MaybeMutable, Source, Statement,
};
use rdf_reader::{Format, Reader};

use crate::{parse_header, BorshQuad, BorshStatement, BorshTerm, BorshTermId};

pub struct BorshReader<R: Read> {
    decompressor: FrameDecoder<R>,

    term_dict: BTreeMap<BorshTermId<u16>, BorshTerm>,

    quad_count: usize,
    read_count: usize,
}

impl<R: Read> BorshReader<R> {
    pub fn new(mut source: R) -> Result<Self> {
        let quad_count = {
            let mut buf = [0u8; 10];
            source.read_exact(&mut buf)?;
            parse_header(&mut &buf[..]).unwrap()
        };

        let _term_block_size = {
            let mut buf = [0u8; 4];
            source.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        let term_dict = {
            let mut reader = FrameDecoder::new(&mut source);
            borsh::from_reader::<_, Vec<BorshTerm>>(&mut reader)?
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, term| {
                    let id = BorshTermId::from(acc.len() as u16 + 1);
                    acc.insert(id, term);
                    acc
                })
        };

        let _quad_block_size = {
            let mut buf = [0u8; 4];
            source.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        let _quad_count = {
            let mut buf = [0u8; 4];
            source.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        // rest of `source` is the quads section block

        Ok(Self {
            decompressor: FrameDecoder::new(source),
            quad_count: quad_count as _,
            term_dict,
            read_count: 0usize,
        })
    }
}

impl<R: Read> Reader for BorshReader<R> {
    fn format(&self) -> Format {
        todo!() // TODO
    }
}

impl<R: Read> Source for BorshReader<R> {}
impl<R: Read> Enumerable for BorshReader<R> {}
impl<R: Read> MaybeDurable for BorshReader<R> {}
impl<R: Read> MaybeIndexed for BorshReader<R> {}
impl<R: Read> MaybeMutable for BorshReader<R> {}

impl<R: Read> Countable for BorshReader<R> {
    fn count(&self) -> usize {
        self.quad_count
    }
}

impl<R: Read> Iterator for BorshReader<R> {
    type Item = core::result::Result<Box<dyn Statement>, Box<dyn Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len() == 0 {
            return None;
        }

        let quad = borsh::from_reader::<_, BorshQuad<u16>>(&mut self.decompressor).unwrap();

        let s = self.term_dict.get(&quad.subject).unwrap().clone();
        let p = self.term_dict.get(&quad.predicate).unwrap().clone();
        let o = self.term_dict.get(&quad.object).unwrap().clone();
        let g = self.term_dict.get(&quad.context).unwrap().clone();

        let stmt = BorshStatement::from((s, p, o, g));

        self.read_count += 1;

        Some(Ok(Box::new(stmt)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.quad_count - self.read_count;
        (rem, Some(rem))
    }
}

impl<R: Read> ExactSizeIterator for BorshReader<R> {}

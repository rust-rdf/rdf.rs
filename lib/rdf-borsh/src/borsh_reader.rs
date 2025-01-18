// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};
use borsh::{
    io::{Read, Result},
    BorshDeserialize,
};
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
            parse_header(&mut &buf[..]).map_err(|_| borsh::io::ErrorKind::InvalidData)?
        };

        let mut decompressor = FrameDecoder::new(source);

        let term_dict = {
            Vec::<BorshTerm>::deserialize_reader(&mut decompressor)?
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, term| {
                    let id = BorshTermId::from(acc.len() as u16 + 1);
                    acc.insert(id, term);
                    acc
                })
        };

        let _quad_count = {
            let mut buf = [0u8; 4];
            decompressor.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        // rest of `source` is the quads section block

        Ok(Self {
            decompressor,
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

        let quad = match BorshQuad::<u16>::deserialize_reader(&mut self.decompressor) {
            Ok(q) => q,
            Err(e) => return Some(Err(Box::new(e))),
        };

        let Some(s) = self.term_dict.get(&quad.subject) else {
            return Some(Err(Box::new(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "subject has unknown term id",
            ))));
        };
        let Some(p) = self.term_dict.get(&quad.predicate) else {
            return Some(Err(Box::new(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "predicate has unknown term id",
            ))));
        };
        let Some(o) = self.term_dict.get(&quad.object) else {
            return Some(Err(Box::new(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "object has unknown term id",
            ))));
        };
        let Some(g) = self.term_dict.get(&quad.context) else {
            return Some(Err(Box::new(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "context has unknown term id",
            ))));
        };

        let stmt = BorshStatement::from((s.clone(), p.clone(), o.clone(), g.clone()));

        self.read_count += 1;

        Some(Ok(Box::new(stmt)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.quad_count - self.read_count;
        (rem, Some(rem))
    }
}

impl<R: Read> ExactSizeIterator for BorshReader<R> {}

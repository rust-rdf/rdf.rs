// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::boxed::Box;
use borsh::io::{Read, Result};
use core::error::Error;
use lz4_flex::frame::FrameDecoder;
use rdf_model::{
    Countable, Enumerable, MaybeDurable, MaybeIndexed, MaybeMutable, Source, Statement,
};
use rdf_reader::{Format, Reader};

pub struct BorshReader<R: Read> {
    #[allow(unused)]
    decompressor: FrameDecoder<R>,
    #[allow(unused)]
    quad_count: usize,
}

impl<R: Read> BorshReader<R> {
    pub fn new(mut source: R) -> Result<Self> {
        let mut header: [u8; 5] = [0; 5];
        source.read_exact(&mut header)?;

        let version = header[0];
        if version != 0 {
            //return Err(()); // TODO
        }

        let quad_count = u32::from_le_bytes([header[1], header[2], header[3], header[4]]);

        Ok(Self {
            decompressor: FrameDecoder::new(source),
            quad_count: quad_count as _,
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
        None // TODO
    }
}

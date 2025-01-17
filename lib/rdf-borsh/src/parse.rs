// This is free and unencumbered software released into the public domain.

extern crate alloc;

use alloc::vec::Vec;
use borsh::io::Read;
use lz4_flex::frame::FrameDecoder;
use rdf_model::HeapTerm;
use winnow::{
    binary::{le_u16, le_u32, length_take, u8},
    combinator::{dispatch, fail},
    error::{ContextError, ErrMode, StrContext, StrContextValue},
    PResult, Parser,
};

use crate::{BorshQuad, BorshTerm};

#[derive(Debug)]
pub enum ParseError {
    Io(borsh::io::Error),
    Parse(ErrMode<ContextError>),
}

pub fn parse_dataset(
    input: &mut &[u8],
) -> Result<(Vec<BorshTerm>, Vec<BorshQuad<u16>>), ParseError> {
    let _quad_count = parse_header(input).map_err(ParseError::Parse)?;

    let mut decompressor = FrameDecoder::new(input);
    let mut buf = Vec::new();
    decompressor.read_to_end(&mut buf).map_err(ParseError::Io)?;

    let mut input = buf.as_slice();
    let terms = parse_term_dictionary(&mut input).map_err(ParseError::Parse)?;
    let quads = parse_quads_section(&mut input).map_err(ParseError::Parse)?;

    Ok((terms, quads))
}

pub fn parse_header(input: &mut &[u8]) -> PResult<u32> {
    b"RDFB"
        .context(StrContext::Label("magic number"))
        .context(StrContext::Expected(StrContextValue::StringLiteral("RDFB")))
        .parse_next(input)?;
    b'\x01'
        .context(StrContext::Label("version number"))
        .parse_next(input)?;
    0b00000111_u8
        .context(StrContext::Label("flags"))
        .parse_next(input)?;
    le_u32
        .context(StrContext::Label("number of quads"))
        .parse_next(input)
}

pub fn parse_term_dictionary(input: &mut &[u8]) -> PResult<Vec<BorshTerm>> {
    let term_count = le_u32
        .context(StrContext::Label("term dictionary count"))
        .parse_next(input)?;
    let mut terms = Vec::with_capacity(term_count as usize);
    for _ in 0..term_count {
        terms.push(parse_term.parse_next(input)?)
    }
    Ok(terms)
}

pub fn parse_quads_section(input: &mut &[u8]) -> PResult<Vec<BorshQuad<u16>>> {
    let quad_count = le_u32
        .context(StrContext::Label("quad count"))
        .parse_next(input)?;
    let mut quads = Vec::with_capacity(quad_count as usize);
    for _ in 0..quad_count {
        quads.push(parse_quad.parse_next(input)?)
    }
    Ok(quads)
}

pub fn parse_term(input: &mut &[u8]) -> PResult<BorshTerm> {
    dispatch!(u8;
        0x00_u8 => parse_iri,
        0x01_u8 => parse_blank_node,
        0x02_u8 => parse_plain_literal,
        0x03_u8 => parse_typed_literal,
        0x04_u8 => parse_tagged_literal,
        _ => fail,
    )
    .context(StrContext::Label("term"))
    .parse_next(input)
}

pub fn parse_quad(input: &mut &[u8]) -> PResult<BorshQuad<u16>> {
    let (g, s, p, o) = (le_u16, le_u16, le_u16, le_u16)
        .context(StrContext::Label("quad"))
        .parse_next(input)?;

    Ok(BorshQuad::new(s.into(), p.into(), o.into(), g.into()))
}

pub fn parse_iri(input: &mut &[u8]) -> PResult<BorshTerm> {
    let to_term = |data| {
        core::str::from_utf8(data)
            .map(HeapTerm::iri)
            .map(BorshTerm::from)
    };

    length_take(le_u32).try_map(to_term).parse_next(input)
}

pub fn parse_blank_node(input: &mut &[u8]) -> PResult<BorshTerm> {
    let to_term = |data| {
        core::str::from_utf8(data)
            .map(HeapTerm::bnode)
            .map(BorshTerm::from)
    };

    length_take(le_u32).try_map(to_term).parse_next(input)
}

pub fn parse_plain_literal(input: &mut &[u8]) -> PResult<BorshTerm> {
    let to_term = |data| {
        core::str::from_utf8(data)
            .map(HeapTerm::literal)
            .map(BorshTerm::from)
    };

    length_take(le_u32).try_map(to_term).parse_next(input)
}

pub fn parse_typed_literal(input: &mut &[u8]) -> PResult<BorshTerm> {
    let to_term = |(data, datatype)| {
        let data = core::str::from_utf8(data)?;
        let datatype = core::str::from_utf8(datatype)?;
        Ok::<BorshTerm, core::str::Utf8Error>(BorshTerm::from(HeapTerm::literal_with_datatype(
            data, datatype,
        )))
    };

    (length_take(le_u32), length_take(le_u32))
        .try_map(to_term)
        .parse_next(input)
}

pub fn parse_tagged_literal(input: &mut &[u8]) -> PResult<BorshTerm> {
    let to_term = |(data, tag)| {
        let data = core::str::from_utf8(data)?;
        let tag = core::str::from_utf8(tag)?;
        Ok::<BorshTerm, core::str::Utf8Error>(BorshTerm::from(HeapTerm::literal_with_language(
            data, tag,
        )))
    };

    (length_take(le_u32), length_take(le_u32))
        .try_map(to_term)
        .parse_next(input)
}

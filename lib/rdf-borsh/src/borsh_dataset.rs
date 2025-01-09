// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{BorshQuad, BorshTerm, BorshTermId, BorshTriple};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::FromPrimitive;

#[derive(Clone, Debug, Default, Eq, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BorshDataset {
    pub terms_dict: Vec<BorshTerm>,
    pub quads_set: BTreeSet<BorshQuad<u16>>,
    #[borsh(skip)]
    terms_map: BTreeMap<BorshTerm, BorshTermId<u16>>,
}

impl BorshDataset {
    pub fn new() -> Self {
        Self {
            terms_dict: Vec::new(),
            quads_set: BTreeSet::new(),
            terms_map: BTreeMap::new(),
        }
    }

    pub fn quad_count(&self) -> usize {
        self.quads_set.len()
    }

    pub fn intern_term(&mut self, term: BorshTerm) -> core::result::Result<BorshTermId<u16>, ()> {
        if let Some(&term_id) = self.terms_map.get(&term) {
            return Ok(term_id);
        }
        let term_id: BorshTermId<u16> =
            BorshTermId::from_usize(self.terms_dict.len() + 1).ok_or(())?;
        self.terms_dict.push(term.clone());
        self.terms_map.insert(term, term_id);
        Ok(term_id)
    }

    pub fn insert_triple(&mut self, triple: BorshTriple<u16>) -> bool {
        self.quads_set.insert(triple.into())
    }

    pub fn insert_quad(&mut self, quad: BorshQuad<u16>) -> bool {
        self.quads_set.insert(quad)
    }
}

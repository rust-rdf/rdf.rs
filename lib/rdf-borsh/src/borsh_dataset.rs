// This is free and unencumbered software released into the public domain.

extern crate alloc;

use crate::{BorshQuad, BorshTerm, BorshTermId, BorshTriple};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct BorshDataset {
    pub terms_dict: Vec<BorshTerm>,
    pub quads_set: BTreeSet<BorshQuad>,
    #[borsh(skip)]
    terms_map: BTreeMap<BorshTerm, BorshTermId>,
}

impl BorshDataset {
    pub fn new() -> Self {
        Self {
            terms_dict: Vec::new(),
            quads_set: BTreeSet::new(),
            terms_map: BTreeMap::new(),
        }
    }

    pub fn intern_term(&mut self, term: BorshTerm) -> BorshTermId {
        if let Some(&term_id) = self.terms_map.get(&term) {
            return term_id;
        }
        let term_id = self.terms_dict.len() as BorshTermId;
        self.terms_dict.push(term.clone());
        self.terms_map.insert(term, term_id);
        term_id
    }

    pub fn insert_triple(&mut self, triple: BorshTriple) -> bool {
        self.quads_set.insert(triple.into())
    }

    pub fn insert_quad(&mut self, quad: BorshQuad) -> bool {
        self.quads_set.insert(quad)
    }
}

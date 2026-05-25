// This is free and unencumbered software released into the public domain.

use rdf_model::CowTerm;

pub const HTML: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML");

pub const LANG_STRING: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString");

pub const PLAIN_LITERAL: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#PlainLiteral");

pub const TYPE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");

pub const PROPERTY: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property");

pub const STATEMENT: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement");

pub const SUBJECT: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject");

pub const PREDICATE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate");

pub const OBJECT: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#object");

pub const BAG: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag");

pub const SEQ: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq");

pub const ALT: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt");

pub const VALUE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#value");

pub const LIST: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#List");

pub const NIL: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil");

pub const FIRST: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#first");

pub const REST: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest");

pub const XML_LITERAL: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral");

pub const JSON: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON");

pub const COMPOUND_LITERAL: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#CompoundLiteral");

pub const LANGUAGE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#language");

pub const DIRECTION: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#direction");

// This is free and unencumbered software released into the public domain.

use rdf_model::CowTerm;

pub const RESOURCE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#Resource");

pub const CLASS: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#Class");

pub const SUB_CLASS_OF: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#subClassOf");

pub const SUB_PROPERTY_OF: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#subPropertyOf");

pub const COMMENT: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#comment");

pub const LABEL: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#label");

pub const DOMAIN: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#domain");

pub const RANGE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#range");

pub const SEE_ALSO: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#seeAlso");

pub const IS_DEFINED_BY: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#isDefinedBy");

pub const LITERAL: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#Literal");

pub const CONTAINER: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#Container");

pub const CONTAINER_MEMBERSHIP_PROPERTY: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty");

pub const MEMBER: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#member");

pub const DATATYPE: CowTerm<'static> =
    CowTerm::static_iri("http://www.w3.org/2000/01/rdf-schema#Datatype");

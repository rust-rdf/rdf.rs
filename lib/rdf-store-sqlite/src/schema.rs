// This is free and unencumbered software released into the public domain.

pub type SchemaVersion = u32;
pub type NodeId = u64;

pub const SCHEMA_VERSION: SchemaVersion = 1;

pub(crate) const SCHEMA_SQL: &'static str = include_str!("../etc/schema.sql");

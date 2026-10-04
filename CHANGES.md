# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.0 - 2026-10-04
### Added
- Add validated XSD timezone offsets and timezone-aware calendar values.
- Add checked calendar constructors, conversions, and validated temporal serialization.
- Add the `rdf-message` crate scaffold.
- Complete heap transactions: staged reads, clear/delete, commit/rollback, and lifecycle errors.
- Tighten XSD numeric, temporal, and duration parsing; preserve detailed parse errors.
- Preserve XSD numeric precision and lexical formatting in JSON/BSON conversions.
### Changed
- Update the MSRV to 1.97.
- Distinguish the default graph singleton from the `urn:rdf:default-graph` IRI;
  `None` remains a wildcard.
- Redesign XSD temporal representations and Borsh encodings to retain optional timezones.
- Isolate `alloc`, `std`, and optional interoperability features across the workspace.
- Update Dogma to 0.3.0, Clientele to 0.5.0, itertools to 0.15.
### Fixed
- Fix transaction defaults, complete-term matching, and default-graph round trips.
- Correct XSD datatype dispatch, negative years, timezone offsets, and end-of-day values.
- Preserve the storage traits' `Send` bounds for IndexedDB handles and errors
  when compiling the enabled adapter for browsers.

## 0.4.4 - 2026-07-02

## 0.4.3 - 2026-06-25

## 0.4.2 - 2026-06-19

## 0.4.1 - 2026-06-15

## 0.4.0 - 2026-06-11

## 0.3.4 - 2026-06-08

## 0.3.3 - 2026-06-07

## 0.3.2 - 2026-05-26

## 0.3.1 - 2026-05-26

## 0.3.0 - 2026-05-25

## 0.2.3 - 2026-05-17

## 0.2.2 - 2025-05-06

## 0.2.1 - 2025-04-18

## 0.2.0 - 2025-01-18

## 0.1.1 - 2024-12-29

## 0.1.0 - 2024-12-05

## 0.0.1 - 2024-09-08

## 0.0.0 - 2024-08-27
### Added
- Initial crate structure and interdependencies.

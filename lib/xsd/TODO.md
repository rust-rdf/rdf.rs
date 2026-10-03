# XSD enhancement backlog

This is the continuation plan for the `xsd` Cargo package in `lib/xsd`, based on
the 2026-10-02 review of version 0.4.4. It records the remaining findings,
implementation entry points, and acceptance criteria without requiring session
history. Reproduce the relevant finding against the current code before editing.

## Working approach

- Follow the repository's [AGENTS.md](../../AGENTS.md).
- Use atomic commits: select one narrowly scoped subtask, include its regression
  tests and rustdoc, and verify it before moving on. The numbered sections are
  milestones, not instructions to implement an entire section in one change.
- Start with **#3, temporal representations**. Prioritize panics and
  information loss before expanding datatype coverage. Add tests alongside each
  fix; #8 is also an ongoing concern.
- Preserve `no_std`, allocation-free configurations, and optional interoperability
  boundaries. Defaults enable `all` and `std`; `all` enables `datetime` and
  `decimal`. `datetime` enables `jiff`. `serde` and `borsh` imply `alloc`; `bson`,
  `oxrdf`, and `rudof` imply `std`. See [Cargo.toml](Cargo.toml) and
  [src/lib.rs](src/lib.rs) for actual exports and feature forwarding.
- Treat public enum changes, equality/hash changes, and serialized representations
  as compatibility decisions. Preserve existing wire tags or document and test
  the necessary version transition.
- Remove a subtask and its resolved findings after verifying its behavior, tests,
  and documentation. Keep only outstanding work and follow-ups in this file.

## 3. Introduce XSD-aware temporal representations — high priority

**Entry points:** [src/primitive/](src/primitive/), especially `date.rs`,
`datetime.rs`, `time.rs`, `duration.rs`, and the `g*.rs` files;
[src/parse.rs](src/parse.rs) and [src/primitive_value.rs](src/primitive_value.rs).

With `jiff`, `Duration` aliases `jiff::SignedDuration`; without
it, the exported alias is the unsigned `core::time::Duration`.

**Observed with default features, using `xsd::parse`:**

| Datatype | Literal | Current behavior |
| --- | --- | --- |
| `xsd::DURATION` | `P1D`, `P1M`, `P1Y` | Rejected |

**Subtasks and acceptance criteria:**

- [ ] Represent signed durations with calendar-month and day/time components.
  Do not approximate a month as a fixed number of seconds. Extend the fixed-length
  boundary regressions in `tests/duration_limits.rs` to mixed components,
  negative calendar durations, zero, and conversion limits when replacing the alias.
- [ ] Replace raw partial-calendar aliases with validated representations and
  timezone support for `GYear` and `GYearMonth`.
  Enforce validation at the representation/serialization boundary,
  building on the checked `PrimitiveValue::g_month`, `g_day`, `g_month_day`, and
  `g_year_month` constructors.
  Reuse the formatting and `tests/gyear_limits.rs` boundary regressions while
  adding timezone-aware representations. These two partial-calendar parsers
  currently reject timezone suffixes rather than discarding them.
  Verify `GMonthDay` RDF,
  comparison, and reduced-feature contracts.

## 4. Enforce numeric lexical rules and exactness — high priority

**Entry points:** [src/parse.rs](src/parse.rs), the aliases in
[src/primitive/](src/primitive/), [src/derived/integer.rs](src/derived/integer.rs),
[src/decimal_value.rs](src/decimal_value.rs), and
[src/value.rs](src/value.rs). Numeric behavior currently delegates to `valuand`.

**Observed with default features:**

- Parsing decimal `0.123456789012345678901234567890123456789` succeeds but rounds
  it to `0.1234567890123456789012345679`.
- Parsing integer `340282366920938463463374607431768211456` fails because of the
  current bounded representation.
- `xsd::PrimitiveValue::from(i128::MAX)` panics while converting the integer to the
  narrower-capacity decimal representation.

**Subtasks and acceptance criteria:**

- [ ] Document the supported XSD version and lexical-processing policy. Resolve
  version-sensitive rules such as year zero and NaN comparison. Distinguish XML
  Schema whitespace preprocessing from RDF lexical-form handling rather than
  unconditionally trimming every datatype.
- [ ] Detect unsupported decimal precision/range without silently rounding exact
  values. Document limits and expose useful errors. Consider allocation-backed
  arbitrary-precision support as a separate enhancement while preserving the
  allocation-free configuration.
- [ ] Make explicit primitive-decimal construction from large integers range-safe
  (`PrimitiveValue::from(i128)` and `Value::decimal`), using fallible APIs where
  the destination cannot represent the number. Coordinate construction policy with #6.

## 5. Make JSON/BSON conversion precise and consistent — high priority

**Entry points:** `to_json`, `into_json`, `to_bson`, `into_bson`, and conversion
traits in [src/decimal_value.rs](src/decimal_value.rs),
[src/primitive_value.rs](src/primitive_value.rs), and
[src/value.rs](src/value.rs).

**Subtasks and acceptance criteria:**

- [ ] Define fallible numeric-only conversion APIs for
  unsupported target values. Test large integers, high-precision decimals,
  non-finite floats, and target range limits. Document any explicitly lossy API.
- [ ] Document the distinction between derived Serde serialization and explicit
  JSON/BSON conversion for the remaining datatypes beyond `Date`, `Time`,
  `DateTime`, `GMonth`, `GDay`, and `GMonthDay`.
  Test the promised round-trip guarantees for each API,
  including whether datatype identity and exact numeric values survive.

## 6. Normalize datatype identity and define comparison semantics

**Entry points:** [src/type.rs](src/type.rs),
[src/primitive_type.rs](src/primitive_type.rs), [src/value.rs](src/value.rs),
[src/primitive_value.rs](src/primitive_value.rs), and
[src/decimal_value.rs](src/decimal_value.rs).

**Observed behavior:**

- `Type::from(PrimitiveType::Decimal) != xsd::DECIMAL`: two representations of
  the same datatype compare unequal. Values have corresponding duplicate paths.
- `DecimalValue::Byte(1) > DecimalValue::Short(100)` is `true`: derived ordering
  follows enum variant order, not numeric order.
- `DecimalValue::Byte(1) == DecimalValue::Short(1)` is `false`. Parsed NaN values
  compare equal to themselves under the current total-equality representation.

**Subtasks and acceptance criteria:**

- [ ] Establish one canonical datatype/value representation for decimal and
  consistent explicit primitive-decimal construction rules. Account for existing serialized
  enums and downstream matching code before changing variants.
- [ ] Specify structural equality/order separately from XSD semantic comparison.
  Provide explicit value-comparison operations where appropriate, covering numeric
  subtypes, NaN, signed zero, and partially ordered temporal values. Keep Rust
  `Eq`/`Ord`/`Hash` contracts consistent.
- [ ] Extend RDF integration regressions beyond `Date`, `Time`, `DateTime`, `GMonth`, and `GDay`:
  semantic equality must not erase datatype or lexical identity.
  For example, distinct RDF lexical forms such as `"1"` and
  `"01"` can denote the same integer without becoming the same RDF term.
- [ ] Define a lexical-preserving policy for implicit RDF literal construction:
  `HeapTerm::from((String, Datatype))` currently normalizes successfully parsed
  literals, unlike the explicit `HeapTerm::typed_literal` constructor. Account for
  callers relying on value-backed terms before changing this conversion.

## 7. Complete datatype support end to end

**Entry points:** [src/type.rs](src/type.rs) (`TYPES` and constants),
[src/decimal_type.rs](src/decimal_type.rs),
[src/derived/integer.rs](src/derived/integer.rs), value enums, and parser dispatch.

Unsigned and `NonNegativeInteger` aliases exist, but their enum variants, lookup
entries, parser dispatch, and conversions are absent. An exported alias or a
recognized datatype name is not evidence of complete support.

Implement one datatype or tightly related family per change, in this order:

- [ ] Unsigned integer datatypes and sign-constrained integer families:
  `nonNegativeInteger`, `positiveInteger`, `nonPositiveInteger`, `negativeInteger`.
- [ ] Binary parsing/validation and timezone-bearing partial-calendar parsing,
  building on the existing formatting and the representations from #3.
- [ ] `dateTimeStamp`, `dayTimeDuration`, and `yearMonthDuration` after the
  temporal representation work.
- [ ] Common string-derived datatypes: `normalizedString`, `token`, `language`.
- [ ] `anyURI` and context-aware `QName` handling. Define namespace resolution
  explicitly; a lexical prefix alone is not a namespace identity.

For each addition, require datatype lookup/constants, value representation,
validation, parsing, formatting, hierarchy information, feature behavior, and
positive/negative round-trip tests. Check serialization compatibility when
extending enums. Unsupported cases must return meaningful errors.

## 8. Extend behavioral tests and document capability boundaries

**Entry points:** [tests/](tests/), [src/lib.rs](src/lib.rs), public type/module
rustdoc, and [Cargo.toml](Cargo.toml).

- [ ] Add table-driven valid/invalid lexical cases as each parser is corrected,
  including precision, overflow, timezone, and unsupported-operation regressions.
- [ ] Add property/differential tests where useful against an independent XSD
  implementation for datatypes beyond `Date`, `Time`, `DateTime`, `GMonth`,
  `GDay`, and `GMonthDay`, using optional `oxsdatatypes`.
  Use the chosen W3C semantics
  as the authority when implementations disagree.
- [ ] Exercise runtime behavior under reduced feature sets, not just compilation.
  In particular, disabled capabilities must produce the documented errors rather
  than panics. Keep bare-metal consumer checks for genuine `no_std` coverage.
- [ ] Document public APIs' formatting, comparison, precision, errors, panics,
  feature requirements, and datatype-identifier lookup conventions. Explain the
  difference between original lexical form, value representation, and canonical
  lexical form. Use runnable examples in rustdoc.

## Verification for implementation work

Run commands from the repository root. Use the declared MSRV and locked
dependencies:

```sh
cargo +1.97.0 check -p xsd --all-targets --locked
cargo +1.97.0 test -p xsd --locked
cargo +1.97.0 clippy -p xsd --all-targets --locked
cargo +1.97.0 doc -p xsd --no-deps --locked
cargo +1.97.0 fmt --all -- --check
```

Exercise affected feature configurations and interoperability APIs. These are
useful regression configurations; use isolated interop checks as well so Cargo
feature unification cannot hide missing feature forwarding:

```sh
cargo +1.97.0 test -p xsd --no-default-features --locked
cargo +1.97.0 test -p xsd --no-default-features --features alloc --locked
cargo +1.97.0 test -p xsd --all-features --locked
RUSTUP_TOOLCHAIN=1.97.0 python3 .config/check-features.py core
RUSTUP_TOOLCHAIN=1.97.0 python3 .config/check-features.py interop
RUSTUP_TOOLCHAIN=1.97.0 python3 .config/check-features.py adapters
RUSTUP_TOOLCHAIN=1.97.0 python3 .config/check-features.py no-std
```

The feature checker is [here](../../.config/check-features.py). Adapter checks
need `wasm32-unknown-unknown`, libclang, and an ODBC driver manager. Bare-metal
checks need `thumbv7em-none-eabihf` installed for the selected toolchain. Test
`rdf-model` too when error types, conversions, or RDF literal behavior change.
Report existing failures precisely and keep unrelated fixes separate.

## Standards references

- [XSD 1.0 Part 2: Datatypes](https://www.w3.org/TR/xmlschema-2/)
- [XSD 1.1 Part 2: Datatypes](https://www.w3.org/TR/xmlschema11-2/)
- [RDF 1.1 Concepts: Literals](https://www.w3.org/TR/rdf11-concepts/#section-Graph-Literal)
- [RDF 1.2 Concepts: Literals](https://www.w3.org/TR/rdf12-concepts/#section-Graph-Literal)

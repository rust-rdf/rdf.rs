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
- Start with **#2, structured parsing diagnostics**. Prioritize panics and
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

## 2. Make parsing reliably fallible — highest priority

**Entry points:** [src/parse.rs](src/parse.rs),
[src/parse_error.rs](src/parse_error.rs), and the typed-literal conversion in
[rdf-model/src/heap/term.rs](../rdf-model/src/heap/term.rs).

**Observed behavior:**

- Boolean, decimal, floating-point, and temporal parser failures collapse to
  `ParseError::InvalidLiteral`, losing the requested datatype and underlying cause.

**Subtasks and acceptance criteria:**

Keep diagnostic error types usable without `std` or mandatory allocation.

- [ ] Retain the datatype and underlying cause for decimal parsing failures,
  including range/precision causes supplied by the backend.
- [ ] Retain the datatype and underlying cause for float/double parsing failures.
- [ ] Add datatype-aware diagnostics for boolean parsing failures.
- [ ] Retain the datatype and underlying cause for temporal parsing failures
  when `jiff` is enabled.

## 3. Introduce XSD-aware temporal representations — high priority

**Entry points:** [src/primitive/](src/primitive/), especially `date.rs`,
`datetime.rs`, `time.rs`, `duration.rs`, and the `g*.rs` files;
[src/parse.rs](src/parse.rs) and [src/primitive_value.rs](src/primitive_value.rs).

`Date`, `DateTime`, and `Time` directly alias Jiff civil types, which cannot retain
timezone offsets. With `jiff`, `Duration` aliases `jiff::SignedDuration`; without
it, the exported alias is the unsigned `core::time::Duration`.

**Observed with default features, using `xsd::parse`:**

| Datatype | Literal | Current behavior |
| --- | --- | --- |
| `xsd::DATE_TIME` | `2026-12-31T12:34:56+02:00` | Accepted; offset discarded |
| `xsd::DATE_TIME` | `2026-12-31T12:34:56Z` | Rejected |
| `xsd::DATE_TIME` | `2026-12-31T12:34:56-15:00` | Accepted; invalid XSD offset discarded |
| `xsd::DATE_TIME` | `2026-12-31 12:34:56` | Accepted despite the non-XSD separator |
| `xsd::DATE` | `2026-12-31T12:34:56` | Accepted; time discarded |
| `xsd::TIME` | `12:34:56+02:00` | Accepted; offset discarded |
| `xsd::TIME` | `12:34` | Accepted despite missing seconds |
| `xsd::TIME` | `24:00:00` | Rejected despite valid XSD end-of-day notation |
| `xsd::DURATION` | `P1D`, `P1M`, `P1Y` | Rejected |

**Subtasks and acceptance criteria:**

- [ ] Model optional timezone offsets and distinguish an absent timezone from
  UTC. Preserve the represented value through parsing, formatting, and conversion.
  Validate XSD offset bounds and make lossy Jiff conversions explicit/fallible.
- [ ] Enforce temporal lexical grammars, including end-of-day notation, leap-day
  validation, required components, and fractional seconds. Define supported year
  and fractional precision ranges; report unsupported values explicitly.
- [ ] Represent signed durations with calendar-month and day/time components.
  Do not approximate a month as a fixed number of seconds. Test mixed components,
  negative durations, zero, and conversion limits.
- [ ] Replace raw partial-calendar aliases with validated representations and
  timezone support. For example, `GMonth = u8` currently admits `0` and `255`.
  Reuse the formatting regressions while adding constructor and parser coverage.

## 4. Enforce numeric lexical rules and exactness — high priority

**Entry points:** [src/parse.rs](src/parse.rs), the aliases in
[src/primitive/](src/primitive/), [src/derived/integer.rs](src/derived/integer.rs),
[src/decimal_value.rs](src/decimal_value.rs), and
[src/value.rs](src/value.rs). Numeric behavior currently delegates to `valuand`.

**Observed with default features:**

- `xsd::parse("1e2", xsd::DECIMAL)` accepts exponent notation, which is outside
  the XSD decimal lexical grammar.
- `xsd::parse("infinity", xsd::DOUBLE)` and parsing `"nan"` accept non-XSD
  spellings; XSD uses `INF` and `NaN`.
- Parsing decimal `0.123456789012345678901234567890123456789` succeeds but rounds
  it to `0.1234567890123456789012345679`.
- Parsing integer `340282366920938463463374607431768211456` fails because of the
  current bounded representation.
- `xsd::Value::from(i128::MAX)` and
  `xsd::DecimalValue::Integer(i128::MAX.into()).widen()` panic while converting
  the integer to the narrower-capacity decimal representation.

**Subtasks and acceptance criteria:**

- [ ] Document the supported XSD version and lexical-processing policy. Resolve
  version-sensitive rules such as year zero and NaN comparison. Distinguish XML
  Schema whitespace preprocessing from RDF lexical-form handling rather than
  unconditionally trimming every datatype.
- [ ] Validate decimal, integer, float, and double lexical forms before delegating
  numeric conversion. Test both permitted alternate spellings and invalid input.
- [ ] Detect unsupported decimal precision/range without silently rounding exact
  values. Document limits and expose useful errors. Consider allocation-backed
  arbitrary-precision support as a separate enhancement while preserving the
  allocation-free configuration.
- [ ] Make integer construction and decimal widening range-safe. A widening API
  must either preserve the number exactly or report inability to represent it;
  it must not panic. Coordinate construction policy with #6.

## 5. Make JSON/BSON conversion precise and consistent — high priority

**Entry points:** `to_json`, `into_json`, `to_bson`, `into_bson`, and conversion
traits in [src/decimal_value.rs](src/decimal_value.rs),
[src/primitive_value.rs](src/primitive_value.rs), and
[src/value.rs](src/value.rs).

**Observed with the indicated optional feature enabled:**

- `serde`: parsing `9007199254740993` as `xsd::INTEGER` and calling `into_json()`
  yields `9007199254740992.0`. Decimal conversion also loses precision via `f64`.
- `serde`: converting parsed `INF` or `NaN` to JSON panics. Converting
  `PrimitiveValue::HexBinary` or `Base64Binary` reaches `todo!()`.
- `bson`: for `DecimalValue::Integer(9007199254740993_i128.into())`,
  `.into_bson()` preserves an `Int64`, while `bson::Bson::from(value)` produces
  a rounded `Double`.
- `bson`: converting `DecimalValue::Integer(i128::MAX.into())` with
  `.into_bson()` panics when Decimal128 cannot represent it exactly.
- `to_json()` and `to_bson()` wrap the conversion in `Some(...)`; their `Option`
  signatures do not actually handle these failures.

**Subtasks and acceptance criteria:**

- [ ] Define an exact-number encoding policy and fallible conversion APIs for
  unsupported target values. Test large integers, high-precision decimals,
  non-finite floats, and target range limits. Document any explicitly lossy API.
- [ ] Make BSON method and trait conversion routes agree for each value and
  handle Decimal128 representation failures without `unwrap()` panics.
- [ ] Complete binary JSON conversion using the existing lexical encodings.
  Align partial-calendar and QName JSON/BSON string output with the
  `Display` implementation; those methods still contain independent, incomplete
  formatting. Preserve the existing BSON binary representation.
- [ ] Document the distinction between derived Serde serialization and explicit
  JSON/BSON conversion. Test the promised round-trip guarantees for each API,
  including whether datatype identity and exact numeric values survive.

## 6. Normalize datatype identity and define comparison semantics

**Entry points:** [src/type.rs](src/type.rs),
[src/primitive_type.rs](src/primitive_type.rs), [src/value.rs](src/value.rs),
[src/primitive_value.rs](src/primitive_value.rs), and
[src/decimal_value.rs](src/decimal_value.rs).

**Observed behavior:**

- `Type::from(PrimitiveType::Decimal) != xsd::DECIMAL`: two representations of
  the same datatype compare unequal. Values have corresponding duplicate paths.
- `Value::from(42_i32)` produces a primitive decimal, while
  `xsd::parse("42", xsd::INT)` produces a derived `Int` value.
- `DecimalValue::Byte(1) > DecimalValue::Short(100)` is `true`: derived ordering
  follows enum variant order, not numeric order.
- `DecimalValue::Byte(1) == DecimalValue::Short(1)` is `false`. Parsed NaN values
  compare equal to themselves under the current total-equality representation.

**Subtasks and acceptance criteria:**

- [ ] Establish one canonical datatype/value representation for decimal and
  consistent Rust-number construction rules. Account for existing serialized
  enums and downstream matching code before changing variants.
- [ ] Specify structural equality/order separately from XSD semantic comparison.
  Provide explicit value-comparison operations where appropriate, covering numeric
  subtypes, NaN, signed zero, and partially ordered temporal values. Keep Rust
  `Eq`/`Ord`/`Hash` contracts consistent.
- [ ] Add RDF integration regressions: semantic equality must not erase datatype
  or lexical identity. For example, distinct RDF lexical forms such as `"1"` and
  `"01"` can denote the same integer without becoming the same RDF term.

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
- [ ] Binary and partial-calendar parsing/validation, building on the existing
  formatting and the representations from #3.
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
  implementation, such as optional `oxsdatatypes`. Use the chosen W3C semantics
  as the authority when implementations disagree.
- [ ] Exercise runtime behavior under reduced feature sets, not just compilation.
  In particular, disabled capabilities must produce the documented errors rather
  than panics. Keep bare-metal consumer checks for genuine `no_std` coverage.
- [ ] Document public APIs' formatting, comparison, precision, errors, panics,
  feature requirements, and datatype-identifier lookup conventions. Explain the
  difference between original lexical form, value representation, and canonical
  lexical form. Use runnable examples in rustdoc.
- [ ] Reconcile interoperability claims with implemented APIs. Borsh derives for
  `Value` and `PrimitiveValue` are commented out; `sophia` and `json-ld` are empty
  features in this crate; `rudof` enables a dependency but exposes no dedicated
  conversion API here. Implement or document each boundary in separate changes.

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

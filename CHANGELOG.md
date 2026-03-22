# Changelog

### v0.6.0

- **Breaking:** Replaced `impl From<IntType>` with `impl TryFrom<IntType>` for enums with discriminants.
  `From` violated Rust's infallibility contract by panicking on invalid values. Users must now call
  `MyEnum::try_from(val)` instead of `MyEnum::from(val)` and handle the `Result<Self, ()>`.
- Replaced internal `DeterministicHasher`/`HashMap` with a simple `Vec` for variant tracking, removing
  ~150 lines of custom hasher code that didn't actually guarantee deterministic `HashMap` iteration order.
- Unified `split_pascal_case`, `to_snake_case`, and `to_kebab_case` into a shared `convert_case()` helper,
  eliminating near-identical implementations.
- Extracted `variant_match_pattern()` helper to eliminate 6+ copy-pasted if/else blocks for building
  match patterns across unit, tuple, and struct variants.
- Removed the 16-element `into_parts()` tuple; `ParsedVariants` struct fields are now used directly.
- Deduplicated unit vs payload enum branches by extracting common methods (`count`, `ordinal`,
  `pascal_spaced`, `snake_case`, `kebab_case`, `variant_name`, `is_first`, `is_last`, `comes_before`,
  `comes_after`) into a single unconditional block.
- Replaced string-based `check_derive_traits` with proper `syn` path parsing via
  `Punctuated::<Path, Token![,]>::parse_terminated`.
- Extracted shared `resolve_int_type()` helper in `core.rs`, used by both `proc.rs` and `attr.rs`.
- Simplified redundant clone, loop, and branch patterns throughout `core.rs`.
- Added unit tests for `to_snake_case`, `to_kebab_case` edge cases (empty strings, single chars,
  all-caps, mixed alphanumeric).
- Removed duplicate 108-line doc comment from `proc.rs` (already included via `include_str!`).
- Updated README.md, ATTR.md, and PROCS.md: added `TryFrom` to utility functions reference, fixed
  stale cross-references, removed duplicated content (ATTR.md/PROCS.md now link to README for full
  API reference), replaced raw HTML with markdown formatting, and added `from_ordinal` Clone prerequisite note.
- Expanded examples: added `TryFrom`, discriminant expressions, Copy/const, and auto-derive demos
  to `basic_disc.rs`; added `variant_name()` demo and missing test to `basic_no_disc.rs`; added
  test module to `complex.rs`; removed unused `rand::RngExt` imports.

### v0.5.2

- Added and expanded `trybuild` UI compile-fail coverage to lock in macro diagnostics and edge-case validation
  (invalid `IntType`, duplicate `enum_def`, `#[enum_extend]` on non-enums, and complex payload variants missing
  explicit discriminants).
- Improved macro maintainability by refactoring parts of expansion plumbing in `core` (including cleaner parsed-variant
  destructuring flow).
- Updated docs and project policy details, including `rand = "0.10"` examples and explicit Rust edition/MSRV guidance.
- Expanded `examples/` coverage to demonstrate nearly all generated feature groups, including navigation, filtering,
  batch helpers, metadata APIs, and random-feature example compatibility updates.

### v0.5.1

- Improved internal deterministic hasher.
- bumped dependencies.

### v0.5.0

- Added support for complex enums (variants with payloads)
    - <b>note</b>: not all utility features are possible for complex enums and are
      omitted from these types of enums only (non-complex enums still have them). see the
      [Complex enum support](./README.md#complex-enum-support) section for more details.
  ```rust
  use enum_ext::enum_extend;

  #[enum_extend(IntType = "i32")]
  #[derive(Debug, Clone, PartialEq)]
  pub enum DiscrExpression {
      // singles
      X10(u32) = 10,
      // tuples
      X25((i32,i16)) = 5 * 5,
      // structs
      Y26{foo: u32,bar: String} = 13 + 13,
  }
  ```

### v0.4.5

- Added support for discriminant expressions, instead of just literals.
  ```rust
  use enum_ext::enum_extend;

  #[enum_extend(IntType = "i32")]
  #[derive(Debug, Clone, Copy, PartialEq)]
  pub enum DiscrExpression {
      // literals
      X10 = 10,
      // expressions
      X25 = 5 * 5,
      Y26 = 13 + 13,
      Z100 = 10 * (5 + 5),
  }
  ```

### v0.4.4

- swapped `std::ops::Range` with `core::ops::Range` for compatibility with `no_std`

### v0.4.3

- as_<int_type> is now `const` fn if dev derives `Copy` on enum.

### v0.4.2

- Parse the configured `IntType` into a real Rust type using `syn::parse_str::<syn::Type>` instead of string-based token
  hacks; this makes `IntType` handling more robust and prevents malformed type tokens from being emitted.
- Emit `#[repr(...)]` whenever the user explicitly specifies `IntType` (even if no discriminants are present), so the
  enum layout matches the requested integer representation.
- Reject multiple `#[enum_def(...)]` attributes on the derive macro; the macro now returns a clear compile error if more
  than one `enum_def` attribute is present.
- Use the local `EnumDefArgs::default()` directly and tidy up attribute parsing code paths for clarity.
- Improve tests and validation across the macros;
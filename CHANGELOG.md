# Changelog

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
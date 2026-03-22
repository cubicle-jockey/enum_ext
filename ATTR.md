# `#[enum_extend]` — Attribute Macro

[![Crate](https://img.shields.io/crates/v/enum_ext.svg)](https://crates.io/crates/enum_ext)
[![API](https://docs.rs/enum_ext/badge.svg)](https://docs.rs/enum_ext)

The `#[enum_extend]` attribute macro enhances Rust enums with additional methods and conversions.
See the [README](./README.md) for the full list of generated utility functions and features.

For the procedural macro variant, see [`enum_ext!`](./PROCS.md).

## Attributes

Attributes are optional and used to customize the generated methods.

* `IntType` specifies the discriminant type for conversion methods. Supported types include standard Rust
  integer types like `i32`, `u32`, `i64`, etc. If not specified, `usize` is used as the default.
    * **Note**: If the enum has discriminant values, `#[derive(Clone)]` is added to the enum (if not already present).

When using `#[enum_extend]`, the attribute is applied directly in the tag:

```rust
use enum_ext::enum_extend;

// example with no attribute
#[enum_extend]
#[derive(Debug, Clone, PartialEq)]
pub enum Discr1 {
    A = 10,
    B = 20,
    C = 30,
}

// example with an attribute
#[enum_extend(IntType = "i32")]  // <- `IntType` is the discriminant type for conversion methods
#[derive(Debug, Clone, PartialEq)]
pub enum Discr2 {
    A = 10,
    B = 20,
    C = 30,
}

```

## Usage

### Basic Example

To use the `#[enum_extend]` attribute macro, simply include it in your Rust project and apply it to your enum
definitions. Here's an example:

```rust
fn main() {
    use enum_ext::enum_extend;

    #[enum_extend(IntType = "i32")]
    #[derive(Debug, Default, Clone, PartialEq)]
    pub enum AdvancedEnum {
        #[default]
        A = 10,
        B = 20,
        C = 30,
    }

    for x in AdvancedEnum::iter() {
        let i = x.as_i32();
        let v = AdvancedEnum::from_i32(i).unwrap();
        assert_eq!(i, v.as_i32());
        assert_eq!(*x, v); // This comparison requires that PartialEq be derived
    }

    let v = AdvancedEnum::from_i32(20).unwrap();
    assert_eq!(v, AdvancedEnum::B);
}
```

Or any int type you prefer: `i8` to `u128`.

```rust
fn main() {
    use enum_ext::enum_extend;

    #[enum_extend(IntType = "i8")]
    #[derive(Debug, Default, Clone, PartialEq)]
    pub enum AdvancedEnum {
        #[default]
        A = -10,
        B = -20,
        C = 30,
    }

    for x in AdvancedEnum::iter() {
        let i = x.as_i8();
        let v = AdvancedEnum::from_i8(i).unwrap();
        assert_eq!(i, v.as_i8());
        assert_eq!(*x, v); // This comparison requires that PartialEq be derived
    }

    let v = AdvancedEnum::from_i8(-20).unwrap();
    assert_eq!(v, AdvancedEnum::B);
}
```

### Additional Utility Methods

```rust
use enum_ext::enum_extend;

#[enum_extend(IntType = "i32")]
#[derive(Debug, PartialEq)]
pub enum DevelopmentStatus {
    InDev = 10,
    InQA = 20,
    CodeReview = 30,
    FinalQA = 40,
    FinalCodeReview = 50,
    Accepted = 60,
    Closed = 70,
}

fn main() {
    // Using list()
    let variants = DevelopmentStatus::list();
    assert_eq!(variants,
               [DevelopmentStatus::InDev,
                   DevelopmentStatus::InQA,
                   DevelopmentStatus::CodeReview,
                   DevelopmentStatus::FinalQA,
                   DevelopmentStatus::FinalCodeReview,
                   DevelopmentStatus::Accepted,
                   DevelopmentStatus::Closed]);

    // Using count()
    let count = DevelopmentStatus::count();
    assert_eq!(count, 7);

    // Using ordinal()
    let ordinal = DevelopmentStatus::CodeReview.ordinal();
    assert_eq!(ordinal, 2);  // CodeReview is the third variant, so its ordinal is 2
    assert_eq!(DevelopmentStatus::from_ordinal(2), Some(DevelopmentStatus::CodeReview));

    // Using iter()
    for (i, variant) in DevelopmentStatus::iter().enumerate() {
        assert_eq!(i, variant.ordinal());
    }

    // Using from_i32() and as_i32()
    let variant = DevelopmentStatus::from_i32(20).unwrap();
    assert_eq!(variant, DevelopmentStatus::InQA);
    assert_eq!(variant.as_i32(), 20);

    // Using pascal_spaced() method that returns the variant name in spaced PascalCase.
    // This is useful for displaying enum variants in a user-friendly format (e.g., in a UI).
    // One example usage is converting InQA to "In QA" for display on a web page.
    let status = DevelopmentStatus::InQA;
    assert_eq!(status.pascal_spaced(), "In QA");

    // Using from_pascal_spaced() method that returns the variant from the spaced PascalCase name.
    // This is useful for converting user-friendly format back to an enum variant.
    // This is the reverse of the example above, converting "In QA" back to an enum.
    let status2 = DevelopmentStatus::from_pascal_spaced("In QA").unwrap();
    assert_eq!(status2, DevelopmentStatus::InQA);
}
```

### Complex enum support

As of v0.5.0, enums with payloads (tuple and struct variants) are supported.

Requirements:

- Each payload-carrying variant must have an explicit discriminant expression (e.g., `A(u32) = 4`). A
  compile-time error is emitted if any complex variant lacks a discriminant.
- `#[repr(..)]` is emitted automatically when IntType is specified or when discriminants are present. If IntType is not
  specified, the default conversion target is usize and `as_usize()` will be generated.

Generated API for complex enums:

- Available (const and using match on self):
    - count(), ordinal(), valid_ordinal()
    - pascal_spaced(), snake_case(), kebab_case()
    - variant_name(), is_first(), is_last(), comes_before(), comes_after()
    - `as_<IntType>(&self) -> <IntType>` (for example, `as_u32()`)
- Omitted for complex enums:
    - list(), iter(), slice(), range(), first_n(), last_n()
    - from_ordinal(), ref_from_ordinal(), next(), previous(), next_linear(), previous_linear()
    - `from_<IntType>(...)`, `impl TryFrom<<IntType>>`
    - from_pascal_spaced(...), from_snake_case(...), from_kebab_case(...), variant_names()
    - random() helpers (feature = "random")

Example:

```rust
use enum_ext::enum_extend;

#[enum_extend(IntType = "u32")]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Complex {
    AlphaOne(u32) = 4,
    BetaTwo((u32, i16)) = 8,
    CharlieThree { fred: u32, barny: i16 } = 16,
}

fn main() {
    let a = Complex::AlphaOne(10);
    assert_eq!(a.as_u32(), 4);
    assert_eq!(a.pascal_spaced(), "Alpha One");
}
```

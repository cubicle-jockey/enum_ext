# Enum Extension Library

[![Rust](https://github.com/cubicle-jockey/enum_ext/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/enum_ext/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/enum_ext/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/enum_ext/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/enum_ext.svg)](https://crates.io/crates/enum_ext)
[![API](https://docs.rs/enum_ext/badge.svg)](https://docs.rs/enum_ext)

This Rust crate provides a procedural macro `enum_ext!` that enhances Rust enums with additional methods and
conversions. It simplifies working with enums by automatically generating utility methods for common tasks such as
retrieving a list of variants, counting variants, and converting between discriminates and integer types.

## Utility Functions

- `list()`: Returns an array of all variants in the enum.
- `count()`: Returns the count of variants in the enum.
- `ordinal()`: Returns the ordinal of a variant.
- `iter()`: Returns an iterator over the variants in the enum.
- Conversion methods `from_<IntType>(val)` and `to_<IntType>(&self)`, if specified in the attributes.
- `pascal_spaced(&self)`: Returns the variant name in spaced PascalCase. InQA becomes "In QA".
- `from_pascal_spaced()`: Returns the variant from the spaced PascalCase name. "In QA" becomes InQA.
- `from_ordinal()`: Returns the variant from the ordinal.
- `ref_from_ordinal()`: Returns a reference to the variant from the ordinal.
- more to come...

## Attributes

`#[enum_def(IntType = "i32")]`: Specifies the integer type for conversion methods. The generated methods allow
conversion
from the specified integer type to an enum variant and vice versa. Supported types include standard Rust integer types
like `i32`, `u32`, `i64`, etc.<br>
<b>Note</b>: If the integer type is not specified in the `enum_def` attribute, `usize` is used as the default.<br>
<b>Note</b>: If the enum
has discriminant values, `#[derive(Clone)]` is added to the enum (if not already present).

## Usage

To use the `enum_ext!` macro, simply include it in your Rust project and apply it to your enum definitions. Here's an
example:

```rust
fn main() {
    use enum_ext::enum_ext;

    enum_ext!(
        #[derive(Debug, Clone)]
        pub enum SimpleEnum {
            A,
            B,
            C,
        }
    );
    // With this, you can now use the generated methods on SimpleEnum:
    let x = SimpleEnum::B;
    assert_eq!(x.ordinal(), 1); // B is the second variant, so its ordinal is 1

    let mut count = 0;
    for x in SimpleEnum::iter() {
        let i = x.ordinal();
        assert_eq!(i, count);
        count += 1;
    }
}
```

### Using the `enum_def` Attribute

You can specify the integer type for conversion methods using the enum_def attribute. The generated methods allow
conversion from the specified integer type to an enum variant and vice versa. Supported types include standard Rust
integer types like `i32`, `u32`, `i64`, etc.

```rust
fn main() {
    use enum_ext::enum_ext;

    enum_ext!(
        #[enum_def(IntType = "i32")]  // <- Specify the discriminant type
        #[derive(Debug, Default, Clone, PartialEq)]
        pub enum AdvancedEnum {
            #[default]
            A = 1, // <- do not specify the discriminant type here
            B = 2,
            C = 3,
        }
    );

    for x in AdvancedEnum::iter() {
        let i = x.as_i32();
        let v = AdvancedEnum::from_i32(i).unwrap();
        assert_eq!(i, v.as_i32());
        assert_eq!(*x, v); // This comparison requires that PartialEq be derived
    }
}
```

## Installation

Add the following to your Cargo.toml file:

```toml
[dependencies]
enum_ext = "0.1.2"
```
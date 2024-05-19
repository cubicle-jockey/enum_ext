# Enum Extension Library

[![Rust](https://github.com/cubicle-jockey/enum_ext/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/enum_ext/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/enum_ext/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/enum_ext/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/enum_ext.svg)](https://crates.io/crates/enum_ext)
[![API](https://docs.rs/enum_ext/badge.svg)](https://docs.rs/enum_ext)

This Rust crate provides `procedural` and `attribute` macros that enhance Rust enums with additional methods and
conversions. It simplifies working with enums by automatically generating utility methods for common tasks such as
retrieving a list of variants, counting variants, and converting between discriminants and integer types.

See the `enum_ext!` and `#[enum_extend]` macro examples below for more information.

Both macros generate the same utility methods so you can choose the one that best fits your coding style.

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

Attributes are optional and used to customize the generated methods.

* `IntType` is currently the only attribute supported and specifies the discriminant type for conversion methods. The
  generated methods allow
  conversion from this type to an enum variant and vice versa. Supported types include standard Rust
  integer types like `i32`, `u32`, `i64`, etc. If this attribute is not specified, `usize` is used as the default.
    * **Note**: If the enum has discriminant values, `#[derive(Clone)]` is added to the enum (if not already present).

When using `enum_ext!`, the attribute is applied in an `enum_def` parameter to the macro:

```rust
use enum_ext::enum_ext;

enum_ext!(
    #[enum_def(IntType = "i32")]  // <- `IntType` is the discriminant type. 
    #[derive(Debug, Clone, PartialEq)]
    pub enum AdvancedEnum {
        A = 10,  // <- do not specify a discriminant type here (10i32 etc)
        B = 20,
        C = 30,
    }
);
```

## Usage

### Using the `enum_ext!` Procedural Macro

To use the `enum_ext!` macro, simply include it in your Rust project and apply it to your enum definitions. Here's an
example:

```rust
fn main() {
    use enum_ext::enum_ext;

    enum_ext!(
        #[derive(Debug, Clone, PartialEq)]
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

    // enum_ext gives enums an iterator and variants can be iterated over
    for x in SimpleEnum::iter() {
        // The ordinal of the variant can be retrieved
        let i = x.ordinal();
        assert_eq!(i, count);
        count += 1;
    }

    // enums also get a list method that returns an array of all variants
    let list = SimpleEnum::list();
    assert_eq!(list, [SimpleEnum::A, SimpleEnum::B, SimpleEnum::C]);

    enum_ext!(
        #[derive(Debug, Clone, Default, PartialEq)]
        pub enum TicketStatus {
            #[default]
            Open,
            InDev,
            Completed,
            InQA,
            CodeReview,
            FinalQA,
            FinalCodeReview,
            Accepted,
            Closed,
        }
    );

    // enums now have a pascal_spaced method that returns the variant name in spaced PascalCase.
    // This is useful for displaying enum variants in a user-friendly format (e.g., in a UI).
    // One example usage is converting InQA to "In QA" for display on a web page.
    let status = TicketStatus::InQA;
    assert_eq!(status.pascal_spaced(), "In QA");

    // enums also get a from_pascal_spaced method that returns the variant from the spaced PascalCase name.
    // This is useful for converting user-friendly format back to an enum variant.
    // This is the reverse of the example above, converting "In QA" back to an enum.
    let status2 = TicketStatus::from_pascal_spaced("In QA").unwrap();
    assert_eq!(status2, TicketStatus::InQA);
}
```

Additional utility methods are generated for the enum variants:

```rust

use enum_ext::enum_ext;

enum_ext!(
    #[derive(Debug, Clone, PartialEq)]
    #[enum_def(IntType = "i32")]
    pub enum DevelopmentStatus {
        InDev = 1,
        InQA = 2,
        CodeReview = 3,
        FinalQA = 4,
        FinalCodeReview = 5,
        Accepted = 6,
        Closed = 7,
    }
);

fn main() {
    // Using list()
    let variants = DevelopmentStatus::list();
    assert_eq!(variants, [DevelopmentStatus::InDev,
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

    // Using iter()
    for (i, variant) in DevelopmentStatus::iter().enumerate() {
        assert_eq!(i, variant.ordinal());
    }

    // Using from_i32() and as_i32()
    let variant = DevelopmentStatus::from_i32(2).unwrap();
    assert_eq!(variant, DevelopmentStatus::InQA);
    assert_eq!(variant.as_i32(), 2);

    // Using pascal_spaced()
    let status = DevelopmentStatus::InQA;
    assert_eq!(status.pascal_spaced(), "In QA");

    // Using from_pascal_spaced()
    let status2 = DevelopmentStatus::from_pascal_spaced("In QA").unwrap();
    assert_eq!(status2, DevelopmentStatus::InQA);
}
```
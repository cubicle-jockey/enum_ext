# Enum Extension Library

[![Rust](https://github.com/cubicle-jockey/enum_ext/actions/workflows/rust.yml/badge.svg)](https://github.com/cubicle-jockey/enum_ext/actions/workflows/rust.yml)
[![Dependency Review](https://github.com/cubicle-jockey/enum_ext/actions/workflows/dependency-review.yml/badge.svg)](https://github.com/cubicle-jockey/enum_ext/actions/workflows/dependency-review.yml)
[![Crate](https://img.shields.io/crates/v/enum_ext.svg)](https://crates.io/crates/enum_ext)
[![API](https://docs.rs/enum_ext/badge.svg)](https://docs.rs/enum_ext)

This Rust crate provides `procedural` and `attribute` macros that enhance Rust enums with additional methods and
conversions. It simplifies working with enums by automatically generating utility methods for common tasks such as
retrieving a list of variants, counting variants, and converting between discriminants and integer types.

See the `enum_ext!` and `#[enum_extend]` macro examples below for more information.

Both macros generate the same utility methods, so you can choose the one that best fits your coding style.

## Utility Functions

- **`list()`**: Returns an array containing all variants of the enum.
- **`count()`**: Returns the number of variants in the enum.
- **`ordinal()`**: Returns the ordinal (index) of a variant.
- **`from_ordinal(ordinal: usize)`**: Returns the variant corresponding to the given ordinal.
- **`ref_from_ordinal(ordinal: usize)`**: Returns a reference to the variant corresponding to the given ordinal.
- **`valid_ordinal(ordinal: usize)`**: Checks if the given ordinal is valid for the enum.
- **`iter()`**: Returns an iterator over all the variants of the enum.
- **`from_<IntType>(value: <IntType>)`** and **`as_<IntType>(&self)`**: Convert to and from the specified integer type,
  if defined in the attributes.
    - For example, `from_i32(10)` and `as_i32()` if `IntType = "i32"`, or `from_u32(10)`
      and `as_u32()` if `IntType = "u32"`, etc.
- **`pascal_spaced(&self)`**: Converts the variant name to spaced PascalCase. For instance, `InQA` becomes `"In QA"`.
- **`from_pascal_spaced(name: &str)`**: Returns the variant corresponding to the spaced PascalCase name. For
  example, `"In QA"` becomes `InQA`.
- **`pretty_print()`**: Returns a formatted string displaying the enum and all its variants in a pretty-print format.
- **More to come...**: Stay tuned for additional utility functions and features.

### `See examples in the repository for more information.`

## Attributes

Attributes are optional and used to customize the generated methods.

* `IntType` is currently the only attribute supported and specifies the discriminant type for conversion methods. The
  generated methods allow
  conversion from this type to an enum variant and vice versa. Supported types include standard Rust
  integer types like `i32`, `u32`, `i64`, etc. If this attribute is not specified, `usize` is used as the default.
    * **Note**: If the enum has discriminant values, `#[derive(Clone)]` is added to the enum (if not already present).

Assigning attributes vary slightly depending on the macro used.

When using `enum_extend`, the attribute is applied directly in the tag:

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

When using `enum_ext!`, the attribute is applied in an `enum_def` parameter to the macro:

```rust
use enum_ext::enum_ext;

enum_ext!(
    #[enum_def(IntType = "i32")]  // <- `IntType` is the discriminant type. 
    #[derive(Debug, Clone, PartialEq)]
    pub enum AdvancedEnum {
        A = 10,  
        B = 20,
        C = 30,
    }
);
```

## Usage

### Using the `#[enum_extend]` Attribute Macro

To use the enum_extend attribute macro, simply include it in your Rust project and apply it to your enum definitions.
Here's an example:

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

## Getting Started

Add the following to your Cargo.toml file:

```toml
[dependencies]
enum_ext = "0.2.2"
```

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

### Core Functions

- **`list()`**: Returns an array containing all variants of the enum.
- **`count()`**: Returns the number of variants in the enum.
- **`ordinal()`**: Returns the ordinal (index) of a variant.
- **`from_ordinal(ordinal: usize)`**: Returns the variant corresponding to the given ordinal.
- **`ref_from_ordinal(ordinal: usize)`**: Returns a reference to the variant corresponding to the given ordinal.
- **`valid_ordinal(ordinal: usize)`**: Checks if the given ordinal is valid for the enum.
- **`iter()`**: Returns an iterator over all the variants of the enum.
- **`pretty_print()`**: Returns a formatted string displaying the enum and all its variants in a pretty-print format.

### String Conversion Functions

- **`pascal_spaced(&self)`**: Converts the variant name to spaced PascalCase. For instance, `InQA` becomes `"In QA"`.
- **`from_pascal_spaced(name: &str)`**: Returns the variant corresponding to the spaced PascalCase name. For example,
  `"In QA"` becomes `InQA`.
- **`snake_case(&self)`**: Converts the variant name to snake_case. For instance, `InQA` becomes `"in_qa"`.
- **`from_snake_case(name: &str)`**: Returns the variant corresponding to the snake_case name. For example, `"in_qa"`
  becomes `InQA`.
- **`kebab_case(&self)`**: Converts the variant name to kebab-case. For instance, `InQA` becomes `"in-qa"`.
- **`from_kebab_case(name: &str)`**: Returns the variant corresponding to the kebab-case name. For example, `"in-qa"`
  becomes `InQA`.

### Navigation Functions

- **`next(&self)`**: Returns the next variant in ordinal order (wraps around to first when at last).
- **`previous(&self)`**: Returns the previous variant in ordinal order (wraps around to last when at first).
- **`next_linear(&self)`**: Returns the next variant without wrapping (returns `None` at end).
- **`previous_linear(&self)`**: Returns the previous variant without wrapping (returns `None` at start).

### Validation Functions

- **`is_first(&self)`**: Returns `true` if this is the first variant (ordinal 0).
- **`is_last(&self)`**: Returns `true` if this is the last variant.
- **`comes_before(&self, other: &Self)`**: Returns `true` if this variant comes before the other in ordinal order.
- **`comes_after(&self, other: &Self)`**: Returns `true` if this variant comes after the other in ordinal order.

### Filtering Functions

- **`variants_containing(substring: &str)`**: Returns variants whose names contain the substring.
- **`variants_starting_with(prefix: &str)`**: Returns variants whose names start with the prefix.
- **`variants_ending_with(suffix: &str)`**: Returns variants whose names end with the suffix.

### Batch Operations

- **`slice(start: usize, end: usize)`**: Returns a slice of variants from start to end ordinal.
- **`range(range: std::ops::Range<usize>)`**: Returns variants in the specified ordinal range.
- **`first_n(n: usize)`**: Returns the first N variants.
- **`last_n(n: usize)`**: Returns the last N variants.

### Metadata Functions

- **`variant_name(&self)`**: Returns the variant name as a string.
- **`variant_names()`**: Returns all variant names as a vector of strings.

### Random Selection (Optional Feature)

- **`random()`**: Returns a random variant (requires `"random"` feature).
- **`random_with_rng<R: Rng>(rng: &mut R)`**: Returns a random variant using provided RNG (requires `"random"` feature).

### Integer Conversion Functions (When IntType is specified)

- **`from_<IntType>(value: <IntType>)`** and **`as_<IntType>(&self)`**: Convert to and from the specified integer type,
  if defined in the attributes.
    - For example, `from_i32(10)` and `as_i32()` if `IntType = "i32"`, or `from_u32(10)` and `as_u32()` if
      `IntType = "u32"`, etc.

### `See examples in the repository for more information.`

## Attributes

Attributes are optional and used to customize the generated methods.

* `IntType` is currently the only attribute supported and specifies the discriminant type for conversion methods. The
  generated methods allow
  conversion from this type to an enum variant and vice versa. Supported types include standard Rust
  integer types like `i32`, `u32`, `i64`, etc. If this attribute is not specified, `usize` is used as the default.
    * **Note**: If the enum has discriminant values, `#[derive(Clone)]` is added to the enum (if not already present).

## Features

This crate supports optional features that can be enabled in your `Cargo.toml`:

* `random` - Enables random variant selection functionality (`random()` and `random_with_rng()` methods). Add this to
  your `Cargo.toml`:
  ```toml
  [dependencies]
  rand = "0.9.1"
  enum_ext = { version = "0.4.1", features = ["random"] }
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

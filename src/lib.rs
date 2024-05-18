#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDocTests;
mod proc;
//mod derive;  future...

/// `enum_ext` is a procedural macro that enhances Rust enums with additional methods and conversions.
///
/// This macro simplifies working with enums by automatically generating utility methods
/// for common tasks such as retrieving a list of variants, counting variants, and converting
/// between variants and integer types.
///
/// ## Enhancements
/// - `list()`: Returns an array of all variants in the enum.
/// - `count()`: Returns the count of variants in the enum.
/// - `ordinal()`: Returns the ordinal of a variant.
/// - `iter()`: Returns an iterator over the variants in the enum.
/// - Conversion methods `from_<IntType>(val)` and `to_<IntType>(&self)`, if specified in the attributes.
/// - `pascal_spaced()`: Returns the variant name in spaced PascalCase. InQA becomes "In QA".
/// - `from_pascal_spaced()`: Returns the variant from the spaced PascalCase name. "In QA" becomes InQA.
/// - `from_ordinal()`: Returns the variant from the ordinal.
/// - `ref_from_ordinal()`: Returns a reference to the variant from the ordinal.
///
/// ## Attributes
/// - `#[enum_def(IntType = "i32")]`: Specifies the integer type for conversion methods.
///   The generated methods allow conversion from the specified integer type to an enum variant
///   and vice versa. Supported types include standard Rust integer types like `i32`, `u32`, `i64`, etc.
///
/// - **Note:** If the integer type is not specified in the `enum_def` attribute, usize is used as the default.
/// - **Note:** If the enum has discriminant values, `#[derive(Clone)]` is added to the enum (if not already present).
///
/// ## Usage
/// - `input`: A `TokenStream` representing an enum definition.
///
/// ## Errors
/// Returns a compilation error if the input is not a valid enum or if there are issues with parsing.
///
/// ## Examples
/// Basic usage:
/// ```
/// # use enum_ext::enum_ext;
/// enum_ext!(
///     #[derive(Debug, Clone, PartialEq)]
///     pub enum SimpleEnum {
///         A,
///         B,
///         C,
///     }
/// );
/// // With this, you can now use the generated methods on SimpleEnum:
/// let x = SimpleEnum::B;
/// assert_eq!(x.ordinal(), 1); // B is the second variant, so its ordinal is 1
///
/// let mut count = 0;
///
/// // enum_ext gives enums an iterator and variants can be iterated over
/// for x in SimpleEnum::iter() {
///     // ordinal() returns the ordinal of the variant
///     let i = x.ordinal();
///     assert_eq!(i, count);
///     count += 1;
/// }
///
/// // enums also get a list method that returns an array of all variants
/// let list = SimpleEnum::list();
/// assert_eq!(list, [SimpleEnum::A, SimpleEnum::B, SimpleEnum::C]);
///
/// enum_ext!(
///     #[derive(Debug, Clone, Default, PartialEq)]
///     pub enum TicketStatus {
///         #[default]
///         Open,
///         InDev,
///         Completed,
///         InQA,
///         CodeReview,
///         FinalQA,
///         FinalCodeReview,
///         Accepted,
///         Closed,
///     }
/// );
///
/// // enums now have a pascal_spaced method that returns the variant name in spaced PascalCase.
/// // This is useful for displaying enum variants in a user-friendly format (e.g., in a UI).
/// // One example usage is converting InQA to "In QA" for display on a web page.
/// let status = TicketStatus::InQA;
/// assert_eq!(status.pascal_spaced(), "In QA");
///
/// // enums also get a from_pascal_spaced method that returns the variant from the spaced PascalCase name.
/// // This is useful for converting user-friendly format back to an enum variant.
/// // This is the reverse of the example above, converting "In QA" back to an enum.
/// let status2 = TicketStatus::from_pascal_spaced("In QA").unwrap();
/// assert_eq!(status2, TicketStatus::InQA);
/// ```
///
/// Advanced usage with `enum_def` attribute:
/// ```
/// use enum_ext::enum_ext;
/// enum_ext!(
///     #[enum_def(IntType = "i32")]  // <- Specify the discriminant type
///     #[derive(Debug, Default, Clone, PartialEq)]
///     pub enum AdvancedEnum {
///         #[default]
///         A = 10, // <- do not specify the discriminant type here
///         B = 20,
///         C = 30,
///     }
/// );
///
/// for x in AdvancedEnum::iter() {
///     let i = x.as_i32();
///     let v = AdvancedEnum::from_i32(i).unwrap();
///     assert_eq!(i, v.as_i32());
///     assert_eq!(*x, v); // <-- This comparison requires that PartialEq be derived
/// }
///
/// let v = AdvancedEnum::from_i32(20).unwrap();
/// assert_eq!(v, AdvancedEnum::B);
///
/// ```
///
/// ## Failures
/// Example of a failure case (complex variants are not yet supported):
/// ```text
/// # use crate::enum_ext::enum_ext;
/// enum_ext!(
///     #[derive(Debug, Clone)]
///     pub enum FailureEnum {
///         A(usize),
///         B(String),
///         C,
///     }
/// );
/// // error: Variant error: Unsupported variant 'A(usize)': complex variants are not yet supported by enum_ext
/// ```

#[proc_macro]
pub fn enum_ext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc::enum_ext(input)
}

/* FUTURE...
#[proc_macro_derive(EnumExt, attributes(enum_def))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::enum_derive(input)
}

 */

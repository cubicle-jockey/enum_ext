use super::core::{
    generate_expanded_enum, valid_int_type, EnumDefArgs, EnumMacroError,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, Attribute, DeriveInput};

/// Processes the attributes of an enum variant.
///
/// This function takes a reference to a slice of attributes and returns a tuple.
/// The first element of the tuple is an instance of `EnumDefArgs` which contains the parsed arguments of the `enum_def` attribute.
/// The second element of the tuple is a vector of attributes that are not related to `enum_def`.
///
/// # Arguments
///
/// * `attrs` - A slice of attributes to be processed.
///
/// # Returns
///
/// A tuple where the first element is an instance of `EnumDefArgs` containing the parsed arguments of the `enum_def` attribute,
/// and the second element is a vector of attributes that are not related to `enum_def`.
///
/// # Errors
///
/// This function returns an error if there is an error in parsing the attributes.
///
/// # Examples
///
/// ```text
/// let (my_args, not_mine) =  match process_attributes(&input.attrs) {
///    Ok(result) => result,
///    Err(error) => {
///      let error_message = format!("{}", error);
///      return TokenStream::from(quote! { compile_error!(#error_message); });
///    }
/// };
///
/// ```
fn process_attributes(
    attrs: &[Attribute],
) -> Result<(EnumDefArgs, Vec<Attribute>), EnumMacroError> {
    // Logic to process attributes
    let mut not_mine = Vec::<Attribute>::new();
    let mut my_args = None;
    for attr in attrs {
        if attr.path().is_ident("enum_def") {
            let args: EnumDefArgs = attr
                .parse_args_with(EnumDefArgs::parse)
                .map_err(|e| EnumMacroError::ParseError(e.to_string()))?;

            my_args = Some(args);
        } else {
            not_mine.push(attr.clone());
        }
    }
    Ok((
        my_args.unwrap_or_else(crate::proc::EnumDefArgs::default),
        not_mine,
    ))
}
/// A procedural macro to enhance enums in Rust with additional methods and conversions.
///
/// `enum_ext` simplifies working with enums by automatically generating utility methods
/// for common tasks such as retrieving a list of variants, counting variants, and converting
/// between variants and integer types.
/// <br><br>
/// I wrote this macro because I was tired of writing the same boilerplate code for enums over and over again. All
/// variants (in my case) are saved to the database or transmitted over the wire as integer types(discriminant values).
/// This macro generates most of the boilerplate code for enums that are used in this way.  It is missing some boilerplate features that
/// I use though, such as serde, but I plan to add those in the near future.
///
/// ## Enhancements
/// - `list()`: Returns an array of all variants in the enum.
/// - `count()`: Returns the count of variants in the enum.
/// - `ordinal()`: Returns the ordinal of a variant.
/// - `iter()`: Returns an iterator over the variants in the enum.
/// - `from_<IntType>(val)` and `as_<IntType>(&self)`, if specified in the attributes.
/// - `pascal_spaced(&self)`: Returns the variant name in spaced PascalCase. InQA becomes "In QA".
/// - `from_pascal_spaced(s: &str)`: Returns the variant from the spaced PascalCase name. "In QA" becomes InQA.
/// - `from_ordinal(ord: usize)`: Returns the variant from the ordinal.
/// - `ref_from_ordinal(ord: usize)`: Returns a reference to the variant from the ordinal.
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
/// // With this, you can now use the generated methods
/// // on SimpleEnum:
/// let x = SimpleEnum::B;
/// assert_eq!(x.ordinal(), 1); // B is the second variant,
///                             // so its ordinal is 1
///
/// let mut count = 0;
///
/// // enum_ext gives enums an iterator and variants can be
/// // iterated over
/// for x in SimpleEnum::iter() {
///     // ordinal() returns the ordinal of the variant
///     let i = x.ordinal();
///     assert_eq!(i, count);
///     count += 1;
/// }
///
/// // enums also get a list method that returns an array
/// // of all variants
/// let list = SimpleEnum::list();
/// assert_eq!(list, [SimpleEnum::A, SimpleEnum::B, SimpleEnum::C]);
///
/// // pascal_spaced() examples
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
/// // enums now have a `pascal_spaced` method that returns the
/// // variant name in spaced PascalCase. This is useful for
/// // displaying enum variants in a user-friendly
/// // format (e.g., in a UI).
/// // One example usage is converting InQA to "In QA" for
/// // display on a web page.
/// let status = TicketStatus::InQA;
/// assert_eq!(status.pascal_spaced(), "In QA");
///
/// // enums also get a `from_pascal_spaced` method that returns
/// // the variant from the spaced PascalCase name. This is
/// // useful for converting user-friendly format back to an
/// // enum variant.
/// // This is the reverse of the example above,
/// // converting "In QA" back to an enum.
/// let status2 = TicketStatus::from_pascal_spaced("In QA").unwrap();
/// assert_eq!(status2, TicketStatus::InQA);
/// ```
#[doc = include_str!("../PROCS.md")]
pub fn enum_ext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Ensure it's an enum
    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => return TokenStream::from(quote! { compile_error!("enum_ext only works on enums"); }),
    };

    // Parse the attributes to extract enum_def arguments
    let (my_args, derives_etc) = match process_attributes(&input.attrs) {
        Ok(result) => result,
        Err(error) => {
            let error_message = format!("{}", error);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }
    };

    // Set up integer type
    let mut int_type = quote! { usize };
    let mut int_type_str = "usize".to_string();

    if let Some(lit_str) = my_args.int_type {
        int_type_str = lit_str.value();
        if !valid_int_type(&int_type_str) {
            let error_message = format!("Invalid IntType: {}", int_type_str);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }

        int_type = match lit_str.parse() {
            Ok(result) => result,
            Err(error) => {
                let error_message = format!("Invalid IntType: {}", error);
                return TokenStream::from(quote! { compile_error!(#error_message); });
            }
        };
    }

    // Generate the expanded enum using the shared function
    match generate_expanded_enum(
        &derives_etc,
        &input.vis,
        &input.ident,
        &variants,
        &int_type_str,
        &int_type,
    ) {
        Ok(expanded) => expanded.into(),
        Err(error) => {
            let error_message = format!("{}", error);
            TokenStream::from(quote! { compile_error!(#error_message); })
        }
    }
}

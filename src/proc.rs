use super::core::{
    append_int_fns, check_derive_traits, parse_variants, valid_int_type, EnumDefArgs,
    EnumMacroError,
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, Attribute, DeriveInput, Ident};

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
    // Parse the input tokens into a syntax tree
    // input is in the form of:
    // #[derive()], - optional
    // pub enum MyEnum {
    //     A = 1,  - discriminant value is optional
    //     B = 2,
    //     C = 3,
    // }

    let input = parse_macro_input!(input as DeriveInput);

    // first make sure it's an enum
    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => return TokenStream::from(quote! { compile_error!("enum_ext only works on enums"); }),
    };

    // placeholders
    let mut int_type = quote! { usize };
    let mut int_type_str = "usize".to_string();
    let mut _other_type_str = "".to_string();

    // parse the attributes. EnumDefArgs will contain stuff we're interested in. everything else (like derive etc) will be in derives_etc.
    let (my_args, derives_etc) = match process_attributes(&input.attrs) {
        Ok(result) => result,
        Err(error) => {
            let error_message = format!("{}", error);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }
    };

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

    if let Some(lit_str) = my_args.other_type {
        _other_type_str = lit_str.value();
    }

    let derive_summary = check_derive_traits(&derives_etc);

    let vis = input.vis;
    let name = input.ident;

    // Prepare the enum body with variants
    let (
        enum_body,
        variant_list,
        variant_ordinals,
        variant_map,
        to_pascal_split,
        from_pascal_split,
        variant_count,
        variant_from_ordinals,
    ) = match parse_variants(&name, &variants, &int_type) {
        Ok(result) => result,
        Err(error) => {
            let error_message = format!("{}", error);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }
    };

    let mut enum_fns = quote! {
        /// Returns an array of all variants in the enum
        #[inline]
        pub const fn list() -> [#name; #variant_count] {
            [#variant_list]
        }
        /// Returns the number of variants in the enum
        #[inline]
        pub const fn count() -> usize {
            #variant_count
        }
        /// Returns the ordinal of the variant
        #[inline]
        pub const fn ordinal(&self) -> usize {
            match self {
                #variant_ordinals
            }
        }
        /// Returns true if the ordinal is valid for the enum
        #[inline]
        pub const fn valid_ordinal(&self,ordinal : usize) -> bool {
            ordinal < #variant_count
        }
        /// Returns &Self from the ordinal.
        pub const fn ref_from_ordinal(ord: usize) -> Option<&'static Self> {
            const list : [#name; #variant_count] = #name::list();
            if ord >= #variant_count {
                return None;
            }
            Some(&list[ord])
        }
        /// Returns an iterator over the variants in the enum
        pub fn iter() -> impl Iterator<Item = &'static #name> {
            const list : [#name; #variant_count] = #name::list();
            list.iter()
        }

        /// Returns the variant name in spaced PascalCase
        /// * For example, MyEnum::InQA.pascal_spaced() returns "In QA"
        pub const fn pascal_spaced(&self) -> &'static str {
            match self {
                #to_pascal_split
            }
        }

        /// Returns the variant from the spaced PascalCase name
        /// * For example, MyEnum::from_pascal_spaced("In QA") returns Some(MyEnum::InQA)
        pub fn from_pascal_spaced(s: &str) -> Option<Self> {
            match s {
                #from_pascal_split
                _ => None,
            }
        }
    };

    let mut needed_derives = TokenStream2::new();

    let int_type_added =
        append_int_fns(&mut enum_fns, &name, variant_map, &int_type_str, &int_type);

    let mut clone_added = false;
    if int_type_added {
        if !derive_summary.has_derive {
            clone_added = true;
            needed_derives.extend(quote! {
                #[derive(Clone)]
            });
        } else {
            if !derive_summary.has_clone {
                clone_added = true;
                needed_derives.extend(quote! {
                    #[derive(Clone)]
                });
            }
        }
    }

    if derive_summary.has_clone || clone_added {
        // fn's that require Clone
        enum_fns.extend(quote! {
           /// Returns Self from the ordinal.
        pub const fn from_ordinal(ord: usize) -> Option<Self> {
            match ord {
                #variant_from_ordinals
                _ => None,
            }
        }
        });
    }

    let mut expanded_enum = quote! {
        #(#derives_etc)*
        #needed_derives
        #vis enum #name {
            #enum_body
        }

        impl #name {
            #enum_fns
        }

    };

    if int_type_added {
        let from_fn_name_str = format!("from_{}", int_type_str);
        let from_fn_name = Ident::new(&from_fn_name_str, Span::call_site());
        let impl_from = quote! {
            impl From<#int_type> for #name {
                /// Returns the enum variant from the integer value.
                /// <br><br>
                /// This will panic if the integer value is not a valid discriminant. Use the #from_fn_name or `try_from` functions
                /// instead if you want to handle invalid values.
                #[inline]
                fn from(val: #int_type) -> Self {
                    Self::#from_fn_name(val).unwrap()
                }
            }
        };

        expanded_enum.extend(impl_from);
    }

    // Convert to TokenStream and return
    expanded_enum.into()
}

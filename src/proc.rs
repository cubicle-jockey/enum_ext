use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Eq};
use syn::{parse_macro_input, Attribute, DeriveInput, Expr, Ident, LitStr, Token, Variant};

#[derive(Debug)]
enum EnumMacroError {
    ParseError(String),
    VariantError(String),
    // ... other kinds of errors
}

impl std::fmt::Display for EnumMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumMacroError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            EnumMacroError::VariantError(msg) => write!(f, "Variant error: {}", msg),
            // ... handle other errors
        }
    }
}

impl std::error::Error for EnumMacroError {}

struct EnumDefArgs {
    int_type: Option<LitStr>,
    other_type: Option<LitStr>,
    // other fields for additional configurations
}

impl Default for EnumDefArgs {
    fn default() -> Self {
        EnumDefArgs {
            int_type: None,
            other_type: None,
        }
    }
}

fn valid_int_type(int_type: &str) -> bool {
    match int_type {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" | "isize"
        | "usize" => true,
        _ => false,
    }
}

impl Parse for EnumDefArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut int_type = None;
        let mut other_type = None;
        // ... handle other fields similarly

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            if ident == "IntType" {
                let int_type_v: LitStr = input.parse()?;

                if !valid_int_type(&int_type_v.value()) {
                    return Err(syn::Error::new(int_type_v.span(), format!("Invalid IntType: {}. Supported types are i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize", int_type_v.value())));
                }

                int_type = Some(int_type_v);
            } else if ident == "OtherType" {
                other_type = Some(input.parse()?);
                // ... handle other fields similarly
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("expected IntType, found {}", ident.to_string()),
                ));
            }

            // ... handle other identifiers similarly
            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(EnumDefArgs {
            int_type,
            other_type,
            // ... set other fields
        })
    }
}

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

#[derive(Debug, Default, Clone)]
struct DeriveSummary {
    pub has_derive: bool,
    pub has_debug: bool,
    pub has_default: bool,
    pub has_clone: bool,
    pub has_copy: bool,
    pub has_partial_eq: bool,
    pub has_partial_ord: bool,
    pub has_eq: bool,
    pub has_ord: bool,
}

/// Checks whether the enum has a derives attribute and if it derives anything we may care about.
fn check_derive_traits(derive_attrs: &[Attribute]) -> DeriveSummary {
    let mut summary = DeriveSummary::default();

    for attr in derive_attrs {
        if attr.path().is_ident("derive") {
            summary.has_derive = true;
            // I was unable to find a way to check inner Ident tokens in a proc_macro2::TokenStream without converting it to a string. #noob
            match attr.meta {
                syn::Meta::List(ref meta_list) => {
                    meta_list
                        .tokens
                        .to_string()
                        .split(',')
                        .for_each(|x| match x.trim() {
                            "Clone" => {
                                summary.has_clone = true;
                            }
                            "Copy" => {
                                summary.has_copy = true;
                            }
                            "Debug" => {
                                summary.has_debug = true;
                            }
                            "Default" => {
                                summary.has_default = true;
                            }
                            "Eq" => {
                                summary.has_eq = true;
                            }
                            "Ord" => {
                                summary.has_ord = true;
                            }
                            "PartialEq" => {
                                summary.has_partial_eq = true;
                            }
                            "PartialOrd" => {
                                summary.has_partial_ord = true;
                            }
                            _ => {}
                        });
                }
                _ => {}
            }
        }
    }

    summary
}

fn split_pascal_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 1);
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_uppercase()
            && result
                .chars()
                .last()
                .map_or(false, |last| !last.is_uppercase())
        {
            result.push(' ');
        }
        result.push(c);
    }

    result
}

/// Parses the variants of an enum.
///
/// This function takes a reference to the enum name and a reference to the punctuated list of variants.
/// It returns a tuple containing:
/// - A token stream for the enum body.
/// - A token stream for the variant list.
/// - A token stream for the variant ordinals.
/// - A hashmap mapping variant identifiers to their optional discriminant values.
/// - The count of variants.
///
/// # Arguments
///
/// * `enum_name` - The identifier of the enum.
/// * `variants` - A punctuated list of the variants of the enum.
///
/// # Returns
///
/// A tuple containing:
/// - A token stream for the enum body.
/// - A token stream for the variant list.
/// - A token stream for the variant ordinals.
/// - A hashmap mapping variant identifiers to their optional discriminant values.
/// - The count of variants.
/// - A token stream for the variant from ordinals.
///
/// # Examples
///
/// ```text
/// let (enum_body, variant_list, variant_ordinals, variant_map, variant_count) =
///     parse_variants(&name, &variants);
/// ```
fn parse_variants(
    enum_name: &Ident,
    variants: &Punctuated<Variant, Comma>,
    int_type: &TokenStream2,
) -> Result<
    (
        TokenStream2,
        TokenStream2,
        TokenStream2,
        HashMap<Ident, Option<(Eq, Expr)>>,
        TokenStream2,
        TokenStream2,
        usize,
        TokenStream2,
    ),
    EnumMacroError,
> {
    let name = enum_name.clone();
    let mut enum_body = TokenStream2::new();
    let mut variant_count = 0usize;
    let mut variant_list = TokenStream2::new();
    let mut variant_ordinals = TokenStream2::new();
    let mut variant_from_ordinals = TokenStream2::new();
    let mut variant_ordinal = 0usize;
    let mut variant_ordinal2 = 0usize;
    let mut variant_map = HashMap::new();
    let mut to_pascal_split = TokenStream2::new();
    let mut from_pascal_split = TokenStream2::new();

    for variant in variants {
        if !variant.fields.is_empty() {
            // Variant has additional data (like `A(String)`)
            return Err(EnumMacroError::VariantError(format!(
                "Unsupported variant '{}': complex variants are not yet supported by enum_ext",
                variant.to_token_stream()
            )));
        }
        let variant_ident = variant.ident.clone();
        let variant_ident2 = variant.ident.clone();
        let variant_ident3 = variant.ident.clone();
        let variant_ident4 = variant.ident.clone();
        let variant_ident5 = variant.ident.clone();
        let variant_ident6 = variant.ident.clone();
        //let variant_value = variant.discriminant.clone();
        let variant_value = if let Some((_eq, expr)) = &variant.discriminant {
            let new_expr = quote! { #expr }.to_string();
            let int_type_str = int_type.to_string();
            let new_expr_with_type = format!("{}{}", new_expr, int_type_str);
            Some((
                _eq.clone(),
                syn::parse_str::<syn::Expr>(&new_expr_with_type).unwrap(),
            ))
        } else {
            None
        };

        variant_map.insert(variant_ident, variant_value);

        let variant_tokens = quote! {
            #variant,
        };
        enum_body.extend(variant_tokens);

        let variant_list_tokens = quote! {
            #name::#variant_ident2,
        };
        variant_list.extend(variant_list_tokens);
        variant_count += 1;

        let variant_ordinals_tokens = quote! {
            #name::#variant_ident3 => #variant_ordinal,
        };
        variant_ordinals.extend(variant_ordinals_tokens);
        variant_ordinal += 1;

        let pascal_split_str = split_pascal_case(&variant_ident4.to_string());
        let variant_pascal_tokens = quote! {
            #name::#variant_ident4 => #pascal_split_str,
        };
        to_pascal_split.extend(variant_pascal_tokens);

        let variant_pascal_tokens = quote! {
            #pascal_split_str => Some(#name::#variant_ident5),
        };
        from_pascal_split.extend(variant_pascal_tokens);

        let variant_ordinals_tokens = quote! {
            #variant_ordinal2 => Some(#name::#variant_ident6),
        };
        variant_from_ordinals.extend(variant_ordinals_tokens);
        variant_ordinal2 += 1;
    }

    Ok((
        enum_body,
        variant_list,
        variant_ordinals,
        variant_map,
        to_pascal_split,
        from_pascal_split,
        variant_count,
        variant_from_ordinals,
    ))
}

/// Appends integer conversion functions to the enum.
///
/// This function takes mutable references to a token stream for the functions, the enum name, a hashmap mapping variant identifiers to their optional discriminant values, a string for the integer type, and a token stream for the integer type.
/// It returns a boolean indicating whether the integer type was added to the enum.
///
/// # Arguments
///
/// * `fns` - A mutable reference to a token stream for the functions.
/// * `enum_name` - The identifier of the enum.
/// * `variant_map` - A hashmap mapping variant identifiers to their optional discriminant values.
/// * `int_type_str` - A string for the integer type.
/// * `int_type` - A token stream for the integer type.
///
/// # Returns
///
/// A boolean indicating whether the integer type was added to the enum.
///
/// # Examples
///
/// ```text
/// let int_type_added = append_int_fns(&mut enum_fns, &name, variant_map, &int_type_str, &int_type);
/// ```
fn append_int_fns(
    fns: &mut TokenStream2,
    enum_name: &Ident,
    variant_map: HashMap<Ident, Option<(Eq, Expr)>>,
    int_type_str: &str,
    int_type: &TokenStream2,
) -> bool {
    let mut from_int_tokens = TokenStream2::new();
    let mut int_type_added = false;
    for (variant_ident, variant_value) in variant_map {
        match variant_value {
            Some(v) => {
                let v = v.1;
                let variant_tokens = quote! {
                    #v => Some(#enum_name::#variant_ident),
                };
                from_int_tokens.extend(variant_tokens);
                int_type_added = true;
            }
            None => {}
        };
    }
    if int_type_added {
        // Construct the function name string and parse it into an identifier.
        let from_fn_name_str = format!("from_{}", int_type_str);
        let from_fn_name = Ident::new(&from_fn_name_str, Span::call_site());

        let as_fn_name_str = format!("as_{}", int_type_str); // Similar for the `to_` function
        let as_fn_name = Ident::new(&as_fn_name_str, Span::call_site());

        let int_helpers = quote! {

            /// Returns the enum variant from the integer value
            #[inline]
            pub const fn #from_fn_name(val: #int_type) -> Option<Self> {
                match val {
                    #from_int_tokens
                    _ => None,
                }
            }
            /// Returns the integer value from the enum variant
            #[inline]
            pub fn #as_fn_name(&self) -> #int_type {
                self.clone() as #int_type
            }

        };

        fns.extend(int_helpers);
    }
    int_type_added
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
/// - `ordinal(&self)`: Returns the ordinal of a variant.
/// - `iter()`: Returns an iterator over the variants in the enum.
/// - Conversion methods `from_<IntType>(val)` and `to_<IntType>(&self)`, if specified in the attributes.
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
///     #[derive(Debug, Clone)]
///     pub enum SimpleEnum {
///         A,
///         B,
///         C,
///     }
/// );
///
/// let x = SimpleEnum::B;
/// assert_eq!(x.ordinal(), 1);
///
/// let mut count = 0;
/// for x in SimpleEnum::iter() {
///     let i = x.ordinal();
///     assert_eq!(i, count);
///     count += 1;
/// }
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
///         A = 1, // <- do not specify the discriminant type here
///         B = 2,
///         C = 3,
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
/// ```
///
/// ## Failures
/// Example of a failure case (complex variants are not yet supported):
/// ```text
/// # use crate::enum_ext::enum_extro;
/// enum_extro!(
///     #[derive(Debug, Clone)]
///     pub enum FailureEnum {
///         A(usize),
///         B(String),
///         C,
///     }
/// );
/// // error: Variant error: Unsupported variant 'A(usize)': complex variants are not yet supported by enum_ext
/// ```

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
        pub const fn list() -> [#name; #variant_count] {
            [#variant_list]
        }
        /// Returns the number of variants in the enum
        pub const fn count() -> usize {
            #variant_count
        }
        /// Returns the ordinal of the variant
        pub const fn ordinal(&self) -> usize {
            match self {
                #variant_ordinals
            }
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

#[cfg(test)]
mod test {
    #[test]
    fn pascal_case() {
        assert_eq!(super::split_pascal_case("MyEnum"), "My Enum");
        assert_eq!(super::split_pascal_case("InQA"), "In QA");
    }
}

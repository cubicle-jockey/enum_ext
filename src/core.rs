use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Expr, LitStr, Token, Variant};

/// Returns true if the given string represents a supported valid integer type ("i8" through "usize")
pub(crate) fn valid_int_type(int_type: &str) -> bool {
    matches!(
        int_type,
        "i8" | "u8"
            | "i16"
            | "u16"
            | "i32"
            | "u32"
            | "i64"
            | "u64"
            | "i128"
            | "u128"
            | "isize"
            | "usize"
    )
}

#[derive(Debug)]
pub(crate) enum EnumMacroError {
    ParseError(String),
    VariantError(String),
}

impl std::fmt::Display for EnumMacroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnumMacroError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            EnumMacroError::VariantError(msg) => write!(f, "Variant error: {}", msg),
        }
    }
}

impl std::error::Error for EnumMacroError {}

pub(crate) struct EnumDefArgs {
    pub int_type: Option<LitStr>,
    pub other_type: Option<LitStr>,
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

#[derive(Debug, Default, Clone)]
pub(crate) struct DeriveSummary {
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
pub(crate) fn check_derive_traits(derive_attrs: &[Attribute]) -> DeriveSummary {
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

pub(crate) fn split_pascal_case(s: &str) -> String {
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
pub(crate) fn parse_variants(
    enum_name: &Ident,
    variants: &Punctuated<Variant, Comma>,
    int_type: &TokenStream2,
) -> Result<
    (
        TokenStream2,
        TokenStream2,
        TokenStream2,
        HashMap<Ident, Option<(syn::token::Eq, Expr)>>,
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
pub(crate) fn append_int_fns(
    fns: &mut TokenStream2,
    enum_name: &Ident,
    variant_map: HashMap<Ident, Option<(syn::token::Eq, Expr)>>,
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

#[cfg(test)]
mod test {
    #[test]
    fn pascal_case() {
        assert_eq!(super::split_pascal_case("MyEnum"), "My Enum");
        assert_eq!(super::split_pascal_case("InQA"), "In QA");
    }
}

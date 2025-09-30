use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use std::collections::HashMap;
use std::hash::{BuildHasher, Hasher};
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Expr, LitStr, Token, Variant, Visibility};

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
    // other fields for additional configurations
}

impl Default for EnumDefArgs {
    fn default() -> Self {
        EnumDefArgs { int_type: None }
    }
}

impl Parse for EnumDefArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut int_type = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            if ident == "IntType" {
                let int_type_v: LitStr = input.parse()?;

                if !valid_int_type(&int_type_v.value()) {
                    return Err(syn::Error::new(
                        int_type_v.span(),
                        format!(
                            "Invalid IntType: {}. Supported types are i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize",
                            int_type_v.value()
                        ),
                    ));
                }

                int_type = Some(int_type_v);
            } else {
                return Err(syn::Error::new(
                    ident.span(),
                    format!("expected IntType, found {}", ident.to_string()),
                ));
            }

            if !input.is_empty() {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(EnumDefArgs { int_type })
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

/// Splits a PascalCase string into a space-separated string.
///
/// For example, "InQA" becomes "In QA".
///
/// # Arguments
///
/// * `s` - The PascalCase string to split
///
/// # Returns
///
/// A new string with spaces inserted before uppercase letters that follow lowercase letters
pub(crate) fn split_pascal_case(s: &str) -> String {
    // Optimize by pre-allocating with extra capacity for spaces
    let mut result = String::with_capacity(s.len() + 5);

    // Track the previous character to avoid calling chars().last() repeatedly
    let mut prev_char_type = CharType::None;

    for c in s.chars() {
        if c.is_uppercase() && prev_char_type == CharType::Lowercase {
            result.push(' ');
        }

        result.push(c);

        // Update previous character type
        prev_char_type = if c.is_uppercase() {
            CharType::Uppercase
        } else if c.is_lowercase() {
            CharType::Lowercase
        } else {
            CharType::Other
        };
    }

    result
}

/// Converts a PascalCase string to snake_case.
///
/// For example, "InQA" becomes "in_qa".
///
/// # Arguments
///
/// * `s` - The PascalCase string to convert
///
/// # Returns
///
/// A new string in snake_case format
pub(crate) fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 5);
    let mut prev_char_type = CharType::None;

    for c in s.chars() {
        if c.is_uppercase() && prev_char_type == CharType::Lowercase {
            result.push('_');
        }

        result.push(c.to_lowercase().next().unwrap_or(c));

        prev_char_type = if c.is_uppercase() {
            CharType::Uppercase
        } else if c.is_lowercase() {
            CharType::Lowercase
        } else {
            CharType::Other
        };
    }

    result
}

/// Converts a PascalCase string to kebab-case.
///
/// For example, "InQA" becomes "in-qa".
///
/// # Arguments
///
/// * `s` - The PascalCase string to convert
///
/// # Returns
///
/// A new string in kebab-case format
pub(crate) fn to_kebab_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 5);
    let mut prev_char_type = CharType::None;

    for c in s.chars() {
        if c.is_uppercase() && prev_char_type == CharType::Lowercase {
            result.push('-');
        }

        result.push(c.to_lowercase().next().unwrap_or(c));

        prev_char_type = if c.is_uppercase() {
            CharType::Uppercase
        } else if c.is_lowercase() {
            CharType::Lowercase
        } else {
            CharType::Other
        };
    }

    result
}

pub struct ParsedVariants {
    pub enum_body: TokenStream2,
    pub variant_list: TokenStream2,
    pub variant_ordinals: TokenStream2,
    pub variant_map: HashMap<Ident, Option<(syn::token::Eq, Expr)>, DeterministicHasher>,
    pub to_pascal_split: TokenStream2,
    pub from_pascal_split: TokenStream2,
    pub to_snake_case: TokenStream2,
    pub from_snake_case: TokenStream2,
    pub to_kebab_case: TokenStream2,
    pub from_kebab_case: TokenStream2,
    pub variant_name_tokens: TokenStream2,
    pub variant_count: usize,
    pub variant_from_ordinals: TokenStream2,
}

/// Enum to track character types for split_pascal_case
#[derive(PartialEq)]
enum CharType {
    None,
    Uppercase,
    Lowercase,
    Other,
}

/// Parses the variants of an enum.
///
/// This function takes a reference to the enum name and a reference to the punctuated list of variants.
/// It returns a `ParsedVariants` struct containing all the parsed variant information.
///
/// # Arguments
///
/// * `enum_name` - The identifier of the enum.
/// * `variants` - A punctuated list of the variants of the enum.
/// * `int_type` - The integer type for discriminant values.
///
/// # Returns
///
/// A `Result<ParsedVariants, EnumMacroError>` containing:
/// - `enum_body` - A token stream for the enum body.
/// - `variant_list` - A token stream for the variant list.
/// - `variant_ordinals` - A token stream for the variant ordinals.
/// - `variant_map` - A hashmap mapping variant identifiers to their optional discriminant values.
/// - `to_pascal_split` - A token stream for converting variants to pascal-spaced strings.
/// - `from_pascal_split` - A token stream for converting pascal-spaced strings to variants.
/// - `variant_count` - The count of variants.
/// - `variant_from_ordinals` - A token stream for the variant from ordinals.
///
/// # Examples
///
/// ```text
/// let parsed_variants = parse_variants(&name, &variants, &int_type)?;
/// ```
pub(crate) fn parse_variants(
    enum_name: &Ident,
    variants: &Punctuated<Variant, Comma>,
    int_type: &TokenStream2,
) -> Result<ParsedVariants, EnumMacroError> {
    let name = enum_name.clone();
    let mut enum_body = TokenStream2::new();
    let mut variant_count = 0usize;
    let mut variant_list = TokenStream2::new();
    let mut variant_ordinals = TokenStream2::new();
    let mut variant_from_ordinals = TokenStream2::new();
    let mut variant_ordinal = 0usize;
    let mut variant_ordinal2 = 0usize;
    let mut variant_map = HashMap::with_hasher(DeterministicHasher::new());
    let mut to_pascal_split = TokenStream2::new();
    let mut from_pascal_split = TokenStream2::new();
    let mut to_snake_case_tokens = TokenStream2::new();
    let mut from_snake_case_tokens = TokenStream2::new();
    let mut to_kebab_case_tokens = TokenStream2::new();
    let mut from_kebab_case_tokens = TokenStream2::new();
    let mut variant_name_tokens = TokenStream2::new();

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

        // Generate snake_case conversions
        let snake_case_str = to_snake_case(&variant_ident4.to_string());
        let variant_snake_tokens = quote! {
            #name::#variant_ident4 => #snake_case_str,
        };
        to_snake_case_tokens.extend(variant_snake_tokens);

        let variant_snake_tokens = quote! {
            #snake_case_str => Some(#name::#variant_ident5),
        };
        from_snake_case_tokens.extend(variant_snake_tokens);

        // Generate kebab-case conversions
        let kebab_case_str = to_kebab_case(&variant_ident4.to_string());
        let variant_kebab_tokens = quote! {
            #name::#variant_ident4 => #kebab_case_str,
        };
        to_kebab_case_tokens.extend(variant_kebab_tokens);

        let variant_kebab_tokens = quote! {
            #kebab_case_str => Some(#name::#variant_ident5),
        };
        from_kebab_case_tokens.extend(variant_kebab_tokens);

        // Generate variant name tokens for metadata extraction
        let variant_name_str = variant_ident4.to_string();
        let variant_name_match_tokens = quote! {
            #name::#variant_ident4 => #variant_name_str,
        };
        variant_name_tokens.extend(variant_name_match_tokens);

        let variant_ordinals_tokens = quote! {
            #variant_ordinal2 => Some(#name::#variant_ident6),
        };
        variant_from_ordinals.extend(variant_ordinals_tokens);
        variant_ordinal2 += 1;
    }

    Ok(ParsedVariants {
        enum_body,
        variant_list,
        variant_ordinals,
        variant_map,
        to_pascal_split,
        from_pascal_split,
        to_snake_case: to_snake_case_tokens,
        from_snake_case: from_snake_case_tokens,
        to_kebab_case: to_kebab_case_tokens,
        from_kebab_case: from_kebab_case_tokens,
        variant_name_tokens,
        variant_count,
        variant_from_ordinals,
    })
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
    variant_map: HashMap<Ident, Option<(syn::token::Eq, Expr)>, DeterministicHasher>,
    int_type_str: &str,
    int_type: &TokenStream2,
    has_copy: bool,
) -> bool {
    // Filter the map first to avoid empty matches
    let variants_with_values: Vec<_> = variant_map
        .into_iter()
        .filter_map(|(ident, value)| value.map(|v| (ident, v.1)))
        .collect();

    let int_type_added = !variants_with_values.is_empty();

    if int_type_added {
        // Generate tokens for all variants with values
        let from_int_tokens = variants_with_values.iter().map(|(ident, v)| {
            quote! { #v => Some(#enum_name::#ident), }
        });

        // Construct the function name string and parse it into an identifier.
        let from_fn_name_str = format!("from_{}", int_type_str);
        let from_fn_name = Ident::new(&from_fn_name_str, Span::call_site());

        let as_fn_name_str = format!("as_{}", int_type_str);
        let as_fn_name = Ident::new(&as_fn_name_str, Span::call_site());

        let int_helpers = if !has_copy {
            quote! {
                /// Returns the enum variant from the integer value
                #[inline]
                pub const fn #from_fn_name(val: #int_type) -> Option<Self> {
                    match val {
                        #(#from_int_tokens)*
                        _ => None,
                    }
                }
                /// Returns the integer value from the enum variant
                #[inline]
                pub fn #as_fn_name(&self) -> #int_type {
                    self.clone() as #int_type
                }
            }
        } else {
            quote! {
                /// Returns the enum variant from the integer value
                #[inline]
                pub const fn #from_fn_name(val: #int_type) -> Option<Self> {
                    match val {
                        #(#from_int_tokens)*
                        _ => None,
                    }
                }
                /// Returns the integer value from the enum variant
                #[inline]
                pub const fn #as_fn_name(&self) -> #int_type {
                    *self as #int_type
                }
            }
        };

        fns.extend(int_helpers);
    }

    int_type_added
}

/// Constructs the pretty print string for the enum.
pub(crate) fn make_pretty_print(
    attrs: &[Attribute],
    needed_derives: &TokenStream2,
    vis: &Visibility,
    name: &Ident,
    enum_body: &TokenStream2,
    repl_value: &TokenStream2,
) -> String {
    let mut pretty_print_body = Vec::with_capacity(10); // Pre-allocate with reasonable capacity
    let attrs_str = (quote! { #(#attrs)* }).to_string().trim().to_owned();
    if !attrs_str.is_empty() {
        pretty_print_body.push(attrs_str);
        pretty_print_body.push("\n".to_owned());
    }
    let needed_derives_str = (quote! { #needed_derives }).to_string().trim().to_owned();
    if !needed_derives_str.is_empty() {
        pretty_print_body.push(needed_derives_str);
        pretty_print_body.push("\n".to_owned());
    }
    let repl_value_str = (quote! { #repl_value }).to_string().trim().to_owned();
    if !repl_value_str.is_empty() {
        pretty_print_body.push(repl_value_str);
        pretty_print_body.push("\n".to_owned());
    }
    let decla = (quote! { #vis enum #name }).to_string().trim().to_owned();
    pretty_print_body.push(decla);
    pretty_print_body.push(" {\n".to_owned());

    // Optimize string handling by using a more efficient approach
    let enum_body_str = enum_body.to_string();
    let parts: Vec<&str> = enum_body_str.trim().split(',').map(str::trim).collect();

    // Join non-empty parts with commas and newlines, preserving trailing comma
    let mut formatted_parts = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        if !part.is_empty() {
            formatted_parts.push(*part);
        } else if i == parts.len() - 1 {
            // This is the trailing empty part after the last comma
            break;
        }
    }

    let formatted_body = if formatted_parts.is_empty() {
        String::new()
    } else {
        format!("{},", formatted_parts.join(",\n    "))
    };

    pretty_print_body.push("    ".to_owned());
    pretty_print_body.push(formatted_body);

    pretty_print_body.push("\n}".to_owned());

    pretty_print_body.join("")
}

/// Generates the expanded enum with all implementations.
///
/// This function centralizes the logic for generating the expanded enum with all its implementations,
/// reducing code duplication between the procedural and attribute macros.
///
/// # Arguments
///
/// * `attrs` - The attributes of the enum
/// * `vis` - The visibility of the enum
/// * `name` - The name of the enum
/// * `variants` - The variants of the enum
/// * `int_type_str` - The integer type as a string
/// * `int_type` - The integer type as a TokenStream
///
/// # Returns
///
/// A TokenStream containing the expanded enum with all implementations
pub(crate) fn generate_expanded_enum(
    attrs: &[Attribute],
    vis: &Visibility,
    name: &Ident,
    variants: &Punctuated<Variant, Comma>,
    int_type_str: &str,
    int_type: &TokenStream2,
    int_type_specified: bool,
) -> Result<TokenStream2, EnumMacroError> {
    if variants.len() == 0 {
        //panic!("cannot generate methods for empty enums");
        return Err(EnumMacroError::VariantError(
            "cannot generate methods for empty enums".to_owned(),
        ));
    }
    let derive_summary = check_derive_traits(attrs);

    let parsed_vars = parse_variants(name, variants, int_type)?;

    // Parse variants
    let (
        enum_body,
        variant_list,
        variant_ordinals,
        variant_map,
        to_pascal_split,
        from_pascal_split,
        to_snake_case,
        from_snake_case,
        to_kebab_case,
        from_kebab_case,
        variant_name_tokens,
        variant_count,
        variant_from_ordinals,
    ) = (
        parsed_vars.enum_body,
        parsed_vars.variant_list,
        parsed_vars.variant_ordinals,
        parsed_vars.variant_map,
        parsed_vars.to_pascal_split,
        parsed_vars.from_pascal_split,
        parsed_vars.to_snake_case,
        parsed_vars.from_snake_case,
        parsed_vars.to_kebab_case,
        parsed_vars.from_kebab_case,
        parsed_vars.variant_name_tokens,
        parsed_vars.variant_count,
        parsed_vars.variant_from_ordinals,
    );

    // Generate enum functions
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
        pub const fn valid_ordinal(ordinal : usize) -> bool {
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
        /// Returns the variant name in snake_case
        /// * For example, MyEnum::InQA.snake_case() returns "in_qa"
        pub const fn snake_case(&self) -> &'static str {
            match self {
                #to_snake_case
            }
        }
        /// Returns the variant from the snake_case name
        /// * For example, MyEnum::from_snake_case("in_qa") returns Some(MyEnum::InQA)
        pub fn from_snake_case(s: &str) -> Option<Self> {
            match s {
                #from_snake_case
                _ => None,
            }
        }
        /// Returns the variant name in kebab-case
        /// * For example, MyEnum::InQA.kebab_case() returns "in-qa"
        pub const fn kebab_case(&self) -> &'static str {
            match self {
                #to_kebab_case
            }
        }
        /// Returns the variant from the kebab-case name
        /// * For example, MyEnum::from_kebab_case("in-qa") returns Some(MyEnum::InQA)
        pub fn from_kebab_case(s: &str) -> Option<Self> {
            match s {
                #from_kebab_case
                _ => None,
            }
        }
        /// Returns the next variant in ordinal order (wraps around)
        pub const fn next(&self) -> &'static Self {
            let current_ordinal = self.ordinal();
            let next_ordinal = (current_ordinal + 1) % #variant_count;
            Self::ref_from_ordinal(next_ordinal).unwrap()
        }
        /// Returns the previous variant in ordinal order (wraps around)
        pub const fn previous(&self) -> &'static Self {
            let current_ordinal = self.ordinal();
            let prev_ordinal = if current_ordinal == 0 {
                #variant_count - 1
            } else {
                current_ordinal - 1
            };
            Self::ref_from_ordinal(prev_ordinal).unwrap()
        }
        /// Returns the next variant without wrapping (returns None at end)
        pub const fn next_linear(&self) -> Option<&'static Self> {
            let current_ordinal = self.ordinal();
            if current_ordinal + 1 >= #variant_count {
                None
            } else {
                Self::ref_from_ordinal(current_ordinal + 1)
            }
        }
        /// Returns the previous variant without wrapping (returns None at start)
        pub const fn previous_linear(&self) -> Option<&'static Self> {
            let current_ordinal = self.ordinal();
            if current_ordinal == 0 {
                None
            } else {
                Self::ref_from_ordinal(current_ordinal - 1)
            }
        }
        /// Check if this is the first variant (ordinal 0)
        pub const fn is_first(&self) -> bool {
            self.ordinal() == 0
        }
        /// Check if this is the last variant
        pub const fn is_last(&self) -> bool {
            self.ordinal() == #variant_count - 1
        }
        /// Compare ordinal positions - returns true if self comes before other
        pub const fn comes_before(&self, other: &Self) -> bool {
            self.ordinal() < other.ordinal()
        }
        /// Compare ordinal positions - returns true if self comes after other
        pub const fn comes_after(&self, other: &Self) -> bool {
            self.ordinal() > other.ordinal()
        }
        /// Returns variants whose names contain the substring
        pub fn variants_containing(substring: &str) -> Vec<&'static Self> {
            Self::iter()
                .filter(|variant| variant.pascal_spaced().contains(substring))
                .collect()
        }
        /// Returns variants whose names start with the prefix
        pub fn variants_starting_with(prefix: &str) -> Vec<&'static Self> {
            Self::iter()
                .filter(|variant| variant.pascal_spaced().starts_with(prefix))
                .collect()
        }
        /// Returns variants whose names end with the suffix
        pub fn variants_ending_with(suffix: &str) -> Vec<&'static Self> {
            Self::iter()
                .filter(|variant| variant.pascal_spaced().ends_with(suffix))
                .collect()
        }
        /// Returns a slice of variants from start to end ordinal
        pub fn slice(start: usize, end: usize) -> &'static [Self] {
            const LIST : [#name; #variant_count] = #name::list();
            const EMPTY : [#name; 0] = [];
            if start >= #variant_count || end > #variant_count || start >= end {
                return &EMPTY;
            }

            &LIST[start..end]
        }
        /// Returns variants in the specified ordinal range
        pub fn range(range: core::ops::Range<usize>) -> &'static [Self] {
            Self::slice(range.start, range.end)
        }
        /// Returns the first N variants
        pub fn first_n(n: usize) -> &'static [Self] {
            Self::slice(0, n.min(#variant_count))
        }
        /// Returns the last N variants
        pub fn last_n(n: usize) -> &'static [Self]  {
            let start = if n >= #variant_count { 0 } else { #variant_count - n };
            Self::slice(start, #variant_count)
        }
        /// Returns the variant name as a string (metadata extraction)
        pub const fn variant_name(&self) -> &'static str {
            match self {
                #variant_name_tokens
            }
        }
        /// Returns all variant names as a vector of strings
        pub fn variant_names() -> Vec<&'static str> {
            Self::iter().map(|v| v.variant_name()).collect()
        }
    };

    // Add random methods if "random" feature is enabled
    #[cfg(feature = "random")]
    {
        enum_fns.extend(quote! {
            /// Returns a random variant (requires "random" feature)
            pub fn random() -> &'static Self {
                use rand::Rng;
                let mut rng = rand::rng();
                Self::random_with_rng(&mut rng)
            }
            /// Returns a random variant using provided RNG (requires "random" feature)
            pub fn random_with_rng<R: rand::Rng>(rng: &mut R) -> &'static Self {
                let random_ordinal = rng.random_range(0..#variant_count);
                Self::ref_from_ordinal(random_ordinal).unwrap()
            }
        });
    }

    // Add integer conversion functions if needed
    let mut needed_derives = TokenStream2::new();
    let int_type_added = append_int_fns(
        &mut enum_fns,
        name,
        variant_map,
        int_type_str,
        int_type,
        derive_summary.has_copy,
    );

    // Add Clone derive if needed
    let mut clone_added = false;
    if int_type_added {
        if !derive_summary.has_derive {
            clone_added = true;
            needed_derives.extend(quote! {
                #[derive(Clone)]
            });
        } else if !derive_summary.has_clone {
            clone_added = true;
            needed_derives.extend(quote! {
                #[derive(Clone)]
            });
        }
    }

    // Add repr attribute if needed (emit if user specified IntType or if discriminants exist)
    let mut repl_value = TokenStream2::new();
    if int_type_specified || int_type_added {
        repl_value.extend(quote! {
            #[repr(#int_type)]
        });
    }

    // Add from_ordinal if Clone is available
    if derive_summary.has_clone || clone_added {
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

    // Generate pretty_print body
    let pretty_print_body =
        make_pretty_print(attrs, &needed_derives, vis, name, &enum_body, &repl_value);

    let pretty_print_body = LitStr::new(&pretty_print_body, Span::call_site());

    // Generate the expanded enum
    let mut expanded_enum = quote! {
        #(#attrs)*
        #needed_derives
        #repl_value
        #vis enum #name {
            #enum_body
        }

        impl #name {
            #enum_fns

            /// Returns a pretty printed string of the enum definition
            pub const fn pretty_print() -> &'static str {
                #pretty_print_body
            }
        }
    };

    // Add From implementation if int_type is specified
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

    Ok(expanded_enum)
}

/// ### Deterministic Hasher
/// this is not a secure or collision-free hasher and should not be used outside of this crate.
/// - purpose is to guarantee consistent hashes
pub(crate) struct DeterministicHasher {
    value: u64,
}

impl DeterministicHasher {
    fn new() -> Self {
        Self { value: 0 }
    }
}

impl Hasher for DeterministicHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.value = self.value.rotate_left(8) + *b as u64;
        }
    }
}

impl BuildHasher for DeterministicHasher {
    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        DeterministicHasher::new()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn pascal_case() {
        assert_eq!(super::split_pascal_case("MyEnum"), "My Enum");
        assert_eq!(super::split_pascal_case("InQA"), "In QA");
    }
}

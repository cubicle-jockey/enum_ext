use super::core::{EnumDefArgs, EnumMacroError, generate_expanded_enum, resolve_int_type};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{Attribute, DeriveInput, parse_macro_input};

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
/// `Ok((EnumDefArgs, Vec<Attribute>))` where the first element contains the parsed `enum_def` arguments
/// and the second element contains all attributes not related to `enum_def`.
///
/// # Errors
///
/// Returns `Err(EnumMacroError::ParseError)` if parsing the `enum_def` attribute fails or if multiple
/// `enum_def` attributes are present.
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
    let mut not_mine = Vec::<Attribute>::new();
    let mut my_args: Option<EnumDefArgs> = None;
    for attr in attrs {
        if attr.path().is_ident("enum_def") {
            if my_args.is_some() {
                return Err(EnumMacroError::ParseError(
                    "Multiple `enum_def` attributes found; only one is allowed".to_string(),
                ));
            }
            let args: EnumDefArgs = attr
                .parse_args_with(EnumDefArgs::parse)
                .map_err(|e| EnumMacroError::ParseError(e.to_string()))?;
            my_args = Some(args);
        } else {
            not_mine.push(attr.clone());
        }
    }
    Ok((my_args.unwrap_or_else(EnumDefArgs::default), not_mine))
}

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

    let (int_type, int_type_str, int_type_specified) = match resolve_int_type(&my_args) {
        Ok(result) => result,
        Err(error) => {
            let error_message = format!("{}", error);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }
    };

    // Generate the expanded enum using the shared function
    match generate_expanded_enum(
        &derives_etc,
        &input.vis,
        &input.ident,
        &variants,
        &int_type_str,
        &int_type,
        int_type_specified,
    ) {
        Ok(expanded) => expanded.into(),
        Err(error) => {
            let error_message = format!("{}", error);
            TokenStream::from(quote! { compile_error!(#error_message); })
        }
    }
}

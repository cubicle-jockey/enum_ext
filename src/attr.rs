use super::core::{EnumDefArgs, generate_expanded_enum, resolve_int_type};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[doc = include_str!("../ATTR.md")]
pub fn enum_extend(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EnumDefArgs);
    let input = parse_macro_input!(item as DeriveInput);

    // Ensure it's an enum
    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => {
            return TokenStream::from(
                quote! { compile_error!("enum_extend only works on enums"); },
            );
        }
    };

    let (int_type, int_type_str, int_type_specified) = match resolve_int_type(&args) {
        Ok(result) => result,
        Err(error) => {
            let error_message = format!("{}", error);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }
    };

    // Generate the expanded enum using the shared function
    match generate_expanded_enum(
        &input.attrs,
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

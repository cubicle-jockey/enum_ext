use super::core::{
    generate_expanded_enum, valid_int_type, EnumDefArgs,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[doc = include_str!("../ATTR.md")]
pub fn enum_extend(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EnumDefArgs);
    let input = parse_macro_input!(item as DeriveInput);

    // Ensure it's an enum
    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => {
            return TokenStream::from(quote! { compile_error!("enum_extend only works on enums"); })
        }
    };

    // Set up integer type
    let mut int_type = quote! { usize };
    let mut int_type_str = "usize".to_string();

    if let Some(lit_str) = args.int_type {
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
        &input.attrs,
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

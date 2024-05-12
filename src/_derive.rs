/*
 this will need to be rewritten. unlike proc macros, derive macros cannot modify the input, only append to it.
 so in its current form, it causes the enum to be duplicated in the output...
*/
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::spanned::Spanned;
use syn::Meta::List;
use syn::{parse_macro_input, Attribute, DeriveInput, LitStr, Meta, MetaList};

pub fn enum_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    // Check if input is an enum
    if let syn::Data::Enum(_) = input.data {
        // Manipulate attributes to remove EnumExt from derive
        let filtered_attrs: Vec<Attribute> = input
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path().is_ident("derive") {
                    // Convert the derive list to string and filter out EnumExt

                    if let List(meta_list) = attr.meta.clone() {
                        let traits: Vec<String> = meta_list
                            .tokens
                            .into_iter()
                            .map(|m| m.into_token_stream().to_string())
                            .filter(|name| !name.contains("EnumExt")) // Remove "EnumExt" from the list
                            .collect();

                        if traits.is_empty() {
                            return None; // if all traits are filtered out, skip this attribute
                        }

                        let traits_str = traits.join(", ");
                        let new_tokens = format!("derive({})", traits_str);
                        let new_lit = LitStr::new(&new_tokens, attr.span());

                        return Some(Attribute {
                            //meta: attr.path().clone(),
                            //bracket_token: quote! { (#new_lit) },
                            meta: List(MetaList {
                                path: attr.path().clone(),
                                delimiter: meta_list.delimiter,

                                tokens: quote! { #new_lit },
                            }),
                            ..attr.clone()
                        });
                    } else {
                        return None;
                    }
                } else {
                    return None; //Some(attr.clone());
                }
                //Some(attr.clone()) // not a derive attribute or couldn't parse, return original
            })
            .collect();

        //input.attrs = filtered_attrs;

        // Continue with original macro logic...
        let name = &input.ident;
        let vis = &input.vis;
        let generics = &input.generics;
        let enum_items: Vec<_> = if let syn::Data::Enum(data) = &input.data {
            data.variants.iter().map(|v| quote! { #v }).collect()
        } else {
            vec![]
        };

        let output = quote! {
            enum_ext::enum_ext! {
                #(#filtered_attrs)*
                #vis enum #name #generics {
                    #(#enum_items),*
                }
            }
        };

        // Convert to TokenStream and return
        TokenStream::from(output)
    } else {
        TokenStream::from(quote! { compile_error!("EnumExt only supports enums"); })
    }
}

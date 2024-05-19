use super::core::{check_derive_traits, parse_variants, valid_int_type, EnumDefArgs};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::HashMap;
use syn::token::Eq;
use syn::{parse_macro_input, DeriveInput, Expr, Ident};

#[doc = include_str!("../ATTR.md")]
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
        let from_fn_name_str = format!("from_{}", int_type_str);
        let from_fn_name = Ident::new(&from_fn_name_str, Span::call_site());

        let as_fn_name_str = format!("as_{}", int_type_str);
        let as_fn_name = Ident::new(&as_fn_name_str, Span::call_site());

        let int_helpers = quote! {
            #[inline]
            pub const fn #from_fn_name(val: #int_type) -> Option<Self> {
                match val {
                    #from_int_tokens
                    _ => None,
                }
            }

            #[inline]
            pub fn #as_fn_name(&self) -> #int_type {
                self.clone() as #int_type
            }
        };

        fns.extend(int_helpers);
    }
    int_type_added
}

pub fn enum_ext(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EnumDefArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => return TokenStream::from(quote! { compile_error!("enum_ext only works on enums"); }),
    };

    let mut int_type = quote! { usize };
    let mut int_type_str = "usize".to_string();
    let mut _other_type_str = "".to_string();

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

    if let Some(lit_str) = args.other_type {
        _other_type_str = lit_str.value();
    }

    let derive_summary = check_derive_traits(&input.attrs);

    let vis = &input.vis;
    let name = &input.ident;
    let attrs = &input.attrs;

    let (
        enum_body,
        variant_list,
        variant_ordinals,
        variant_map,
        to_pascal_split,
        from_pascal_split,
        variant_count,
        variant_from_ordinals,
    ) = match parse_variants(name, &variants, &int_type) {
        Ok(result) => result,
        Err(error) => {
            let error_message = format!("{}", error);
            return TokenStream::from(quote! { compile_error!(#error_message); });
        }
    };

    let mut enum_fns = quote! {
        pub const fn list() -> [#name; #variant_count] {
            [#variant_list]
        }

        pub const fn count() -> usize {
            #variant_count
        }

        pub const fn ordinal(&self) -> usize {
            match self {
                #variant_ordinals
            }
        }

        pub const fn ref_from_ordinal(ord: usize) -> Option<&'static Self> {
            const list : [#name; #variant_count] = #name::list();
            if ord >= #variant_count {
                return None;
            }
            Some(&list[ord])
        }

        pub fn iter() -> impl Iterator<Item = &'static #name> {
            const list : [#name; #variant_count] = #name::list();
            list.iter()
        }

        pub const fn pascal_spaced(&self) -> &'static str {
            match self {
                #to_pascal_split
            }
        }

        pub fn from_pascal_spaced(s: &str) -> Option<Self> {
            match s {
                #from_pascal_split
                _ => None,
            }
        }
    };

    let mut needed_derives = TokenStream2::new();

    let int_type_added = append_int_fns(&mut enum_fns, name, variant_map, &int_type_str, &int_type);

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
        enum_fns.extend(quote! {
           pub const fn from_ordinal(ord: usize) -> Option<Self> {
                match ord {
                    #variant_from_ordinals
                    _ => None,
                }
            }
        });
    }

    let expanded_enum = quote! {
        #(#attrs)*
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
                #[inline]
                fn from(val: #int_type) -> Self {
                    Self::#from_fn_name(val).unwrap()
                }
            }
        };

        let expanded_enum = quote! {
            #expanded_enum
            #impl_from
        };

        expanded_enum.into()
    } else {
        expanded_enum.into()
    }
}

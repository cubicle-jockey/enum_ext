use super::core::{
    append_int_fns, check_derive_traits, make_pretty_print, parse_variants, valid_int_type,
    EnumDefArgs,
};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[doc = include_str!("../ATTR.md")]
pub fn enum_extend(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as EnumDefArgs);
    let input = parse_macro_input!(item as DeriveInput);

    let variants = match input.data {
        syn::Data::Enum(e) => e.variants,
        _ => {
            return TokenStream::from(quote! { compile_error!("enum_extend only works on enums"); })
        }
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
        /// * For example, MyEnum::InQA. pascal_spaced() returns "In QA"
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

    let mut repl_value = TokenStream2::new();
    //dbg!(int_type_added);
    if int_type_added {
        repl_value.extend(quote! {
            #[repr(#int_type)]
        });
        //dbg!(&repl_value);
    }

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

    let attrs2 = attrs.clone();
    let needed_derives2 = needed_derives.clone();
    let repl_value2 = repl_value.clone();
    let vis2 = vis.clone();
    let name2 = name.clone();
    let enum_body2 = enum_body.clone();
    let pretty_print_body = make_pretty_print(
        attrs2,
        needed_derives2,
        vis2,
        name2,
        enum_body2,
        repl_value2,
    );

    let expanded_enum = quote! {
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

        let expanded_enum = quote! {
            #expanded_enum
            #impl_from
        };

        expanded_enum.into()
    } else {
        expanded_enum.into()
    }
}

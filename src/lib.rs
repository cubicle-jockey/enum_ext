mod proc;
//mod derive;

#[proc_macro]
pub fn enum_ext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc::enum_ext(input)
}

/* FUTURE...
#[proc_macro_derive(EnumExt, attributes(enum_def))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::enum_derive(input)
}

 */

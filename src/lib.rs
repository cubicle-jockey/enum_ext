#[doc = include_str!("../README.md")]
#[cfg(doctest)]
struct ReadmeDocTests;

#[doc = include_str!("../PROCS.md")]
#[cfg(doctest)]
struct ProcsDocTests;

#[doc = include_str!("../ATTR.md")]
#[cfg(doctest)]
struct AttrDocTests;

#[doc = include_str!("../README.md")]
mod attr;
mod core;
mod proc;

#[doc = include_str!("../PROCS.md")]
#[proc_macro]
pub fn enum_ext(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc::enum_ext(input)
}

#[doc = include_str!("../ATTR.md")]
#[proc_macro_attribute]
pub fn enum_extend(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    attr::enum_ext(attr, item)
}

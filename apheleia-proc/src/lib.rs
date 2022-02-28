#![deny(
    non_ascii_idents,
    // missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    future_incompatible,
    missing_debug_implementations,
    missing_copy_implementations,
    nonstandard_style,
    unreachable_pub,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    rustdoc::broken_intra_doc_links
)]
#![feature(proc_macro_span, proc_macro_diagnostic)]

use proc_macro::TokenStream;
use syn::parse_macro_input;

mod queries;
mod subject_area;

#[proc_macro_attribute]
pub fn subject_area(attr: TokenStream, item: TokenStream) -> TokenStream {
    subject_area::subject_area(attr, item)
}

#[proc_macro]
pub fn queries(item: TokenStream) -> TokenStream {
    let generator = parse_macro_input!(item as queries::QueriesGenerator);
    match queries::queries(generator) {
        Ok(t) => t,
        Err(e) => e.into_compile_error().into(),
    }
}

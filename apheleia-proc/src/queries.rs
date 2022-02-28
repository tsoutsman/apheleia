use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, Lit, Token,
};

#[derive(Debug)]
pub(crate) struct QueriesGenerator {
    /// Absolute path to the SQL file.
    file_path: PathBuf,
    lit_span: proc_macro2::Span,
    args: Vec<(Ident, Expr)>,
}

impl Parse for QueriesGenerator {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut rust_file_path = proc_macro::Span::call_site().source_file().path();
        rust_file_path.pop();

        let (file_path, lit_span) = match Lit::parse(input)? {
            Lit::Str(file_path) => match file_path.value().parse::<PathBuf>() {
                Ok(path_buf) => (path_buf, file_path.span()),
                Err(_) => return Err(syn::Error::new(file_path.span(), "invalid file path")),
            },
            lit => return Err(syn::Error::new(lit.span(), "expected string literal")),
        };
        let file_path = rust_file_path.join(file_path);

        let mut args = Vec::new();
        // In a while loop because I may add more args in the future.
        while !input.is_empty() {
            input.parse::<Token![,]>()?;
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let expr = input.parse()?;
            args.push((ident, expr));
        }
        Ok(Self {
            file_path,
            lit_span,
            args,
        })
    }
}

pub(crate) fn queries(generator: QueriesGenerator) -> syn::Result<TokenStream> {
    let file_contents = std::fs::read_to_string(generator.file_path)
        .map_err(|_| syn::Error::new(generator.lit_span, "error reading file"))?;
    let consume_args = generator
        .args
        .iter()
        // format inception
        .map(|(i, _)| format!("{{{}:.0}}", i))
        .collect::<String>();
    let args = generator.args.iter().map(|(i, e)| quote! { #i = #e });
    let args = quote! { #(#args),* };
    let mut formats = file_contents
        .split_inclusive(';')
        .map(|l| quote! { &format!(concat!(#l, #consume_args), #args) })
        .collect::<Vec<_>>();
    // Remove trailing newline
    formats.pop();
    let x = quote! {
        [#(::sqlx::query(#formats)),*]
    }
    .into();
    Ok(x)
}

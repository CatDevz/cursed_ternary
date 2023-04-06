#![warn(
    clippy::wildcard_imports,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::manual_ok_or,
    unused_lifetimes
)]

use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Peek};
use syn::{parse_macro_input, Expr, Token};

struct TernaryInput {
    condition: Expr,
    if_true: Expr,
    if_false: Expr,
}

impl Parse for TernaryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        fn parse_until(input: ParseStream, terminator: impl Peek) -> syn::Result<Expr> {
            let mut tokens = TokenStream::new();
            while !input.peek(terminator) {
                tokens.append(TokenTree::parse(input)?);
            }

            TokenTree::parse(input)?;
            syn::parse2(tokens)
        }

        Ok(Self {
            condition: parse_until(input, Token![?])?,
            if_true: parse_until(input, Token![:])?,
            if_false: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn t(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let TernaryInput {
        condition,
        if_true,
        if_false,
    } = parse_macro_input!(input as TernaryInput);

    quote! {
        if #condition {
            #if_true
        } else {
            #if_false
        }
    }
    .into()
}

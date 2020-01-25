//! # enum-primitive-conversion
//!
//! [![Build Status](https://www.travis-ci.org/ssalonen/enum-repr-derive.svg?branch=master)](https://www.travis-ci.org/ssalonen/enum-repr-derive)
//! [![Crate](https://img.shields.io/crates/v/enum-repr-derive.svg)](https://crates.io/enum-repr-derive)
//! [![Documentation](https://docs.rs/enum-repr-derive/badge.svg)](https://docs.rs/enum-repr-derive)
//!
//! Procedural derive macro for converting fieldless enums to (`Into`) and from (`TryFrom`) its repr type.
//!
//! See the [Nomicon section on `repr`](https://doc.rust-lang.org/nomicon/other-reprs.html#repru-repri) for more details on fieldless enums.
//!
//! ## Example code
//!
//! By using this library the following code just works:
//!
//! ```rust
//! extern crate enum_repr_derive;
//! use enum_repr_derive::{Into, TryFrom};
//! use std::convert::TryFrom;
//!
//! #[repr(i8)]
//! #[derive(TryFrom, Into, Copy, Clone, Debug, PartialEq)]
//! enum Foo {
//!     VAR1 = -1,
//!     VAR2 = -3,
//! }
//! assert_eq!(Foo::try_from(-1), Ok(Foo::VAR1));
//! assert_eq!(Foo::try_from(-3), Ok(Foo::VAR2));
//! assert_eq!(Foo::try_from(-9), Err(-9));
//! assert_eq!(Into::<i8>::into(Foo::VAR1), -1);
//! assert_eq!(Into::<i8>::into(Foo::VAR2), -3);
//! ```
//!
//! ## License
//!
//! Licensed under MIT. See `LICENSE` file.
//!
//! ## For developers
//!
//! Release: `cargo release`
//!

extern crate proc_macro;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::quote;
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Expr, Ident};

#[proc_macro_derive(TryFrom)]
#[proc_macro_error]
pub fn derive_try_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let reprtype = find_repr_type(input.attrs);
    let enum_name = input.ident;

    let enum_data = get_enum_data(&input.data);
    let match_expr = match_impl(&enum_name, enum_data);

    proc_macro::TokenStream::from(quote! {

        impl core::convert::TryFrom<#reprtype> for #enum_name  {
            type Error = #reprtype;

            fn try_from(val : #reprtype) -> Result<Self, <Self as core::convert::TryFrom<#reprtype>>::Error> {
                #match_expr
            }
        }
    })
}

#[proc_macro_derive(Into)]
#[proc_macro_error]
pub fn derive_into(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let reprtype = find_repr_type(input.attrs);
    let enum_name = input.ident;

    proc_macro::TokenStream::from(quote! {

        impl core::convert::Into<#reprtype> for #enum_name  {
               fn into(self : #enum_name) -> #reprtype {
                self as #reprtype
            }
        }
    })
}

fn find_repr_type(attrs: Vec<Attribute>) -> Ident {
    for attr in attrs {
        match attr.style {
            AttrStyle::Outer => {
                if attr.path.is_ident(&Ident::new("repr", Span::call_site())) {
                    let tokens = attr.tokens;
                    let mut repr_tokens_iter = tokens.into_iter();
                    let first_token: TokenTree = repr_tokens_iter.next().unwrap();
                    if repr_tokens_iter.next().is_some() {
                        abort!(
                            first_token.span(),
                            "Repr is malformed, expecting repr(TYPE)"
                        );
                    }
                    let repr_type = match first_token.clone() {
                        TokenTree::Group(repr_items) => {
                            if repr_items.delimiter() != Delimiter::Parenthesis {
                                abort!(repr_items.span(), "Repr is malformed, expecting repr(TYPE)")
                            }
                            let mut repr_types_iter = repr_items.stream().into_iter();
                            let first_repr_item = repr_types_iter.next().unwrap();
                            // Unwrap if many repr types are specified
                            if let Some(second_repr_type) = repr_types_iter.next() {
                                abort!(
                                    second_repr_type.span(),
                                    "Many repr types specified. Expecting only one."
                                )
                            }
                            match first_repr_item.clone() {
                                TokenTree::Ident(repr_type) => repr_type,
                                unexpected_type => abort!(
                                    first_repr_item.span(),
                                    "Unexpected type in repr {}",
                                    unexpected_type
                                ),
                            }
                        }
                        unexpected_token => abort!(
                            first_token.span(),
                            "Unexpected token with repr {}",
                            unexpected_token
                        ),
                    };
                    return repr_type;
                }
            }
            _ => {
                // skipping non-outer attributes
                continue;
            }
        }
    }
    abort_call_site!("Repr not found");
}

fn get_enum_data(data: &Data) -> Vec<(Ident, Expr)> {
    let mut enum_data: Vec<(Ident, Expr)> = Vec::new();
    match *data {
        Data::Enum(ref data) => {
            for variant in data.variants.iter() {
                let pair = variant.discriminant.as_ref().unwrap();
                let expr = pair.1.clone();
                enum_data.push((variant.ident.clone(), expr));
            }
        }
        Data::Struct(_) | Data::Union(_) => {
            abort_call_site!("Unexpected type! Use derive with enums only")
        }
    }
    enum_data
}

fn match_impl(enum_name: &Ident, enum_data: Vec<(Ident, Expr)>) -> TokenStream {
    let mut match_arms = TokenStream::new();
    for (id, expr) in enum_data {
        match_arms.extend(quote! { #expr => Ok(#enum_name::#id),});
    }
    match_arms.extend(quote! { unexpected => Err(unexpected) });
    quote! {
        match val {
            #match_arms
        }
    }
}

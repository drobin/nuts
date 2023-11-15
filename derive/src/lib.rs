// MIT License
//
// Copyright (c) 2023 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro implementation of the [`FromBytes`] trait.
///
/// This derive macro generates a [`FromBytes`] implementation for `struct` and
/// `enum` types. `union` types are currently not supported.
///
/// [`FromBytes`]: trait.FromBytes.html
#[proc_macro_derive(FromBytes)]
pub fn from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let from_impl = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = &field.ident;

                    quote!(
                        #field_name: FromBytes::from_bytes(source)?
                    )
                });

                quote!( Ok(#name { #(#fields,)* }) )
            }
            Fields::Unnamed(fields) => {
                let fields =
                    (0..fields.unnamed.len()).map(|_| quote!(FromBytes::from_bytes(source)?));

                quote!(
                    Ok(#name( #(#fields,)* ))
                )
            }
            Fields::Unit => quote!(
                Ok(#name)
            ),
        },
        Data::Enum(data) => {
            if data.variants.len() > 0 {
                let variants = data.variants.iter().enumerate().map(|(idx, variant)| {
                    let variant_name = &variant.ident;

                    let fields = match &variant.fields {
                        Fields::Named(fields) => {
                            let fields = fields.named.iter().map(|field| {
                                let field_name = &field.ident;

                                quote!(
                                    #field_name: FromBytes::from_bytes(source)?
                                )
                            });

                            quote!( { #(#fields,)* } )
                        }
                        Fields::Unnamed(fields) => {
                            let fields = (0..fields.unnamed.len())
                                .map(|_| quote!(FromBytes::from_bytes(source)?));

                            quote!(
                                ( #(#fields,)* )
                            )
                        }
                        Fields::Unit => quote!(),
                    };

                    quote!(
                        #idx => {
                            Ok(#name::#variant_name #fields )
                        }
                    )
                });

                quote!(
                    let idx: usize = FromBytes::from_bytes(source)?;

                    match idx {
                        #(#variants,)*
                        _=> Err(E::invalid_variant_index(idx))
                    }
                )
            } else {
                let span = name.span();

                quote_spanned!(
                    span => compile_error!("zero-variant enums cannot be instantiated")
                )
            }
        }
        Data::Union(_data) => {
            let span = name.span();

            quote_spanned! {
                span => compile_error!("the union type is currently not supported")
            }
        }
    };

    let expanded = quote! {
        impl<E:nuts_bytes::TakeDeriveError> nuts_bytes::FromBytes<E> for #name {
            fn from_bytes<TB: nuts_bytes::TakeBytes>(source: &mut TB) -> Result<Self, E> {
                #from_impl
            }
        }
    };

    TokenStream::from(expanded)
}

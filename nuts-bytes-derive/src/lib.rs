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

mod attr;

use attr::FieldAttributes;
use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

macro_rules! parse_field_attributes {
    ($input:expr) => {
        match FieldAttributes::parse($input) {
            Ok(attrs) => attrs,
            Err(err) => return err.into_compile_error().into(),
        }
    };
}

/// Derive macro implementation of the [`FromBytes`] trait.
///
/// This derive macro generates a [`FromBytes`] implementation for `struct` and
/// `enum` types. `union` types are currently not supported.
///
/// # Attributes
///
/// * **`#[nuts_bytes(map_from_bytes = $path)]`**
///
/// Calls the function `$path` on the deserialized value.
/// The given function must be callable as
/// `fn<S: FromBytes, T, E: Into<Box<dyn std::error::Error + Send + Sync>>(S) -> std::result::Result<T, E>`
///
/// If the function succeeds, the returned value is assigned to this field. An
/// error is converted into a [`Error::Custom`] error, where the error (`E`) is
/// attached to the [`Error::Custom`] variant.
///
/// * **`#[nuts_bytes(map = $module)]`**
///
/// Combination of `map_from_bytes` and `map_to_bytes`. The crate will use
/// `$module::from_bytes` as the `map_from_bytes` function and
/// `$module::to_bytes` as the `map_to_bytes` function.
///
/// [`FromBytes`]: trait.FromBytes.html
/// [`Error::Custom`]: enum.Error.html#variant.Custom
#[proc_macro_derive(FromBytes, attributes(nuts_bytes))]
pub fn from_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let from_impl = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let attributes = parse_field_attributes!(&field.attrs);
                    let field_name = &field.ident;

                    if let Some(map_func) = attributes.map_from_bytes() {
                        quote!(
                            #field_name: {
                                let value_in = FromBytes::from_bytes(source)?;

                                match #map_func(value_in) {
                                    Ok(value_out) => { value_out }
                                    Err(err) => { return Err(nuts_bytes::Error::Custom(err.into())); }
                                }
                            }
                        )
                    } else {
                        quote!(
                            #field_name: FromBytes::from_bytes(source)?
                        )
                    }
                });

                quote!( #name { #(#fields,)* } )
            }
            Fields::Unnamed(fields) => {
                let fields = fields.unnamed.iter().map(|field| {
                    let attributes = parse_field_attributes!(&field.attrs);

                    if let Some(map_func) = attributes.map_from_bytes() {
                        quote! {
                            {
                                let value_in = FromBytes::from_bytes(source)?;

                                match #map_func(value_in) {
                                    Ok(value_out) => { value_out }
                                    Err(err) => { return Err(nuts_bytes::Error::Custom(err.into())); }
                                }
                            }
                        }
                    } else {
                        quote!(FromBytes::from_bytes(source)?)
                    }
                });

                quote!(
                    #name( #(#fields,)* )
                )
            }
            Fields::Unit => quote!(
                #name
            ),
        },
        Data::Enum(data) => {
            if data.variants.len() > 0 {
                let variants = data
                    .variants
                    .iter()
                    .take(u32::MAX as usize)
                    .enumerate()
                    .map(|(idx, variant)| {
                        let variant_idx = idx as u32;
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
                            #variant_idx => {
                                #name::#variant_name #fields
                            }
                        )
                    });

                quote!(
                    let idx: u32 = FromBytes::from_bytes(source)?;

                    match idx {
                        #(#variants,)*
                        _=> { return Err(nuts_bytes::Error::InvalidVariantIndex(idx)); }
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
        impl #impl_generics nuts_bytes::FromBytes for #name #ty_generics #where_clause {
            fn from_bytes<TB: nuts_bytes::TakeBytes>(source: &mut TB) -> std::result::Result<Self, nuts_bytes::Error> {
                let result = { #from_impl };

                Ok(result)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro implementation of the [`ToBytes`] trait.
///
/// This derive macro generates a [`ToBytes`] implementation for `struct` and
/// `enum` types. `union` types are currently not supported.
///
/// [`ToBytes`]: trait.ToBytes.html
#[proc_macro_derive(ToBytes)]
pub fn to_bytes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let to_impl = match input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().enumerate().map(|(idx, field)| {
                let variant_idx = Index::from(idx);
                let field_ref = field
                    .ident
                    .as_ref()
                    .map_or_else(|| quote!(&self.#variant_idx), |ident| quote!(&self.#ident));

                quote!(ToBytes::to_bytes(#field_ref, target)?)
            });

            quote! {
                let mut n = 0;

                #(n += #fields;)*

                Ok(n)
            }
        }
        Data::Enum(data) => {
            if data.variants.len() > 0 {
                let variants = data
                    .variants
                    .iter()
                    .take(u32::MAX as usize)
                    .enumerate()
                    .map(|(idx, variant)| {
                        let variant_idx = Index::from(idx);
                        let variant_name = &variant.ident;

                        let left_arm_args =
                            variant.fields.iter().enumerate().map(|(idx, field)| {
                                let ident = field.ident.as_ref().map_or_else(
                                    || format_ident!("f{}", Index::from(idx)),
                                    |ident| ident.clone(),
                                );

                                quote!(#ident)
                            });
                        let left_arm = match &variant.fields {
                            Fields::Named(_) => {
                                quote!( #name::#variant_name { #(#left_arm_args),* } )
                            }
                            Fields::Unnamed(_) => {
                                quote!( #name::#variant_name ( #(#left_arm_args),* ) )
                            }
                            Fields::Unit => quote!( #name::#variant_name ),
                        };

                        let right_arm_fields =
                            variant.fields.iter().enumerate().map(|(idx, field)| {
                                let ident = field.ident.as_ref().map_or_else(
                                    || format_ident!("f{}", Index::from(idx)),
                                    |ident| ident.clone(),
                                );
                                quote!(ToBytes::to_bytes(#ident, target)?)
                            });
                        let right_arm = quote! {
                            {
                                let mut m = 0;

                                m += ToBytes::to_bytes(&(#variant_idx as u32), target)?;
                                #(m += #right_arm_fields;)*

                                m
                            }
                        };

                        quote! {
                            #left_arm => #right_arm
                        }
                    });

                quote! {
                    let n = match self {
                        #(#variants,)*
                    };

                    Ok(n)
                }
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
        impl #impl_generics nuts_bytes::ToBytes for #name #ty_generics #where_clause {
            fn to_bytes<PB: nuts_bytes::PutBytes>(&self, target: &mut PB) -> std::result::Result<usize, nuts_bytes::Error> {
                #to_impl
            }
        }
    };

    TokenStream::from(expanded)
}

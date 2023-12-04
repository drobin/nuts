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
mod from_bytes;
mod to_bytes;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

use crate::from_bytes::{from_bytes_enum, from_bytes_struct, from_bytes_union};
use crate::to_bytes::{to_bytes_enum, to_bytes_struct, to_bytes_union};

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
        Data::Struct(data) => from_bytes_struct(&name, data),
        Data::Enum(data) => from_bytes_enum(&name, data),
        Data::Union(_data) => from_bytes_union(&name),
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
        Data::Struct(data) => to_bytes_struct(&name, data),
        Data::Enum(data) => to_bytes_enum(&name, data),
        Data::Union(_data) => to_bytes_union(&name),
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

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

use quote::{format_ident, quote, quote_spanned};
use syn::{DataEnum, DataStruct, Fields, Ident, Index};

pub fn to_bytes_struct(_struct_name: &Ident, data: DataStruct) -> proc_macro2::TokenStream {
    let fields = data.fields.iter().enumerate().map(|(idx, field)| {
        let variant_idx = Index::from(idx);
        let field_ref = field
            .ident
            .as_ref()
            .map_or_else(|| quote!(&self.#variant_idx), |ident| quote!(&self.#ident));

        quote! {
            ToBytes::to_bytes(#field_ref, target)?
        }
    });

    quote! {
        let mut n = 0;

        #(n += #fields;)*

        Ok(n)
    }
}

pub fn to_bytes_enum(enum_name: &Ident, data: DataEnum) -> proc_macro2::TokenStream {
    if data.variants.len() == 0 {
        let span = enum_name.span();

        return quote_spanned! {
            span => compile_error!("zero-variant enums cannot be instantiated")
        };
    }

    let variants = data
        .variants
        .iter()
        .take(u32::MAX as usize)
        .enumerate()
        .map(|(idx, variant)| {
            let variant_idx = Index::from(idx);
            let variant_name = &variant.ident;

            let left_arm_args = variant.fields.iter().enumerate().map(|(idx, field)| {
                let ident = field.ident.as_ref().map_or_else(
                    || format_ident!("f{}", Index::from(idx)),
                    |ident| ident.clone(),
                );

                quote!(#ident)
            });
            let left_arm = match &variant.fields {
                Fields::Named(_) => {
                    quote!( #enum_name::#variant_name { #(#left_arm_args),* } )
                }
                Fields::Unnamed(_) => {
                    quote!( #enum_name::#variant_name ( #(#left_arm_args),* ) )
                }
                Fields::Unit => quote!( #enum_name::#variant_name ),
            };

            let right_arm_fields = variant.fields.iter().enumerate().map(|(idx, field)| {
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
}

pub fn to_bytes_union(union_name: &Ident) -> proc_macro2::TokenStream {
    let span = union_name.span();

    quote_spanned! {
        span => compile_error!("the union type is currently not supported")
    }
}

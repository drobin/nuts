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
use syn::{DataEnum, DataStruct, Fields, FieldsNamed, FieldsUnnamed, Ident};

use crate::attr::{parse_field_attributes, FieldAttributes};

fn call_map_func(attributes: &FieldAttributes, value_in: &Ident) -> proc_macro2::TokenStream {
    if let Some(map_func) = attributes.map_from_bytes() {
        quote! {
            match #map_func(#value_in) {
                Ok(value_out) => { value_out }
                Err(err) => { return Err(nuts_bytes::Error::Custom(err.into())); }
            }
        }
    } else {
        quote! {
            #value_in
        }
    }
}

fn named_struct(struct_name: &Ident, fields: FieldsNamed) -> proc_macro2::TokenStream {
    let fields = fields.named.iter().map(|field| {
        let attributes = parse_field_attributes!(&field.attrs);
        let field_name = &field.ident;

        if attributes.is_skip() {
            let default_func = if let Some(default) = attributes.default() {
                quote!(#default())
            } else {
                quote!(Default::default())
            };

            quote! {
                #field_name: #default_func
            }
        } else {
            let value_in = format_ident!("value_in");
            let map_func = call_map_func(&attributes, &value_in);

            quote! {
                #field_name: {
                    let #value_in = FromBytes::from_bytes(source)?;
                    #map_func
                }
            }
        }
    });

    quote! { #struct_name { #(#fields,)* } }
}

fn unnamed_struct(struct_name: &Ident, fields: FieldsUnnamed) -> proc_macro2::TokenStream {
    let fields = fields.unnamed.iter().map(|field| {
        let attributes = parse_field_attributes!(&field.attrs);

        if attributes.is_skip() {
            if let Some(default) = attributes.default() {
                quote! {
                    #default()
                }
            } else {
                quote! {
                    Default::default()
                }
            }
        } else {
            let value_in = format_ident!("value_in");
            let map_func = call_map_func(&attributes, &value_in);

            quote! {
                {
                    let #value_in = FromBytes::from_bytes(source)?;
                    #map_func
                }
            }
        }
    });

    quote! {
        #struct_name( #(#fields,)* )
    }
}

fn unit_struct(struct_name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        #struct_name
    }
}

pub fn from_bytes_struct(struct_name: &Ident, data: DataStruct) -> proc_macro2::TokenStream {
    match data.fields {
        Fields::Named(fields) => named_struct(struct_name, fields),
        Fields::Unnamed(fields) => unnamed_struct(struct_name, fields),
        Fields::Unit => unit_struct(struct_name),
    }
}

fn named_enum(_enum_name: &Ident, fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let fields = fields.named.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: FromBytes::from_bytes(source)?
        }
    });

    quote!( { #(#fields,)* } )
}

fn unnamed_enum(_enum_name: &Ident, fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let fields = (0..fields.unnamed.len()).map(|_| quote!(FromBytes::from_bytes(source)?));

    quote! {
        ( #(#fields,)* )
    }
}

pub fn from_bytes_enum(enum_name: &Ident, data: DataEnum) -> proc_macro2::TokenStream {
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
            let variant_idx = idx as u32;
            let variant_name = &variant.ident;

            let fields = match &variant.fields {
                Fields::Named(fields) => named_enum(enum_name, fields),
                Fields::Unnamed(fields) => unnamed_enum(enum_name, fields),
                Fields::Unit => quote!(),
            };

            quote! {
                #variant_idx => {
                    #enum_name::#variant_name #fields
                }
            }
        });

    quote! {
        let idx: u32 = FromBytes::from_bytes(source)?;

        match idx {
            #(#variants,)*
            _=> { return Err(nuts_bytes::Error::InvalidVariantIndex(idx)); }
        }
    }
}

pub fn from_bytes_union(union_name: &Ident) -> proc_macro2::TokenStream {
    let span = union_name.span();

    quote_spanned! {
        span => compile_error!("the union type is currently not supported")
    }
}

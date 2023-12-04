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

use quote::format_ident;
use std::borrow::Cow;
use std::ops::Deref;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{AttrStyle, Attribute, Ident, Path, Result, Token};

fn is_nuts_bytes_attr(attr: &Attribute) -> bool {
    attr.style == AttrStyle::Outer && attr.path().is_ident("nuts_bytes")
}

struct AttributeList<T>(Punctuated<T, Token![,]>);

impl<T> Deref for AttributeList<T> {
    type Target = Punctuated<T, Token![,]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Parse> Parse for AttributeList<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Punctuated::parse_separated_nonempty)?;

        Ok(AttributeList(attrs))
    }
}

#[derive(Clone)]
enum FieldAttribute {
    Map(Path),
    MapFromBytes(Path),
    MapToBytes(Path),
}

impl FieldAttribute {
    fn as_map(&self) -> Option<&Path> {
        match self {
            Self::Map(path) => Some(path),
            _ => None,
        }
    }

    fn as_map_from_bytes(&self) -> Option<&Path> {
        match self {
            Self::MapFromBytes(path) => Some(path),
            _ => None,
        }
    }

    fn as_map_to_bytes(&self) -> Option<&Path> {
        match self {
            Self::MapToBytes(path) => Some(path),
            _ => None,
        }
    }
}

impl Parse for FieldAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;

        if key == "map" {
            let _: Token![=] = input.parse()?;
            Ok(Self::Map(input.parse()?))
        } else if key == "map_from_bytes" {
            let _: Token![=] = input.parse()?;
            Ok(Self::MapFromBytes(input.parse()?))
        } else if key == "map_to_bytes" {
            let _: Token![=] = input.parse()?;
            Ok(Self::MapToBytes(input.parse()?))
        } else {
            Err(syn::Error::new_spanned(
                key,
                "unsupported attribute for nuts_bytes",
            ))
        }
    }
}

pub struct FieldAttributes(Vec<FieldAttribute>);

impl FieldAttributes {
    pub fn parse(attrs: &Vec<Attribute>) -> Result<FieldAttributes> {
        let mut attr_vec = vec![];

        let filtered = attrs.iter().filter(|attr| is_nuts_bytes_attr(attr));

        for attr in filtered {
            let list = attr.parse_args::<AttributeList<FieldAttribute>>()?;
            attr_vec.extend(list.iter().map(Clone::clone));
        }

        Ok(FieldAttributes(attr_vec))
    }

    pub fn map_from_bytes<'a>(&'a self) -> Option<Cow<'a, Path>> {
        if let Some(attr) = self
            .0
            .iter()
            .find(|attr| attr.as_map_from_bytes().is_some())
        {
            attr.as_map_from_bytes().map(Cow::Borrowed)
        } else {
            self.0
                .iter()
                .find(|attr| attr.as_map().is_some())
                .map(|attr| {
                    let mut path = attr.as_map().unwrap().clone();

                    path.segments.push(format_ident!("from_bytes").into());

                    Cow::Owned(path)
                })
        }
    }

    pub fn map_to_bytes<'a>(&'a self) -> Option<Cow<'a, Path>> {
        if let Some(attr) = self.0.iter().find(|attr| attr.as_map_to_bytes().is_some()) {
            attr.as_map_to_bytes().map(Cow::Borrowed)
        } else {
            self.0
                .iter()
                .find(|attr| attr.as_map().is_some())
                .map(|attr| {
                    let mut path = attr.as_map().unwrap().clone();

                    path.segments.push(format_ident!("to_bytes").into());

                    Cow::Owned(path)
                })
        }
    }
}

macro_rules! parse_field_attributes {
    ($input:expr) => {
        match crate::attr::FieldAttributes::parse($input) {
            Ok(attrs) => attrs,
            Err(err) => return err.into_compile_error().into(),
        }
    };
}

pub(crate) use parse_field_attributes;

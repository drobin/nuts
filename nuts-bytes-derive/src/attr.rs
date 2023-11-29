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

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{AttrStyle, Attribute, Ident, Meta, Result, Token};

enum FromBytesAttribute {
    Validate,
}

impl FromBytesAttribute {
    fn validate(&self) -> bool {
        match self {
            Self::Validate => true,
        }
    }
}

impl Parse for FromBytesAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;

        if key == "validate" {
            Ok(FromBytesAttribute::Validate)
        } else {
            return Err(syn::Error::new_spanned(
                key,
                "unsupported attribute for from_bytes",
            ));
        }
    }
}

struct FromBytesAttributeList {
    attrs: Punctuated<FromBytesAttribute, Token![,]>,
}

impl Parse for FromBytesAttributeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Punctuated::parse_separated_nonempty)?;

        Ok(FromBytesAttributeList { attrs })
    }
}

pub struct FromBytesAttributes(Vec<FromBytesAttributeList>);

impl FromBytesAttributes {
    pub fn parse(attrs: &Vec<Attribute>) -> Result<FromBytesAttributes> {
        let mut attr_vec = vec![];

        let filtered = attrs.iter().filter(|attr| match attr.style {
            AttrStyle::Outer => match &attr.meta {
                Meta::Path(_) => false,
                Meta::List(list) => match list.path.get_ident() {
                    Some(ident) => ident == "from_bytes",
                    None => false,
                },
                Meta::NameValue(_) => false,
            },
            AttrStyle::Inner(_) => false,
        });

        for attr in filtered {
            match attr.parse_args::<FromBytesAttributeList>() {
                Ok(list) => attr_vec.push(list),
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(FromBytesAttributes(attr_vec))
    }

    pub fn validate(&self) -> bool {
        self.any(|a| a.validate())
    }

    fn any<F: FnMut(&FromBytesAttribute) -> bool + Copy>(&self, f: F) -> bool {
        self.0.iter().any(|p| p.attrs.iter().any(f))
    }
}

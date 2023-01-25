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

use std::collections::HashMap;
use std::ops::Deref;

use crate::plugin;

pub enum Variant {
    String(String),
    U32(u32),
    Bool(bool),
}

impl From<String> for Variant {
    fn from(s: String) -> Self {
        Variant::String(s)
    }
}

impl From<u32> for Variant {
    fn from(n: u32) -> Self {
        Variant::U32(n)
    }
}

impl From<bool> for Variant {
    fn from(b: bool) -> Self {
        Variant::Bool(b)
    }
}

pub struct BasicOptions(HashMap<String, Variant>);

macro_rules! impl_as_method {
    ($fn:ident -> $ty:ty, Variant :: $memb:ident) => {
        pub fn $fn<K: AsRef<str>>(&self, key: K) -> plugin::Result<&$ty> {
            match self.get(key.as_ref()) {
                Some(Variant::$memb(val)) => Ok(val),
                Some(_) => Err(plugin::Error::InvalidOption(key.as_ref().to_string())),
                None => Err(plugin::Error::NoSuchOption(key.as_ref().to_string())),
            }
        }
    };
}

impl BasicOptions {
    fn new() -> BasicOptions {
        BasicOptions(HashMap::new())
    }

    pub fn add(&mut self, key: String, value: Variant) {
        self.0.insert(key, value);
    }

    impl_as_method!(get_as_str -> str, Variant::String);
    impl_as_method!(get_as_u32 -> u32, Variant::U32);
    impl_as_method!(get_as_bool -> bool, Variant::Bool);
}

impl Deref for BasicOptions {
    type Target = HashMap<String, Variant>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! make_basic_options_impl {
    ($name:ident) => {
        pub struct $name(BasicOptions);

        impl $name {
            pub fn new() -> $name {
                $name(BasicOptions::new())
            }

            pub fn add<K: AsRef<str>, V: Into<Variant>>(mut self, key: K, value: V) -> Self {
                self.0.add(key.as_ref().to_string(), value.into());
                self
            }
        }

        impl Deref for $name {
            type Target = BasicOptions;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

make_basic_options_impl!(CreateOptions);
make_basic_options_impl!(OpenOptions);

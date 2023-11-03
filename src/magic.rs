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

pub(crate) const MAGIC: [u8; 12] = *b"nuts-archive";

macro_rules! magic_type {
    ($name:ident, $err:literal, size) => {
        impl $name {
            fn size() -> usize {
                std::mem::size_of::<[u8; 12]>()
            }
        }

        magic_type!($name, $err);
    };

    ($name:ident, $err:literal) => {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        #[serde(try_from = "[u8; 12]")]
        struct $name([u8; 12]);

        impl $name {
            fn new() -> $name {
                $name(crate::magic::MAGIC)
            }
        }

        impl<T: AsRef<[u8]>> PartialEq<T> for $name {
            fn eq(&self, other: &T) -> bool {
                self.0 == other.as_ref()
            }
        }

        impl std::convert::TryFrom<[u8; 12]> for $name {
            type Error = String;

            fn try_from(buf: [u8; 12]) -> Result<Self, String> {
                if buf == crate::magic::MAGIC {
                    Ok(Magic(buf))
                } else {
                    Err($err.to_string())
                }
            }
        }
    };
}

pub(crate) use magic_type;

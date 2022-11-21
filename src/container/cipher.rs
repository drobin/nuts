// MIT License
//
// Copyright (c) 2022 Robin Doer
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

use std::io::{Read, Write};

use crate::bytes::{self, FromBytes, FromBytesExt, ToBytes, ToBytesExt};

/// Supported cipher algorithms.
#[derive(Clone, Copy, Debug)]
pub enum Cipher {
    /// No encryption.
    None,
}

impl FromBytes for Cipher {
    fn from_bytes<R: Read>(source: &mut R) -> bytes::Result<Self> {
        let n = source.from_bytes()?;

        match n {
            0u8 => Ok(Cipher::None),
            _ => Err(bytes::Error::invalid(format!("invalid cipher: {}", n))),
        }
    }
}

impl ToBytes for Cipher {
    fn to_bytes<W: Write>(&self, target: &mut W) -> bytes::Result<()> {
        let n = match self {
            Cipher::None => 0u8,
        };

        target.to_bytes(&n)
    }
}

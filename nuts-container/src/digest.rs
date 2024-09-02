// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

#[cfg(test)]
mod tests;

use openssl::hash::MessageDigest;
use std::fmt;
use std::str::FromStr;

use crate::buffer::{Buffer, BufferError, BufferMut};

/// Supported message digests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Digest {
    /// SHA1
    Sha1,

    /// SHA2 with a 224 bit hash
    Sha224,

    /// SHA2 with a 256 bit hash
    Sha256,

    /// SHA2 with a 384 bit hash
    Sha384,

    /// SHA2 with a 512 bit hash
    Sha512,
}

impl fmt::Display for Digest {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Digest::Sha1 => "sha1",
            Digest::Sha224 => "sha224",
            Digest::Sha256 => "sha256",
            Digest::Sha384 => "sha384",
            Digest::Sha512 => "sha512",
        };

        fmt.write_str(s)
    }
}

impl FromStr for Digest {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, ()> {
        match str {
            "sha1" => Ok(Digest::Sha1),
            "sha224" => Ok(Digest::Sha224),
            "sha256" => Ok(Digest::Sha256),
            "sha384" => Ok(Digest::Sha384),
            "sha512" => Ok(Digest::Sha512),
            _ => Err(()),
        }
    }
}

impl Digest {
    /// Return the size of the message digest.
    ///
    /// This is the size of the resulting hash.
    pub fn size(&self) -> usize {
        match self {
            Digest::Sha1 => 20,
            Digest::Sha224 => 28,
            Digest::Sha256 => 32,
            Digest::Sha384 => 48,
            Digest::Sha512 => 64,
        }
    }

    pub(crate) fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Digest, BufferError> {
        let b = buf.get_u32()?;

        match b {
            0 => Ok(Digest::Sha1),
            1 => Ok(Digest::Sha224),
            2 => Ok(Digest::Sha256),
            3 => Ok(Digest::Sha384),
            4 => Ok(Digest::Sha512),
            _ => Err(BufferError::InvalidIndex("Digest".to_string(), b)),
        }
    }

    pub(crate) fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), BufferError> {
        let b = match self {
            Digest::Sha1 => 0,
            Digest::Sha224 => 1,
            Digest::Sha256 => 2,
            Digest::Sha384 => 3,
            Digest::Sha512 => 4,
        };

        buf.put_u32(b)
    }

    pub(crate) fn as_openssl(&self) -> MessageDigest {
        match self {
            Digest::Sha1 => MessageDigest::sha1(),
            Digest::Sha224 => MessageDigest::sha224(),
            Digest::Sha256 => MessageDigest::sha256(),
            Digest::Sha384 => MessageDigest::sha384(),
            Digest::Sha512 => MessageDigest::sha512(),
        }
    }
}

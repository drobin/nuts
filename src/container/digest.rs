// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported message digests.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Digest {
    /// SHA1
    Sha1,
}

impl fmt::Display for Digest {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Digest::Sha1 => "sha1",
        };

        fmt.write_str(s)
    }
}

impl FromStr for Digest {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, ()> {
        match str {
            "sha1" => Ok(Digest::Sha1),
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
        }
    }

    pub(crate) fn to_openssl(&self) -> MessageDigest {
        match self {
            Digest::Sha1 => MessageDigest::sha1(),
        }
    }
}

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

use nuts_backend::{Binary, IdSize};
use std::convert::TryInto;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::error::{Error, Result};

#[cfg(test)]
fn rand_bytes() -> Result<[u8; SIZE]> {
    Ok([
        0xdb, 0x3d, 0x05, 0x23, 0xd4, 0x50, 0x75, 0x30, 0xe8, 0x6d, 0xf9, 0x6a, 0x1b, 0x76, 0xaa,
        0x0c,
    ])
}

#[cfg(not(test))]
fn rand_bytes() -> Result<[u8; SIZE]> {
    let mut buf = [0; SIZE];

    // std::io::Error implements From<getrandom::Error>
    // 1. Convert getrandom::Error into std::io::Error
    // 2. Convert std::io::Error into $crate::error::Error
    getrandom::getrandom(&mut buf).map_err(Into::<std::io::Error>::into)?;

    Ok(buf)
}

const SIZE: usize = 16;
const HEX: [char; SIZE] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// The [id](nuts_backend::Backend::Id) of the backend.
///
/// This id as an 16 byte random number.
///
/// When storing a block to disks the path to the file is derived from the id:
/// * The id is converted into a hex string.
/// * The path then would be: `<first two chars>/<next two chars>/<remaining chars>`
#[derive(Clone, PartialEq)]
pub struct Id([u8; SIZE]);

impl Binary for Id {
    fn from_bytes(bytes: &[u8]) -> Option<Id> {
        match bytes.try_into() {
            Ok(buf) => Some(Id(buf)),
            Err(_) => None,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl IdSize for Id {
    fn size() -> usize {
        SIZE
    }
}

impl Id {
    pub(crate) fn generate() -> Result<Id> {
        rand_bytes().map(Id)
    }

    pub(crate) fn min() -> Id {
        Id([u8::MIN; SIZE])
    }

    fn as_hex(&self) -> String {
        let mut target = String::with_capacity(2 * SIZE);

        for b in self.0.iter() {
            target.push(HEX[(*b as usize >> 4) & 0x0f]);
            target.push(HEX[(*b as usize) & 0x0f]);
        }

        target
    }

    pub(crate) fn to_pathbuf(&self, parent: &Path) -> PathBuf {
        let hex = self.as_hex();
        let mut path = PathBuf::new();
        let mut pos = 0;

        path.push(parent);

        for _ in 0..2 {
            path.push(&hex[pos..pos + 2]);
            pos += 2;
        }

        path.push(&hex[pos..]);

        path
    }
}

impl fmt::Display for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_hex())
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_tuple("Id").field(&self.to_string()).finish()
    }
}

impl FromStr for Id {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 2 * SIZE {
            return Err(Error::InvalidId(s.to_string()));
        }

        let mut id = Id([0; SIZE]);

        for (idx, c) in s.chars().enumerate() {
            if let Some(n) = c.to_digit(16) {
                let m = idx / 2;

                if idx % 2 == 0 {
                    id.0[m] |= (n as u8 & 0x0f) << 4;
                } else {
                    id.0[m] |= n as u8 & 0x0f;
                }
            } else {
                return Err(Error::InvalidId(s.to_string()));
            }
        }

        Ok(id)
    }
}

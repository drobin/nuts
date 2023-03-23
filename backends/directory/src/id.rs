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

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use nuts_backend::BlockId;

use crate::error::{Error, Result};

#[cfg(test)]
fn rand_bytes() -> [u8; SIZE] {
    [
        0xdb, 0x3d, 0x05, 0x23, 0xd4, 0x50, 0x75, 0x30, 0xe8, 0x6d, 0xf9, 0x6a, 0x1b, 0x76, 0xaa,
        0x0c,
    ]
}

#[cfg(not(test))]
fn rand_bytes() -> [u8; SIZE] {
    let mut buf = [0; SIZE];

    getrandom::getrandom(&mut buf).unwrap();

    buf
}

const SIZE: usize = 16;
const HEX: [char; SIZE] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// The id of the backend.
///
/// This id as an 16 byte random number.
///
/// When storing a block to disks the path to the file is derive from the id:
/// * The id is converted into a hex string.
/// * Ths path then would be: `<first two chars>/<next two chars>/<remaining chars>`
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DirectoryId([u8; SIZE]);

impl DirectoryId {
    pub(crate) fn generate() -> DirectoryId {
        DirectoryId(rand_bytes())
    }

    pub(crate) fn min() -> DirectoryId {
        DirectoryId([u8::MIN; SIZE])
    }

    pub(crate) fn max() -> DirectoryId {
        DirectoryId([u8::MAX; SIZE])
    }

    fn as_hex(&self) -> String {
        let mut target = String::with_capacity(2 * SIZE);

        for b in self.0.iter() {
            target.push(HEX[(*b as usize >> 4) & 0x0f]);
            target.push(HEX[(*b as usize) & 0x0f]);
        }

        target
    }

    pub(crate) fn to_pathbuf<P: AsRef<Path> + ?Sized>(&self, parent: &P) -> PathBuf {
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

impl fmt::Display for DirectoryId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_hex())
    }
}

impl FromStr for DirectoryId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 2 * SIZE {
            return Err(Error::InvalidId(s.to_string()));
        }

        let mut id = DirectoryId::min();
        let mut iter = s.chars();

        for n in id.0.iter_mut() {
            let b1 = iter
                .next()
                .unwrap()
                .to_digit(16)
                .map_or_else(|| Err(Error::InvalidId(s.to_string())), |n| Ok(n as u8))?;
            let b2 = iter
                .next()
                .unwrap()
                .to_digit(16)
                .map_or_else(|| Err(Error::InvalidId(s.to_string())), |n| Ok(n as u8))?;

            *n = (b1 << 4) | b2;
        }

        Ok(id)
    }
}

impl BlockId for DirectoryId {
    fn null() -> DirectoryId {
        DirectoryId::max()
    }

    fn is_null(&self) -> bool {
        self.0 == DirectoryId::max().0
    }

    fn size() -> usize {
        SIZE
    }
}

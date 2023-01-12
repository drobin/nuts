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

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, mem, result};
use uuid::Uuid;

use nuts_backend::BlockId;
use nuts_bytes::{FromBytes, ToBytes};

use crate::directory::DirectoryError;

#[derive(Clone, Debug, PartialEq)]
pub struct DirectoryId(Uuid);

impl DirectoryId {
    pub(crate) fn generate() -> DirectoryId {
        DirectoryId(Uuid::new_v4())
    }

    pub(crate) fn header() -> DirectoryId {
        DirectoryId(Uuid::nil())
    }

    pub(crate) fn to_pathbuf<P: AsRef<Path>>(&self, parent: &P) -> PathBuf {
        let mut buf = Uuid::encode_buffer();
        let uuid = self.0.simple().encode_lower(&mut buf);

        let mut path = PathBuf::new();
        let mut pos = 0;

        path.push(parent);

        for _ in 0..2 {
            path.push(&uuid[pos..pos + 2]);
            pos += 2;
        }

        path.push(&uuid[pos..]);

        path
    }
}

impl fmt::Display for DirectoryId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl FromStr for DirectoryId {
    type Err = DirectoryError;

    fn from_str(s: &str) -> result::Result<Self, <Self as FromStr>::Err> {
        <Uuid as FromStr>::from_str(s).map_or_else(
            |cause| Err(DirectoryError::InvalidId(cause)),
            |uuid| Ok(DirectoryId(uuid)),
        )
    }
}

impl FromBytes for DirectoryId {
    fn from_bytes<R: Read>(source: &mut R) -> result::Result<Self, nuts_bytes::Error> {
        const SIZE: usize = std::mem::size_of::<u128>();
        let mut bytes = [0; SIZE];

        source.read_exact(&mut bytes)?;

        let n = u128::from_be_bytes(bytes);
        let uuid = Uuid::from_u128(n);

        Ok(DirectoryId(uuid))
    }
}

impl ToBytes for DirectoryId {
    fn to_bytes<W: Write>(&self, target: &mut W) -> result::Result<(), nuts_bytes::Error> {
        let bytes = self.0.as_u128().to_be_bytes();
        Ok(target.write_all(&bytes)?)
    }
}

impl BlockId for DirectoryId {
    fn null() -> DirectoryId {
        DirectoryId(Uuid::from_u128(u128::MAX))
    }

    fn is_null(&self) -> bool {
        self.0.as_u128() == u128::MAX
    }

    fn size() -> usize {
        mem::size_of::<u128>()
    }
}

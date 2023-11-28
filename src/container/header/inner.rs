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

#[cfg(test)]
mod tests;

use nuts_bytes::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::container::header::rev0;
use crate::container::header::HeaderMagicError;

const MAGIC: [u8; 7] = *b"nuts-io";

#[derive(Debug, Deserialize, FromBytes, Serialize, ToBytes)]
#[serde(try_from = "[u8; 7]")]
#[from_bytes(validate)]
struct Magic([u8; 7]);

impl Magic {
    fn validate(&self) -> Result<(), HeaderMagicError> {
        if self.0 == MAGIC {
            Ok(())
        } else {
            Err(HeaderMagicError)
        }
    }
}

impl<T: AsRef<[u8]>> PartialEq<T> for Magic {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

impl TryFrom<[u8; 7]> for Magic {
    type Error = String;

    fn try_from(buf: [u8; 7]) -> Result<Self, String> {
        if buf == MAGIC {
            Ok(Magic(buf))
        } else {
            Err("invalid magic".to_string())
        }
    }
}

#[derive(Debug, Deserialize, FromBytes, Serialize, ToBytes)]
pub enum Revision {
    Rev0(rev0::Data),
}

#[derive(Debug, Deserialize, FromBytes, Serialize, ToBytes)]
pub struct Inner {
    magic: Magic,
    pub rev: Revision,
}

impl Inner {
    pub fn new(rev: Revision) -> Inner {
        Inner {
            magic: Magic(MAGIC),
            rev,
        }
    }
}

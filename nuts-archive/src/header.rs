// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use chrono::{DateTime, Utc};
use nuts_backend::Backend;
use nuts_bytes::{FromBytes, ToBytes};
use thiserror::Error;

use crate::magic::{validate_magic, Magic, MagicErrorFactory, MAGIC};
use crate::{datetime, ArchiveResult, Error};

const CURRENT_REVISION: u16 = 2;
const UNSUPPORTED_REVISIONS: [(u16, &str); 1] = [(1, "0.4.3")];

#[derive(Debug, Error)]
#[error("invalid header")]
pub struct HeaderMagicError;

impl MagicErrorFactory for HeaderMagicError {
    fn create() -> Self {
        HeaderMagicError
    }
}

#[derive(Debug, FromBytes, ToBytes)]
pub struct Header {
    #[nuts_bytes(map_from_bytes = validate_magic::<HeaderMagicError>)]
    magic: Magic,
    revision: u16,
    #[nuts_bytes(map = datetime)]
    pub created: DateTime<Utc>,
    #[nuts_bytes(map = datetime)]
    pub modified: DateTime<Utc>,
    pub nfiles: u64,
}

impl Header {
    pub fn create() -> Header {
        let now = Utc::now();

        Header {
            magic: MAGIC,
            revision: CURRENT_REVISION,
            created: now,
            modified: now,
            nfiles: 0,
        }
    }

    pub fn validate_revision<B: Backend>(&self) -> ArchiveResult<(), B> {
        for (rev, version) in UNSUPPORTED_REVISIONS {
            if self.revision == rev {
                return Err(Error::UnsupportedRevision(rev, version.to_string()));
            }
        }

        Ok(())
    }

    pub fn inc_files(&mut self) {
        self.nfiles += 1;
        self.modified = Utc::now();
    }
}

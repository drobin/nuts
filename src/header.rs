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

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::mem;

use crate::magic::magic_type;

magic_type!(Magic, "invalid header-magic", size);

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    magic: Magic,
    revision: u16,
    #[serde(with = "ts_milliseconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub modified: DateTime<Utc>,
    pub nfiles: u64,
}

impl Header {
    pub fn size() -> usize {
        let magic = Magic::size();
        let revision = mem::size_of::<u16>();
        let tstamps = 2 * mem::size_of::<i64>();
        let nfiles = mem::size_of::<u64>();

        magic + revision + tstamps + nfiles
    }

    pub fn create() -> Header {
        let now = Utc::now();

        Header {
            magic: Magic::new(),
            revision: 1,
            created: now,
            modified: now,
            nfiles: 0,
        }
    }

    pub fn inc_files(&mut self) {
        self.nfiles += 1;
        self.modified = Utc::now();
    }
}

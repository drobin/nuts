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

use chrono::prelude::*;
use chrono::serde::ts_milliseconds;
use serde::{Deserialize, Serialize};

use crate::now;

const MAGIC: [u8; 12] = *b"nuts-archive";

#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "[u8; 12]")]
struct Magic([u8; 12]);

impl TryFrom<[u8; 12]> for Magic {
    type Error = String;

    fn try_from(buf: [u8; 12]) -> Result<Self, String> {
        if buf == MAGIC {
            Ok(Magic(buf))
        } else {
            Err("invalid magic".to_string())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Header {
    magic: Magic,
    pub revision: u8,
    #[serde(with = "ts_milliseconds")]
    pub ctime: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub mtime: DateTime<Utc>,
    pub count: u64,
    pub size: u64,
}

impl Header {
    pub fn create() -> Header {
        let dt = now();

        Header {
            magic: Magic(MAGIC),
            revision: 1,
            ctime: dt,
            mtime: dt,
            count: 0,
            size: 0,
        }
    }
}

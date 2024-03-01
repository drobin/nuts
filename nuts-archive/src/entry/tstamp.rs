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

use chrono::{DateTime, Utc};
use nuts_bytes::{FromBytes, ToBytes};

use crate::datetime;

#[derive(Debug, FromBytes, ToBytes)]
pub struct Timestamps {
    #[nuts_bytes(map = datetime)]
    appended: DateTime<Utc>,

    #[nuts_bytes(map = datetime)]
    created: DateTime<Utc>,

    #[nuts_bytes(map = datetime)]
    changed: DateTime<Utc>,

    #[nuts_bytes(map = datetime)]
    modified: DateTime<Utc>,
}

impl Timestamps {
    pub fn new() -> Timestamps {
        let now = Utc::now();

        Timestamps {
            appended: now,
            created: now,
            changed: now,
            modified: now,
        }
    }

    pub fn appended(&self) -> &DateTime<Utc> {
        &self.appended
    }

    pub fn created(&self) -> &DateTime<Utc> {
        &self.created
    }

    pub fn set_created(&mut self, created: DateTime<Utc>) {
        self.created = created
    }

    pub fn changed(&self) -> &DateTime<Utc> {
        &self.changed
    }

    pub fn set_changed(&mut self, changed: DateTime<Utc>) {
        self.changed = changed
    }

    pub fn modified(&self) -> &DateTime<Utc> {
        &self.modified
    }

    pub fn set_modified(&mut self, modified: DateTime<Utc>) {
        self.modified = modified
    }
}

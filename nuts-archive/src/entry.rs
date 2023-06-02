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
use std::borrow::Cow;

use crate::mode::{Group, Mode, Type};
#[cfg(doc)]
use crate::Archive;

/// An entry of the archive.
///
/// This type identifies an entry of the archive. The [`Archive::scan()`]
/// method returns an iterator with `Entry` instances.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Entry<'a> {
    pub(crate) mode: Mode,
    pub(crate) size: u64,
    #[serde(with = "ts_milliseconds")]
    pub(crate) actime: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub(crate) ctime: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub(crate) mtime: DateTime<Utc>,
    pub(crate) name: Cow<'a, str>,
}

impl<'a> Entry<'a> {
    /// Returns the name of the entry.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the type of this entry.
    ///
    /// It can be either a [file](Type::File), a [directory](Type::Directory)
    /// or a [symbolic link](Type::Symlink).
    pub fn ftype(&self) -> Type {
        self.mode.into()
    }

    /// Returns the size of the entry.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Tests whether this entry is readable for the given `group`.
    pub fn is_readable(&self, group: Group) -> bool {
        self.mode.is_readable(group)
    }

    /// Tests whether this entry is writable for the given `group`.
    pub fn is_writable(&self, group: Group) -> bool {
        self.mode.is_writable(group)
    }

    /// Tests whether this entry is executable for the given `group`.
    pub fn is_executable(&self, group: Group) -> bool {
        self.mode.is_executable(group)
    }

    /// Returns the _archive creation time_ of this entry.
    ///
    /// This is the time when the entry was appended to the archive.
    pub fn actime(&self) -> DateTime<Utc> {
        self.ctime
    }

    /// Returns the _creation time_ of this entry.
    ///
    /// This is the time when the entry originally was created. E.g. when the
    /// entry originally comes from a filesystem entry, the ctime of the
    /// filesystem entry can be stored here.
    pub fn ctime(&self) -> DateTime<Utc> {
        self.ctime
    }

    /// Returns the _modification time_ of this entry.
    ///
    /// This is the time when the entry originally was modified. E.g. when the
    /// entry originally comes from a filesystem entry, the mtime of the
    /// filesystem entry can be stored here.
    pub fn mtime(&self) -> DateTime<Utc> {
        self.mtime
    }
}

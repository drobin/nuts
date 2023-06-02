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

//! A storage application inspired by the `tar` tool.
//!
//! The archive is an application based on the nuts container. Inspired by the `tar`
//! tool you can store files, directories and symlinks in a nuts container.

mod entry;
mod error;
mod header;
mod iter;
mod mode;
mod rc;

use chrono::prelude::*;
use log::{debug, log_enabled, Level::Debug};
use nuts::container::Container;
use nuts::stream::{OpenOptions, Position, Stream};
use nuts_backend::Backend;
use nuts_bytes::Options;
use serde::{Deserialize, Serialize};

use crate::header::Header;
use crate::rc::StreamRc;

pub use entry::Entry;
pub use error::{Error, Result};
pub use iter::{Iter, IterEntry};
pub use mode::{Group, Type};

fn now() -> DateTime<Utc> {
    if cfg!(test) {
        Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap()
    } else {
        Utc::now()
    }
}

/// Information/statistics from an archive.
///
/// An instance of the `Info` type is returned by [`Archive::info()`] and
/// collects some information/statistics from the archive.
///
/// [`Archive::info()`]: struct.Archive.html#method.info
#[derive(Debug)]
pub struct Info {
    /// Time when the archive was created.
    pub ctime: DateTime<Utc>,

    /// Time when the archive was modified the last time.
    pub mtime: DateTime<Utc>,

    /// Number of entries in the archive.
    pub count: u64,

    /// Total number of bytes stored in the archive.
    pub size: u64,
}

pub struct Archive<B: Backend> {
    header: Header,
    stream: StreamRc<B>,
}

impl<B: 'static + Backend> Archive<B> {
    /// Creates a new archive in the given `container`.
    pub fn create(container: Container<B>) -> Result<Archive<B>, B> {
        let id = container.top_id().cloned().ok_or(Error::NoTopId)?;
        let stream = Stream::create(container, id)?;
        let mut rc = StreamRc::new(stream);

        let header = Header::create();
        let mut writer = Options::new().build_writer(&mut rc);

        header.serialize(&mut writer)?;
        debug!("archive created, {:?}", header);

        Ok(Archive { header, stream: rc })
    }

    /// Opens an existing archive from the given `container`.
    pub fn open(container: Container<B>) -> Result<Archive<B>, B> {
        let id = container.top_id().cloned().ok_or(Error::NoTopId)?;
        let stream = OpenOptions::new()
            .read(true)
            .write(true)
            .open(container, id)?;
        let mut rc = StreamRc::new(stream);

        let mut reader = Options::new().build_reader(&mut rc);
        let header = Header::deserialize(&mut reader)?;

        debug!("archive opened, {:?}", header);

        Ok(Archive { header, stream: rc })
    }

    /// Returns some information about the archive.
    ///
    /// See the [`Info`] type for more information.
    pub fn info(&self) -> Info {
        Info {
            ctime: self.header.ctime,
            mtime: self.header.mtime,
            count: self.header.count,
            size: self.header.size,
        }
    }

    /// Scans the archive for entries.
    ///
    /// Returns an [`Iterator`], so you can iterate over all entries of the
    /// archive.
    pub fn scan(&mut self) -> Iter<B> {
        Iter::new(StreamRc::clone(&self.stream))
    }
}

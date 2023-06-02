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

mod builder;
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
use std::cmp;

use crate::builder::EntryBuilder;
use crate::header::Header;
use crate::rc::StreamRc;

pub use builder::{Builder, DirectoryBuilder, FileBuilder, PathBuilder};
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

    /// Appends an entry at the end of the archive.
    ///
    /// As an argument the method expects a type that implements the
    /// [`Builder`] trait. A builder describes the new entry to be appended to
    /// the archive. The crate provides various builders:
    ///
    /// * [`FileBuilder`]: Describes a file in the archive.
    /// * [`DirectoryBuilder`]: Describes a directory in the archive.
    /// * [`PathBuilder`]: Appends an entry from an exsting filesystem entry.
    pub fn add<T: Builder>(&mut self, builder: T) -> Result<(), B> {
        let mut ebuilder = EntryBuilder::new(builder);
        let now = now();

        self.stream.borrow_mut().seek(Position::End(0))?;

        let max_bytes = ebuilder.size_hint() as usize;
        let entry_bytes = self.write_entry(&ebuilder, now, None)?;
        let content_bytes = self.write_content(&mut ebuilder, max_bytes)?;

        if ebuilder.size_hint() != content_bytes {
            let n = -1 * (entry_bytes + content_bytes) as i64;

            debug!("go back for {} bytes", n);
            self.stream.borrow_mut().seek(Position::Current(n))?;

            self.write_entry(&ebuilder, now, Some(content_bytes))?;
        }

        self.touch_header(content_bytes)
    }

    fn write_entry<T: Builder>(
        &mut self,
        builder: &EntryBuilder<T>,
        actime: DateTime<Utc>,
        size: Option<u64>,
    ) -> Result<u64, B> {
        let mut writer = Options::new().build_writer(&mut self.stream);
        let entry = builder.to_entry(actime, size);

        let n = entry.serialize(&mut writer)?;

        if log_enabled!(Debug) {
            if let Some(n) = size {
                debug!("rewrite entry {:?}, ({} bytes written)", entry, n);
            } else {
                debug!("append entry {:?}, ({} bytes written)", entry, n);
            }
        }

        Ok(n as u64)
    }

    fn write_content<T: Builder>(
        &mut self,
        builder: &mut EntryBuilder<T>,
        max_bytes: usize,
    ) -> Result<u64, B> {
        let mut num_bytes = 0;
        let mut buf = [0; 1024];

        loop {
            let remaining = max_bytes - num_bytes;
            let len = cmp::min(buf.len(), remaining);
            let n = builder.read(&mut buf[..len])?;

            if n > 0 {
                self.stream.borrow_mut().write_all(&buf[..n])?;

                num_bytes += n;
            } else {
                debug!("num bytes: {}", num_bytes);
                break;
            }
        }

        Ok(num_bytes as u64)
    }

    fn touch_header(&mut self, nbytes: u64) -> Result<(), B> {
        self.header.mtime = now();
        self.header.count += 1;
        self.header.size += nbytes;

        self.stream.borrow_mut().seek(Position::Start(0))?;

        let mut writer = Options::new().build_writer(&mut self.stream);
        self.header.serialize(&mut writer)?;

        Ok(())
    }

    /// Scans the archive for entries.
    ///
    /// Returns an [`Iterator`], so you can iterate over all entries of the
    /// archive.
    pub fn scan(&mut self) -> Iter<B> {
        Iter::new(StreamRc::clone(&self.stream))
    }
}

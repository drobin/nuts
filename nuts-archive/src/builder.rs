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
use nuts::backend::Backend;
#[cfg(doc)]
use nuts::stream::Stream;
use std::borrow::Cow;
use std::fs::{self, File, Metadata};
use std::io::{self, Cursor, Read};
#[cfg(target_family = "unix")]
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::{error, result};

use crate::entry::Entry;
use crate::error::{Error, Result};
#[cfg(target_family = "unix")]
use crate::mode::unix;
use crate::mode::{Group, Mode, Type};
#[cfg(doc)]
use crate::Archive;

macro_rules! to_dt_or_now {
    ($r:expr) => {
        $r.map_or_else(|_| Utc::now(), |st| st.into())
    };
}

/// Trait used to append a new entry to the archive.
///
/// A type which implements `Builder` is passed to [`Archive::add()`] and
/// describes the entry of the archive. Later, the [`Entry`] type is used to
/// fetch existing entries from the archive.
pub trait Builder {
    /// Returns the type of the new entry.
    fn ftype(&self) -> Type;

    /// Returns the name of the new entry.
    fn name(&self) -> &str;

    /// Returns an optional size hint for the new entry.
    ///
    /// With a size hint you can give the archive an hint, how big the new
    /// entry will be. No more that `sizehint` bytes are read with
    /// [`Self::read()`]. Passing the correct size to the size hint will make
    /// the append much more efficient because no
    /// [further seeking](`Stream::seek`) is necessary.
    ///
    /// A size hint for directory entries makes no sense because a directory
    /// has always zero size. So you can return [`None`] for this case.
    fn size_hint(&self) -> Option<u64>;

    /// Tests whether the new entry should be readable for the given `group`.
    fn is_readable(&self, group: Group) -> bool;

    /// Tests whether the new entry should be writable for the given `group`.
    fn is_writable(&self, group: Group) -> bool;

    /// Tests whether the new entry should be executable for the given `group`.
    fn is_executable(&self, group: Group) -> bool;

    /// Returns the creation time of the new entry.
    fn ctime(&self) -> DateTime<Utc>;

    /// Returns the Modification time of the new entry.
    fn mtime(&self) -> DateTime<Utc>;

    /// Reads some data for new entry.
    ///
    /// * Should return the number of bytes read.
    /// * In case of no more data are available, `Ok(0)` must be returned.
    ///
    /// # Errors
    ///
    /// An arbritary boxed error must be retured in case of an error. Later,
    /// the error is mapped into [`Error::Builder`].
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> result::Result<usize, Box<dyn error::Error + Send + Sync>>;
}

pub struct EntryBuilder<T>(T);

impl<T: Builder> EntryBuilder<T> {
    pub fn new(t: T) -> EntryBuilder<T> {
        EntryBuilder(t)
    }

    pub fn size_hint(&self) -> u64 {
        self.0.size_hint().unwrap_or(u64::MAX)
    }

    pub fn to_entry<'a>(&'a self, actime: DateTime<Utc>, size: Option<u64>) -> Entry<'a> {
        Entry {
            mode: self.make_mode(),
            size: size.unwrap_or(self.size_hint()),
            actime,
            ctime: self.0.ctime(),
            mtime: self.0.mtime(),
            name: Cow::Borrowed(self.0.name()),
        }
    }

    fn make_mode(&self) -> Mode {
        let mut mode = Mode::from_ftype(self.0.ftype());

        for g in [Group::User, Group::Group, Group::Other] {
            mode.set_readable(g, self.0.is_readable(g));
            mode.set_writable(g, self.0.is_writable(g));
            mode.set_executable(g, self.0.is_executable(g));
        }

        mode
    }

    pub fn read<B: Backend>(&mut self, buf: &mut [u8]) -> Result<usize, B> {
        self.0.read(buf).map_err(|err| Error::Builder(err))
    }
}

/// Builder that creates a file entry in the archive.
///
/// Use the setter methods (`set_*`) to modify the new entry.
pub struct FileBuilder<R> {
    name: String,
    size_hint: Option<u64>,
    mode: Mode,
    ctime: DateTime<Utc>,
    mtime: DateTime<Utc>,
    content: Option<R>,
}

impl<R> FileBuilder<R> {
    /// Creates a new `FileBuilder` instance.
    ///
    /// The file will have the given `name`. All other attributes will have
    /// some suitable default values:
    ///
    /// * No size hint.
    /// * creation- & modification time are set to now.
    /// * The file has no content.
    pub fn new(name: String) -> FileBuilder<R> {
        let now = Utc::now();

        FileBuilder {
            name,
            size_hint: None,
            mode: Mode::file(),
            ctime: now,
            mtime: now,
            content: None,
        }
    }

    /// Assigns a size hint to the builder.
    pub fn set_size_hint(mut self, size_hint: u64) -> Self {
        self.size_hint = Some(size_hint);
        self
    }

    /// Assigns a readable flag for the given group to the builder.
    pub fn set_readable(mut self, group: Group, value: bool) -> Self {
        self.mode.set_readable(group, value);
        self
    }

    /// Assigns a writable flag for the given group to the builder.
    pub fn set_writable(mut self, group: Group, value: bool) -> Self {
        self.mode.set_writable(group, value);
        self
    }

    /// Assigns an executable flag for the given group to the builder.
    pub fn set_executable(mut self, group: Group, value: bool) -> Self {
        self.mode.set_executable(group, value);
        self
    }

    /// Assigns a creation time to the builder.
    pub fn set_ctime(mut self, ctime: DateTime<Utc>) -> Self {
        self.ctime = ctime;
        self
    }

    /// Assigns a modification time to the builder.
    pub fn set_mtime(mut self, mtime: DateTime<Utc>) -> Self {
        self.mtime = mtime;
        self
    }

    /// Assigns a content reader to the builder.
    ///
    /// When the file enzty is created, the file content is taken from this
    /// [`Read`] instance.
    pub fn set_content(mut self, r: R) -> Self {
        self.content = Some(r);
        self
    }
}

impl<R: Read> Builder for FileBuilder<R> {
    fn ftype(&self) -> Type {
        Type::File
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size_hint(&self) -> Option<u64> {
        self.size_hint
    }

    fn is_readable(&self, group: Group) -> bool {
        self.mode.is_readable(group)
    }

    fn is_writable(&self, group: Group) -> bool {
        self.mode.is_writable(group)
    }

    fn is_executable(&self, group: Group) -> bool {
        self.mode.is_executable(group)
    }

    fn ctime(&self) -> DateTime<Utc> {
        self.ctime
    }

    fn mtime(&self) -> DateTime<Utc> {
        self.mtime
    }

    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> result::Result<usize, Box<dyn error::Error + Send + Sync>> {
        let n = match &mut self.content {
            Some(r) => r.read(buf)?,
            None => 0,
        };

        Ok(n)
    }
}

/// Builder that creates a directory entry in the archive.
///
/// Use the setter methods (`set_*`) to modify the new entry.
pub struct DirectoryBuilder {
    name: String,
    mode: Mode,
    ctime: DateTime<Utc>,
    mtime: DateTime<Utc>,
}

impl DirectoryBuilder {
    /// Creates a new `DirectoryBuilder` instance.
    ///
    /// The directory will have the given `name`. All other attributes will have
    /// some suitable default values:
    ///
    /// * creation- & modification time are set to now.
    pub fn new(name: String) -> DirectoryBuilder {
        let now = Utc::now();

        DirectoryBuilder {
            name,
            mode: Mode::directory(),
            ctime: now,
            mtime: now,
        }
    }

    /// Assigns a readable flag for the given group to the builder.
    pub fn set_readable(mut self, group: Group, value: bool) -> Self {
        self.mode.set_readable(group, value);
        self
    }

    /// Assigns a writable flag for the given group to the builder.
    pub fn set_writable(mut self, group: Group, value: bool) -> Self {
        self.mode.set_writable(group, value);
        self
    }

    /// Assigns an executable flag for the given group to the builder.
    pub fn set_executable(mut self, group: Group, value: bool) -> Self {
        self.mode.set_executable(group, value);
        self
    }

    /// Assigns a creation time to the builder.
    pub fn set_ctime(mut self, ctime: DateTime<Utc>) -> Self {
        self.ctime = ctime;
        self
    }

    /// Assigns a modification time to the builder.
    pub fn set_mtime(mut self, mtime: DateTime<Utc>) -> Self {
        self.mtime = mtime;
        self
    }
}

impl Builder for DirectoryBuilder {
    fn ftype(&self) -> Type {
        Type::Directory
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size_hint(&self) -> Option<u64> {
        None
    }

    fn is_readable(&self, group: Group) -> bool {
        self.mode.is_readable(group)
    }

    fn is_writable(&self, group: Group) -> bool {
        self.mode.is_writable(group)
    }

    fn is_executable(&self, group: Group) -> bool {
        self.mode.is_executable(group)
    }

    fn ctime(&self) -> DateTime<Utc> {
        self.ctime
    }

    fn mtime(&self) -> DateTime<Utc> {
        self.mtime
    }

    fn read(
        &mut self,
        _buf: &mut [u8],
    ) -> result::Result<usize, Box<dyn error::Error + Send + Sync>> {
        Ok(0)
    }
}

enum PathContent {
    File(File),
    Directory,
    Symlink(Cursor<Vec<u8>>),
}

/// Builder that creates an archive entry from a filesystem entry ([`Path`]).
///
/// Use [`PathBuilder::resolve()`] to create a `PathBuilder` instance from a
/// [`Path`]. The filesystem entry is copied into the archive.
pub struct PathBuilder<'a> {
    name: Cow<'a, str>,
    md: Metadata,
    content: PathContent,
}

impl<'a> PathBuilder<'a> {
    /// Creates a `PathBuilder` instance from the given [`Path`].
    pub fn resolve(path: &'a Path) -> io::Result<PathBuilder<'a>> {
        let md = fs::symlink_metadata(&path)?;

        let content = if md.is_file() {
            let fh = File::open(&path)?;
            PathContent::File(fh)
        } else if md.is_dir() {
            PathContent::Directory
        } else if md.is_symlink() {
            let target = fs::read_link(&path)?;
            let bytes = target.to_string_lossy().as_bytes().to_vec();
            PathContent::Symlink(Cursor::new(bytes))
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid file type"));
        };

        Ok(PathBuilder {
            name: path.to_string_lossy(),
            md,
            content,
        })
    }
}

impl<'a> Builder for PathBuilder<'a> {
    fn ftype(&self) -> Type {
        match self.content {
            PathContent::File(_) => Type::File,
            PathContent::Directory => Type::Directory,
            PathContent::Symlink(_) => Type::Symlink,
        }
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn size_hint(&self) -> Option<u64> {
        Some(self.md.len())
    }

    fn is_readable(&self, group: Group) -> bool {
        if cfg!(target_family = "unix") {
            let mode = self.md.permissions().mode();
            let flag = match group {
                Group::User => unix::S_IRUSR,
                Group::Group => unix::S_IRGRP,
                Group::Other => unix::S_IROTH,
            };

            mode & flag > 0
        } else {
            unimplemented!()
        }
    }

    fn is_writable(&self, group: Group) -> bool {
        if cfg!(target_family = "unix") {
            let mode = self.md.permissions().mode();
            let flag = match group {
                Group::User => unix::S_IWUSR,
                Group::Group => unix::S_IWGRP,
                Group::Other => unix::S_IWOTH,
            };

            mode & flag > 0
        } else {
            unimplemented!()
        }
    }

    fn is_executable(&self, group: Group) -> bool {
        if cfg!(target_family = "unix") {
            let mode = self.md.permissions().mode();
            let flag = match group {
                Group::User => unix::S_IXUSR,
                Group::Group => unix::S_IXGRP,
                Group::Other => unix::S_IXOTH,
            };

            mode & flag > 0
        } else {
            unimplemented!()
        }
    }

    fn ctime(&self) -> DateTime<Utc> {
        to_dt_or_now!(self.md.created())
    }

    fn mtime(&self) -> DateTime<Utc> {
        to_dt_or_now!(self.md.modified())
    }

    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> result::Result<usize, Box<dyn error::Error + Send + Sync>> {
        match &mut self.content {
            PathContent::File(fh) => Ok(fh.read(buf)?),
            PathContent::Directory => Ok(0),
            PathContent::Symlink(cursor) => Ok(cursor.read(buf)?),
        }
    }
}

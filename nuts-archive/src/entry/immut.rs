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

use log::{debug, error, warn};
use nuts_backend::Backend;
use std::cmp;
use std::convert::{TryFrom, TryInto};

use crate::entry::{populate_mode_api, populate_tstamp_api, Inner};
use crate::error::{ArchiveResult, Error};
use crate::pager::Pager;
use crate::tree::Tree;

/// An entry of the archive.
///
/// An instance of `Entry` represents an entry of the archive.
///
/// You can traverse through the archive using the [`Entry::next()`] method.
/// The first entry of the archive is returned by
/// [`Archive::first()`](crate::Archive::first).
pub enum Entry<'a, B: Backend> {
    /// The entry represents a file.
    File(FileEntry<'a, B>),

    /// The entry represents a directory.
    Directory(DirectoryEntry<'a, B>),

    /// The entry represents a symlink.
    Symlink(SymlinkEntry<'a, B>),
}

impl<'a, B: Backend> Entry<'a, B> {
    /// Returns the next entry in the archive.
    ///
    /// If this is the last entry [`None`] is returned, which means that there
    /// are no further entries available.
    pub fn next(self) -> Option<ArchiveResult<Entry<'a, B>, B>> {
        match self.into_inner_entry().next() {
            Some(Ok(entry)) => Some(entry.try_into()),
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }

    /// Returns the name of the entry.
    pub fn name(&self) -> &str {
        &self.inner_entry().inner.name
    }

    /// Returns the size of the entry.
    pub fn size(&self) -> u64 {
        self.inner_entry().inner.size
    }

    populate_mode_api!();
    populate_tstamp_api!();

    /// Tests whether this entry represents a file.
    pub fn is_file(&self) -> bool {
        match self {
            Self::File(_) => true,
            Self::Directory(_) => false,
            Self::Symlink(_) => false,
        }
    }

    /// Returns a reference to the inner [`FileEntry`] instance if this entry
    /// represents a file.
    ///
    /// If this entry is a file then a reference to the inner [`FileEntry`]
    /// wrapped into a [`Some`] is returned. If this is not a file [`None`] is
    /// returned.
    pub fn as_file(&self) -> Option<&FileEntry<'a, B>> {
        match self {
            Self::File(value) => Some(value),
            Self::Directory(_) => None,
            Self::Symlink(_) => None,
        }
    }

    /// Returns the inner [`FileEntry`] instance if this entry represents a
    /// file.
    ///
    /// If this entry is a file then the inner [`FileEntry`] wrapped into a
    /// [`Some`] is returned. If this is not a file [`None`] is returned.
    pub fn into_file(self) -> Option<FileEntry<'a, B>> {
        match self {
            Self::File(value) => Some(value),
            Self::Directory(_) => None,
            Self::Symlink(_) => None,
        }
    }

    /// Tests whether this entry represents a directory.
    pub fn is_directory(&self) -> bool {
        match self {
            Self::File(_) => false,
            Self::Directory(_) => true,
            Self::Symlink(_) => false,
        }
    }

    /// Returns a reference to the inner [`DirectoryEntry`] instance if this
    /// entry represents a directory.
    ///
    /// If this entry is a directory then a reference to the inner
    /// [`DirectoryEntry`] wrapped into a [`Some`] is returned. If this is not
    /// a directory [`None`] is returned.
    pub fn as_directory(&self) -> Option<&DirectoryEntry<'a, B>> {
        match self {
            Self::File(_) => None,
            Self::Directory(value) => Some(value),
            Self::Symlink(_) => None,
        }
    }

    /// Returns the inner [`DirectoryEntry`] instance if this entry represents
    /// a directory.
    ///
    /// If this entry is a directory then the inner [`DirectoryEntry`] wrapped
    /// into a [`Some`] is returned. If this is not a directory [`None`] is
    /// returned.
    pub fn into_directory(self) -> Option<DirectoryEntry<'a, B>> {
        match self {
            Self::File(_) => None,
            Self::Directory(value) => Some(value),
            Self::Symlink(_) => None,
        }
    }

    /// Tests whether this entry represents a symlink.
    pub fn is_symlink(&self) -> bool {
        match self {
            Self::File(_) => false,
            Self::Directory(_) => false,
            Self::Symlink(_) => true,
        }
    }

    /// Returns a reference to the inner [`SymlinkEntry`] instance if this
    /// entry represents a symlink.
    ///
    /// If this entry is a symlink then a reference to the inner
    /// [`SymlinkEntry`] wrapped into a [`Some`] is returned. If this is not a
    /// symlink [`None`] is returned.
    pub fn as_symlink(&self) -> Option<&SymlinkEntry<'a, B>> {
        match self {
            Self::File(_) => None,
            Self::Directory(_) => None,
            Self::Symlink(value) => Some(value),
        }
    }

    /// Returns the inner [`SymlinkEntry`] instance if this entry represents a
    /// symlink.
    ///
    /// If this entry is a symlink then the inner [`SymlinkEntry`] wrapped into
    /// a [`Some`] is returned. If this is not a symlink [`None`] is returned.
    pub fn into_symlink(self) -> Option<SymlinkEntry<'a, B>> {
        match self {
            Self::File(_) => None,
            Self::Directory(_) => None,
            Self::Symlink(value) => Some(value),
        }
    }

    fn inner_entry(&'a self) -> &InnerEntry<'a, B> {
        match self {
            Self::File(inner) => &inner.0,
            Self::Directory(inner) => &inner.0,
            Self::Symlink(inner) => &inner.shared,
        }
    }

    fn into_inner_entry(self) -> InnerEntry<'a, B> {
        match self {
            Self::File(inner) => inner.0,
            Self::Directory(inner) => inner.0,
            Self::Symlink(inner) => inner.shared,
        }
    }

    fn inner(&self) -> &Inner {
        &self.inner_entry().inner
    }
}

impl<'a, B: Backend> TryFrom<InnerEntry<'a, B>> for Entry<'a, B> {
    type Error = Error<B>;

    fn try_from(src: InnerEntry<'a, B>) -> ArchiveResult<Self, B> {
        if src.inner.mode.is_file() {
            Ok(Self::File(FileEntry(src)))
        } else if src.inner.mode.is_directory() {
            Ok(Self::Directory(DirectoryEntry(src)))
        } else if src.inner.mode.is_symlink() {
            Ok(Self::Symlink(SymlinkEntry::new(src)?))
        } else {
            error!(
                "could not detect entry type at {} from mode {:?}",
                src.idx, src.inner.mode
            );

            match src.tree.lookup(src.pager, src.idx) {
                Some(Ok(id)) => Err(Error::InvalidType(Some(id.clone()))),
                Some(Err(err)) => {
                    error!("id lookup failed for idx {}", src.idx);
                    Err(err)
                }
                None => {
                    warn!("no id found at idx {}", src.idx);
                    Err(Error::InvalidType(None))
                }
            }
        }
    }
}

/// A file entry of the archive.
///
/// An instance of this type is attached to the [`Entry::File`] variant and
/// provides file specific options.
///
/// One of the `read*` methods can be used to get the content of the file.
pub struct FileEntry<'a, B: Backend>(InnerEntry<'a, B>);

impl<'a, B: Backend> FileEntry<'a, B> {
    /// Returns the name of the file.
    pub fn name(&self) -> &str {
        &self.0.inner.name
    }

    /// Returns the size of the file.
    pub fn size(&self) -> u64 {
        self.0.inner.size
    }

    populate_mode_api!();
    populate_tstamp_api!();

    /// Reads data from the entry.
    ///
    /// Reads up to [`buf.len()`] bytes and puts them into `buf`.
    ///
    /// The methods returns the number of bytes actually read, which cannot be
    /// greater than the [`buf.len()`].
    ///
    /// [`buf.len()`]: slice::len
    pub fn read(&mut self, buf: &mut [u8]) -> ArchiveResult<usize, B> {
        self.0.read(buf)
    }

    /// Read the exact number of bytes required to fill `buf`
    ///
    /// This function reads as many bytes as necessary to completely fill the
    /// specified buffer `buf`.
    ///
    /// # Errors
    ///
    /// If this function encounters an "end of file" before completely filling
    /// the buffer, it returns an [`Error::UnexpectedEof`] error. The contents
    /// of `buf` are unspecified in this case.
    pub fn read_all(&mut self, mut buf: &mut [u8]) -> ArchiveResult<(), B> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty() {
            Err(Error::UnexpectedEof)
        } else {
            Ok(())
        }
    }

    /// Reads all bytes until EOF and collects them into a [`Vec`] which is
    /// returned.
    pub fn read_vec(&mut self) -> ArchiveResult<Vec<u8>, B> {
        let mut vec = vec![0; self.0.inner.size as usize];
        self.read_all(&mut vec).map(|()| vec)
    }

    fn inner(&self) -> &Inner {
        &self.0.inner
    }
}

/// A directory entry of the archive.
///
/// An instance of this type is attached to the [`Entry::Directory`] variant
/// and provides directory specific options.
pub struct DirectoryEntry<'a, B: Backend>(InnerEntry<'a, B>);

impl<'a, B: Backend> DirectoryEntry<'a, B> {
    /// Returns the name of the directory.
    pub fn name(&self) -> &str {
        &self.0.inner.name
    }

    populate_mode_api!();
    populate_tstamp_api!();

    fn inner(&self) -> &Inner {
        &self.0.inner
    }
}

/// A symlink entry of the archive.
///
/// An instance of this type is attached to the [`Entry::Symlink`] variant and
/// provides symlink specific options.
pub struct SymlinkEntry<'a, B: Backend> {
    shared: InnerEntry<'a, B>,
    target: String,
}

impl<'a, B: Backend> SymlinkEntry<'a, B> {
    fn new(mut shared: InnerEntry<'a, B>) -> ArchiveResult<SymlinkEntry<'a, B>, B> {
        let target = Self::read_target(&mut shared)?;

        Ok(SymlinkEntry { shared, target })
    }

    /// Returns the name of the symlink.
    pub fn name(&self) -> &str {
        &self.shared.inner.name
    }

    /// Returns the target of the symlink.
    ///
    /// This is the path, where the symlink points to.
    pub fn target(&self) -> &str {
        &self.target
    }

    populate_mode_api!();
    populate_tstamp_api!();

    fn read_target(shared: &mut InnerEntry<'a, B>) -> ArchiveResult<String, B> {
        const CHUNK: usize = 64;
        let mut vec = vec![];
        let mut nbytes = 0;

        loop {
            vec.resize(vec.len() + CHUNK, 0);

            let n = shared.read(&mut vec[nbytes..nbytes + CHUNK])?;
            nbytes += n;

            vec.resize(nbytes, 0);

            if n == 0 {
                break;
            }
        }

        Ok(String::from_utf8_lossy(&vec).to_string())
    }

    fn inner(&self) -> &Inner {
        &self.shared.inner
    }
}

pub struct InnerEntry<'a, B: Backend> {
    pager: &'a mut Pager<B>,
    tree: &'a mut Tree<B>,
    inner: Inner,
    idx: usize,
    rcache: Vec<u8>,
    ridx: usize,
}

impl<'a, B: Backend> InnerEntry<'a, B> {
    pub fn load(
        pager: &'a mut Pager<B>,
        tree: &'a mut Tree<B>,
        idx: usize,
        id: &B::Id,
    ) -> ArchiveResult<InnerEntry<'a, B>, B> {
        let inner = Inner::load(pager, id)?;

        Ok(InnerEntry {
            pager,
            tree,
            inner,
            idx,
            rcache: vec![],
            ridx: 0,
        })
    }

    pub fn first(
        pager: &'a mut Pager<B>,
        tree: &'a mut Tree<B>,
    ) -> Option<ArchiveResult<InnerEntry<'a, B>, B>> {
        match tree.lookup(pager, 0) {
            Some(Ok(id)) => {
                debug!("lookup first at {}: {}", 0, id);
                let id = id.clone();
                Some(Self::load(pager, tree, 0, &id))
            }
            Some(Err(err)) => {
                error!("lookup first at {}: {}", 0, err);
                Some(Err(err))
            }
            None => {
                debug!("lookup first at {}: none", 0);
                None
            }
        }
    }

    fn next(self) -> Option<ArchiveResult<InnerEntry<'a, B>, B>> {
        let content_blocks = self.content_blocks() as usize;
        let next_idx = self.idx + content_blocks + 1;

        debug!(
            "next_idx={} (idx={}, size={}, content_blocks={})",
            next_idx, self.idx, self.inner.size, content_blocks
        );

        match self.tree.lookup(self.pager, next_idx) {
            Some(Ok(id)) => {
                debug!("lookup next at {}: {}", next_idx, id);

                let id = id.clone();
                Some(Self::load(self.pager, self.tree, next_idx, &id))
            }
            Some(Err(err)) => {
                error!("lookup next at {}: {}", next_idx, err);
                Some(Err(err))
            }
            None => {
                debug!("lookup next at {}: none", next_idx);
                None
            }
        }
    }

    fn read(&mut self, buf: &mut [u8]) -> ArchiveResult<usize, B> {
        if self.rcache.is_empty() {
            let blocks = self.content_blocks();

            debug!("fill cache: idx={}, blocks={}", self.ridx, blocks);

            if self.ridx >= blocks as usize {
                return Ok(0);
            }

            let block_size = self.pager.block_size() as usize;
            let remaining = self.inner.size as usize - self.ridx * block_size;
            let cache_size = cmp::min(remaining, block_size);

            debug!(
                "fill cache: remaining={}, cache_size={}",
                remaining, cache_size
            );

            self.rcache.resize(cache_size, 0);

            let idx = self.idx + self.ridx + 1;

            match self.tree.lookup(self.pager, idx) {
                Some(Ok(id)) => {
                    let n = self.pager.read(id, self.rcache.as_mut_slice())?;

                    assert_eq!(n, cache_size);

                    self.ridx += 1;
                }
                Some(Err(err)) => return Err(err),
                None => {
                    warn!("premature end of archive, no block at {}", idx);
                    return Ok(0);
                }
            };
        }

        let len = cmp::min(self.rcache.len(), buf.len());

        self.rcache
            .drain(..len)
            .enumerate()
            .for_each(|(i, n)| buf[i] = n);

        Ok(len)
    }

    fn content_blocks(&self) -> u64 {
        let block_size = self.pager.block_size() as u64;

        if self.inner.size % block_size == 0 {
            self.inner.size / block_size
        } else {
            self.inner.size / block_size + 1
        }
    }
}

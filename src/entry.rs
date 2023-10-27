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

use log::{debug, error, warn};
use nuts_bytes::Writer;
use nuts_container::backend::Backend;
use serde::{Deserialize, Serialize};
use std::cmp;

use crate::container::BufContainer;
use crate::error::{ArchiveResult, Error};
use crate::flush_header;
use crate::header::Header;
use crate::tree::Tree;

#[derive(Debug, Deserialize, Serialize)]
struct Inner {
    name: String,
    size: u64,
}

impl Inner {
    fn new(name: String) -> Inner {
        Inner { name, size: 0 }
    }

    fn load<B: Backend>(container: &mut BufContainer<B>, id: &B::Id) -> ArchiveResult<Inner, B> {
        let mut reader = container.read_buf(id)?;
        let inner = reader.deserialize()?;

        Ok(inner)
    }

    fn flush<B: Backend>(
        &self,
        container: &mut BufContainer<B>,
        id: &B::Id,
    ) -> ArchiveResult<(), B> {
        let buf = {
            let mut writer = Writer::new(vec![]);

            writer.serialize(self)?;

            writer.into_target()
        };

        container.write(id, &buf)?;

        Ok(())
    }
}

/// An entry of the archive.
///
/// An instance of `Entry` represents an entry of the archive.
///
/// You can traverse through the archive using the [`Entry::next()`] method.
/// The first entry of the archive is returned by
/// [`Archive::first()`](crate::Archive::first).
pub struct Entry<'a, B: Backend> {
    container: &'a mut BufContainer<B>,
    tree: &'a mut Tree<B>,
    inner: Inner,
    idx: usize,
    rcache: Vec<u8>,
    ridx: usize,
}

impl<'a, B: Backend> Entry<'a, B> {
    fn load(
        container: &'a mut BufContainer<B>,
        tree: &'a mut Tree<B>,
        idx: usize,
        id: &B::Id,
    ) -> ArchiveResult<Entry<'a, B>, B> {
        let inner = Inner::load(container, id)?;

        Ok(Entry {
            container,
            tree,
            inner,
            idx,
            rcache: vec![],
            ridx: 0,
        })
    }

    pub(crate) fn first(
        container: &'a mut BufContainer<B>,
        tree: &'a mut Tree<B>,
    ) -> Option<ArchiveResult<Entry<'a, B>, B>> {
        match tree.lookup(container, 0) {
            Some(Ok(id)) => {
                debug!("lookup first at {}: {}", 0, id);
                let id = id.clone();
                Some(Self::load(container, tree, 0, &id))
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

    /// Returns the next entry in the archive.
    ///
    /// If this is the last entry [`None`] is returned, which means that there
    /// are no further entries available.
    pub fn next(self) -> Option<ArchiveResult<Entry<'a, B>, B>> {
        let content_blocks = self.content_blocks() as usize;
        let next_idx = self.idx + content_blocks + 1;

        debug!(
            "next_idx={} (idx={}, size={}, content_blocks={})",
            next_idx,
            self.idx,
            self.size(),
            content_blocks
        );

        match self.tree.lookup(self.container, next_idx) {
            Some(Ok(id)) => {
                debug!("lookup next at {}: {}", next_idx, id);

                let id = id.clone();
                Some(Self::load(self.container, self.tree, next_idx, &id))
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

    /// Returns the name of the entry.
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// Returns the size of the entry.
    pub fn size(&self) -> u64 {
        self.inner.size
    }

    /// Reads data from the entry.
    ///
    /// Reads up to [`buf.len()`] bytes and puts them into `buf`.
    ///
    /// The methods returns the number of bytes actually read, which cannot be
    /// greater than the [`buf.len()`].
    ///
    /// [`buf.len()`]: slice::len
    pub fn read(&mut self, buf: &mut [u8]) -> ArchiveResult<usize, B> {
        if self.rcache.is_empty() {
            let blocks = self.content_blocks();

            debug!("fill cache: idx={}, blocks={}", self.ridx, blocks);

            if self.ridx >= blocks as usize {
                return Ok(0);
            }

            let block_size = self.container.block_size() as usize;
            let remaining = self.size() as usize - self.ridx * block_size;
            let cache_size = cmp::min(remaining, block_size);

            debug!(
                "fill cache: remaining={}, cache_size={}",
                remaining, cache_size
            );

            self.rcache.resize(cache_size, 0);

            let idx = self.idx + self.ridx + 1;

            match self.tree.lookup(self.container, idx) {
                Some(Ok(id)) => {
                    let n = self.container.read(id, self.rcache.as_mut_slice())?;

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
        let mut vec = vec![0; self.size() as usize];
        self.read_all(&mut vec).map(|()| vec)
    }

    fn content_blocks(&self) -> u64 {
        let block_size = self.container.block_size() as u64;

        if self.size() % block_size == 0 {
            self.size() / block_size
        } else {
            self.size() / block_size + 1
        }
    }
}

/// Builder for an new entry.
///
/// An `EntryBuilder` instance is returned by
/// [`Archive::append()`](crate::Archive::append). Calling
/// [`EntryBuilder::build()`] will create the entry at the end of the archive.
pub struct EntryBuilder<'a, B: Backend> {
    container: &'a mut BufContainer<B>,
    header_id: &'a B::Id,
    header: &'a mut Header,
    tree: &'a mut Tree<B>,
    entry: Inner,
}

impl<'a, B: Backend> EntryBuilder<'a, B> {
    pub(crate) fn new(
        container: &'a mut BufContainer<B>,
        header_id: &'a B::Id,
        header: &'a mut Header,
        tree: &'a mut Tree<B>,
        name: String,
    ) -> EntryBuilder<'a, B> {
        EntryBuilder {
            container,
            header_id,
            header,
            tree,
            entry: Inner::new(name),
        }
    }

    /// Finally, creates the new entry at the end of the archive.
    ///
    /// It returns an [`EntryMut`] instance, where you are able to add content
    /// to the entry.
    pub fn build(self) -> ArchiveResult<EntryMut<'a, B>, B> {
        let id = self.tree.aquire(self.container)?.clone();

        self.entry.flush(self.container, &id)?;

        self.header.inc_files();
        flush_header(self.container, self.header_id, self.header, self.tree)?;

        Ok(EntryMut::new(
            self.container,
            self.header_id,
            self.header,
            self.tree,
            self.entry,
            id,
        ))
    }
}

/// A mutable entry of the archive.
///
/// An `EntryMut` instance is returned by [`EntryBuilder::build()`] and gives
/// you the possibility to add content to the entry.
pub struct EntryMut<'a, B: Backend> {
    container: &'a mut BufContainer<B>,
    header_id: &'a B::Id,
    header: &'a mut Header,
    tree: &'a mut Tree<B>,
    entry: Inner,
    first: B::Id,
    last: B::Id,
    cache: Vec<u8>,
}

impl<'a, B: Backend> EntryMut<'a, B> {
    fn new(
        container: &'a mut BufContainer<B>,
        header_id: &'a B::Id,
        header: &'a mut Header,
        tree: &'a mut Tree<B>,
        entry: Inner,
        id: B::Id,
    ) -> EntryMut<'a, B> {
        EntryMut {
            container,
            header_id,
            header,
            tree,
            entry,
            first: id.clone(),
            last: id,
            cache: vec![],
        }
    }

    /// Appends some content from `buf` at the end of the entry.
    ///
    /// Note that the entire buffer is not necessarily written. The method
    /// returns the number of bytes that were actually written.
    pub fn write(&mut self, buf: &[u8]) -> ArchiveResult<usize, B> {
        let block_size = self.container.block_size() as u64;
        let pos = (self.entry.size % block_size) as usize;

        let available = if pos == 0 {
            self.last = self.tree.aquire(self.container)?.clone();

            debug!("block aquired: {}", self.last);

            self.cache.clear();
            self.cache.resize(block_size as usize, 0);

            block_size as usize
        } else {
            assert_eq!(self.cache.len(), block_size as usize);

            block_size as usize - pos
        };

        let nbytes = cmp::min(buf.len(), available as usize);

        debug!(
            "bsize={}, pos={}, available={}, nbytes={}",
            block_size, pos, available, nbytes
        );

        self.cache[pos..pos + nbytes].copy_from_slice(&buf[..nbytes]);
        self.container.write(&self.last, &self.cache)?;

        self.entry.size += nbytes as u64;
        self.entry.flush(self.container, &self.first)?;
        flush_header(self.container, self.header_id, self.header, self.tree)?;

        Ok(nbytes)
    }

    pub fn write_all(&mut self, mut buf: &[u8]) -> ArchiveResult<(), B> {
        while !buf.is_empty() {
            let n = self.write(buf)?;

            buf = &buf[n..]
        }

        Ok(())
    }
}

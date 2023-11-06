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

use log::debug;
use nuts_container::backend::Backend;
use std::cmp;

use crate::entry::mode::Mode;
use crate::entry::{populate_mode_api, Inner};
use crate::error::ArchiveResult;
use crate::flush_header;
use crate::header::Header;
use crate::pager::Pager;
use crate::tree::Tree;

macro_rules! impl_new {
    ($type:ident, $mode:ident) => {
        pub(crate) fn new(
            pager: &'a mut Pager<B>,
            header_id: &'a B::Id,
            header: &'a mut Header,
            tree: &'a mut Tree<B>,
            name: String,
        ) -> $type<'a, B> {
            $type(InnerBuilder::new(
                pager,
                header_id,
                header,
                tree,
                name,
                Mode::$mode(),
            ))
        }
    };
}

/// Builder for an new file entry.
///
/// A `FileBuilder` instance is returned by
/// [`Archive::append_file()`](crate::Archive::append_file). Calling
/// [`FileBuilder::build()`] will create the entry at the end of the archive.
pub struct FileBuilder<'a, B: Backend>(InnerBuilder<'a, B>);

impl<'a, B: Backend> FileBuilder<'a, B> {
    impl_new!(FileBuilder, file);

    populate_mode_api!(mut);

    /// Finally, creates the new file entry at the end of the archive.
    ///
    /// It returns an [`EntryMut`] instance, where you are able to add content
    /// to the entry.
    pub fn build(self) -> ArchiveResult<EntryMut<'a, B>, B> {
        self.0.build()
    }

    fn inner(&self) -> &Inner {
        &self.0.entry
    }

    fn inner_mut(&mut self) -> &mut Inner {
        &mut self.0.entry
    }
}

/// Builder for an new directory entry.
///
/// A `DirectoryBuilder` instance is returned by
/// [`Archive::append_directory()`](crate::Archive::append_directory). Calling
/// [`DirectoryBuilder::build()`] will create the entry at the end of the
/// archive.
pub struct DirectoryBuilder<'a, B: Backend>(InnerBuilder<'a, B>);

impl<'a, B: Backend> DirectoryBuilder<'a, B> {
    impl_new!(DirectoryBuilder, directory);

    populate_mode_api!(mut);

    /// Finally, creates the new directory entry at the end of the archive.
    pub fn build(self) -> ArchiveResult<(), B> {
        self.0.build().map(|_| ())
    }

    fn inner(&self) -> &Inner {
        &self.0.entry
    }

    fn inner_mut(&mut self) -> &mut Inner {
        &mut self.0.entry
    }
}

/// Builder for an new symlink entry.
///
/// A `SymlinkBuilder` instance is returned by
/// [`Archive::append_symlink()`](crate::Archive::append_symlink). Calling
/// [`SymlinkBuilder::build()`] will create the entry at the end of the
/// archive.
pub struct SymlinkBuilder<'a, B: Backend> {
    builder: InnerBuilder<'a, B>,
    target: String,
}

impl<'a, B: Backend> SymlinkBuilder<'a, B> {
    pub(crate) fn new(
        pager: &'a mut Pager<B>,
        header_id: &'a B::Id,
        header: &'a mut Header,
        tree: &'a mut Tree<B>,
        name: String,
        target: String,
    ) -> SymlinkBuilder<'a, B> {
        let builder = InnerBuilder::new(pager, header_id, header, tree, name, Mode::symlink());

        SymlinkBuilder { builder, target }
    }

    populate_mode_api!(mut);

    /// Finally, creates the new symlink entry at the end of the archive.
    pub fn build(self) -> ArchiveResult<(), B> {
        let mut entry = self.builder.build()?;

        entry.write_all(self.target.as_bytes())?;

        Ok(())
    }

    fn inner(&self) -> &Inner {
        &self.builder.entry
    }

    fn inner_mut(&mut self) -> &mut Inner {
        &mut self.builder.entry
    }
}

struct InnerBuilder<'a, B: Backend> {
    pager: &'a mut Pager<B>,
    header_id: &'a B::Id,
    header: &'a mut Header,
    tree: &'a mut Tree<B>,
    entry: Inner,
}

impl<'a, B: Backend> InnerBuilder<'a, B> {
    fn new(
        pager: &'a mut Pager<B>,
        header_id: &'a B::Id,
        header: &'a mut Header,
        tree: &'a mut Tree<B>,
        name: String,
        mode: Mode,
    ) -> InnerBuilder<'a, B> {
        InnerBuilder {
            pager,
            header_id,
            header,
            tree,
            entry: Inner::new(name, mode),
        }
    }

    fn build(self) -> ArchiveResult<EntryMut<'a, B>, B> {
        let id = self.tree.aquire(self.pager)?.clone();

        self.entry.flush(self.pager, &id)?;

        self.header.inc_files();
        flush_header(self.pager, self.header_id, self.header, self.tree)?;

        Ok(EntryMut::new(
            self.pager,
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
/// An `EntryMut` instance is returned by [`FileBuilder::build()`] and gives
/// you the possibility to add content to the entry.
pub struct EntryMut<'a, B: Backend> {
    pager: &'a mut Pager<B>,
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
        pager: &'a mut Pager<B>,
        header_id: &'a B::Id,
        header: &'a mut Header,
        tree: &'a mut Tree<B>,
        entry: Inner,
        id: B::Id,
    ) -> EntryMut<'a, B> {
        EntryMut {
            pager,
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
        let block_size = self.pager.block_size() as u64;
        let pos = (self.entry.size % block_size) as usize;

        let available = if pos == 0 {
            self.last = self.tree.aquire(self.pager)?.clone();

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
        self.pager.write(&self.last, &self.cache)?;

        self.entry.size += nbytes as u64;
        self.entry.flush(self.pager, &self.first)?;
        flush_header(self.pager, self.header_id, self.header, self.tree)?;

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

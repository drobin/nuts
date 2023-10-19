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

use nuts_bytes::Writer;
use nuts_container::backend::Backend;
use serde::{Deserialize, Serialize};
use std::cmp;

use crate::container::BufContainer;
use crate::tree::Tree;
use crate::ArchiveResult;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Entry {
    name: String,
    size: u64,
}

impl Entry {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> u64 {
        self.size
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

/// Builder for an new entry.
///
/// An `EntryBuilder` instance is returned by
/// [`Archive::append()`](crate::Archive::append). Calling
/// [`EntryBuilder::build()`] will create the entry at the end of the archive.
pub struct EntryBuilder<'a, B: Backend> {
    container: &'a mut BufContainer<B>,
    tree_id: &'a B::Id,
    tree: &'a mut Tree<B>,
    entry: Entry,
}

impl<'a, B: Backend> EntryBuilder<'a, B> {
    pub(crate) fn new(
        container: &'a mut BufContainer<B>,
        tree_id: &'a B::Id,
        tree: &'a mut Tree<B>,
        name: String,
    ) -> EntryBuilder<'a, B> {
        let entry = Entry { name, size: 0 };

        EntryBuilder {
            container,
            tree_id,
            tree,
            entry,
        }
    }

    /// Finally, creates the new entry at the end of the archive.
    ///
    /// It returns an [`EntryMut`] instance, where you are able to add content
    /// to the entry.
    pub fn build(self) -> ArchiveResult<EntryMut<'a, B>, B> {
        let id = self.tree.aquire(self.container)?.clone();

        self.entry.flush(self.container, &id)?;
        self.tree.flush(self.container, self.tree_id)?;

        Ok(EntryMut::new(self.entry, id, self.container, self.tree))
    }
}

/// A mutable entry of the archive.
///
/// An `EntryMut` instance is returned by [`EntryBuilder::build()`] and gives
/// you the possibility to add content to the entry.
pub struct EntryMut<'a, B: Backend> {
    container: &'a mut BufContainer<B>,
    tree: &'a mut Tree<B>,
    cache: Vec<u8>,
    entry: Entry,
    first: B::Id,
    last: B::Id,
}

impl<'a, B: Backend> EntryMut<'a, B> {
    fn new(
        entry: Entry,
        id: B::Id,
        container: &'a mut BufContainer<B>,
        tree: &'a mut Tree<B>,
    ) -> EntryMut<'a, B> {
        EntryMut {
            container,
            tree,
            cache: vec![],
            entry,
            first: id.clone(),
            last: id,
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

            self.cache.clear();
            self.cache.resize(block_size as usize, 0);

            block_size as usize
        } else {
            assert_eq!(self.cache.len(), block_size as usize);

            block_size as usize - pos
        };

        let nbytes = cmp::min(buf.len(), available as usize);

        self.cache[pos..pos + nbytes].copy_from_slice(&buf[..nbytes]);
        self.container.write(&self.last, &self.cache)?;

        self.entry.size += nbytes as u64;
        self.entry.flush(self.container, &self.first)?;

        Ok(nbytes)
    }
}

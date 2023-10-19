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

mod cache;
mod node;
#[cfg(test)]
mod tests;

use nuts_container::backend::{Backend, BlockId};
use nuts_container::container::Container;
use std::mem;

use crate::container::BufContainer;
use crate::error::{ArchiveResult, Error};
use crate::tree::cache::Cache;
use crate::tree::node::Node;

fn ids_per_node<B: Backend>(container: &Container<B>) -> usize {
    container.block_size() as usize / B::Id::size()
}

const NUM_DIRECT: usize = 12;

#[derive(Debug)]
pub struct Tree<B: Backend> {
    direct: Vec<B::Id>,
    indirect: B::Id,
    d_indirect: B::Id,
    t_indirect: B::Id,
    nblocks: usize,
    cache: Vec<Cache<B>>,
}

impl<B: Backend> Tree<B> {
    pub fn new() -> Tree<B> {
        Tree {
            direct: vec![B::Id::null(); NUM_DIRECT],
            indirect: B::Id::null(),
            d_indirect: B::Id::null(),
            t_indirect: B::Id::null(),
            nblocks: 0,
            cache: vec![],
        }
    }

    pub fn load(container: &mut BufContainer<B>, id: &B::Id) -> ArchiveResult<Tree<B>, B> {
        if (container.block_size() as usize) < (15 * B::Id::size() + mem::size_of::<u64>()) {
            return Err(Error::InvalidBlockSize);
        }

        let mut reader = container.read_buf(id)?;
        let mut tree = Self::new();

        for i in 0..NUM_DIRECT {
            tree.direct[i] = reader.deserialize()?;
        }

        tree.indirect = reader.deserialize()?;
        tree.d_indirect = reader.deserialize()?;
        tree.t_indirect = reader.deserialize()?;
        tree.nblocks = reader.deserialize()?;

        Ok(tree)
    }

    pub fn flush(&self, container: &mut BufContainer<B>, id: &B::Id) -> ArchiveResult<(), B> {
        if (container.block_size() as usize) < (15 * B::Id::size() + mem::size_of::<u64>()) {
            return Err(Error::InvalidBlockSize);
        }

        let mut writer = container.create_writer();

        for id in self.direct.iter() {
            writer.serialize(id)?;
        }

        writer.serialize(&self.indirect)?;
        writer.serialize(&self.d_indirect)?;
        writer.serialize(&self.t_indirect)?;
        writer.serialize(&self.nblocks)?;

        container.write_buf(id)?;

        Ok(())
    }

    pub fn aquire(&mut self, container: &mut BufContainer<B>) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container); // ids per node

        if self.nblocks < NUM_DIRECT + ipn + ipn * ipn + ipn * ipn * ipn {
            self.lookup_cache(container, self.nblocks, true)
        } else {
            Err(Error::Full)
        }
    }

    pub fn lookup(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
    ) -> Option<ArchiveResult<&B::Id, B>> {
        let ipn = ids_per_node(container); // ids per node

        if idx < NUM_DIRECT + ipn + ipn * ipn + ipn * ipn * ipn {
            match self.lookup_cache(container, idx, false) {
                Ok(id) => {
                    if id.is_null() {
                        None
                    } else {
                        Some(Ok(id))
                    }
                }
                Err(err) => Some(Err(err)),
            }
        } else {
            None
        }
    }

    fn lookup_cache(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container); // ids per node

        if idx < NUM_DIRECT {
            self.lookup_direct(container, idx, aquire)
        } else if idx < NUM_DIRECT + ipn {
            self.lookup_indirect(container, idx - NUM_DIRECT, aquire)
        } else if idx < NUM_DIRECT + ipn + ipn * ipn {
            self.lookup_d_indirect(container, idx - NUM_DIRECT - ipn, aquire)
        } else {
            self.lookup_t_indirect(container, idx - NUM_DIRECT - ipn - ipn * ipn, aquire)
        }
    }

    fn lookup_direct(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        if self.direct[idx].is_null() && aquire {
            self.direct[idx] = container.aquire()?;
            self.nblocks += 1;
        }

        Ok(&self.direct[idx])
    }

    fn lookup_indirect(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        if self.indirect.is_null() {
            self.indirect = Node::aquire(container)?;
        }

        self.cache.resize_with(1, || Cache::new(container));

        self.cache[0].refresh(container, &self.indirect)?;

        if aquire && self.cache[0].aquire(container, idx, true)? {
            self.nblocks += 1;
        }

        Ok(&self.cache[0][idx])
    }

    fn lookup_d_indirect(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container); // ids per node

        if self.d_indirect.is_null() {
            self.d_indirect = Node::aquire(container)?;
        }

        self.cache.resize_with(2, || Cache::new(container));

        let idx = ((idx / ipn) % ipn, idx % ipn);

        // level 0

        self.cache[0].refresh(container, &self.d_indirect)?;

        if aquire {
            self.cache[0].aquire(container, idx.0, false)?;
        } else if self.cache[0][idx.0].is_null() {
            return Ok(&self.cache[0][idx.0]);
        }

        // level 1

        let id = self.cache[0][idx.0].clone();
        self.cache[1].refresh(container, &id)?;

        if aquire && self.cache[1].aquire(container, idx.1, true)? {
            self.nblocks += 1;
        }

        Ok(&self.cache[1][idx.1])
    }

    fn lookup_t_indirect(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container); // ids per node

        if self.t_indirect.is_null() {
            self.t_indirect = Node::aquire(container)?;
        }

        self.cache.resize_with(3, || Cache::new(container));

        let idx = ((idx / (ipn * ipn)) % ipn, (idx / ipn) % ipn, idx % ipn);

        // level 0

        self.cache[0].refresh(container, &self.t_indirect)?;

        if aquire {
            self.cache[0].aquire(container, idx.0, false)?;
        } else if self.cache[0][idx.0].is_null() {
            return Ok(&self.cache[0][idx.0]);
        }

        // level 1

        let id = self.cache[0][idx.0].clone();
        self.cache[1].refresh(container, &id)?;

        if aquire {
            self.cache[1].aquire(container, idx.1, false)?;
        } else if self.cache[1][idx.1].is_null() {
            return Ok(&self.cache[1][idx.1]);
        }

        // level 2

        let id = self.cache[1][idx.1].clone();
        self.cache[2].refresh(container, &id)?;

        if aquire && self.cache[2].aquire(container, idx.2, true)? {
            self.nblocks += 1;
        }

        Ok(&self.cache[2][idx.2])
    }
}

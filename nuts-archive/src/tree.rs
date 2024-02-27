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

mod cache;
#[cfg(test)]
mod tests;

use log::debug;
use nuts_backend::{Backend, BlockId};
use nuts_bytes::{FromBytes, ToBytes};
use nuts_container::Container;
use std::mem;

use crate::error::{ArchiveResult, Error};
use crate::pager::Pager;
use crate::tree::cache::Cache;

fn ids_per_node<B: Backend>(container: &Container<B>) -> u32 {
    (container.block_size() - mem::size_of::<u32>() as u32) / B::Id::size() as u32
}

const NUM_DIRECT: u32 = 12;
const IDX_INDIRECT: usize = NUM_DIRECT as usize;
const IDX_D_INDIRECT: usize = IDX_INDIRECT + 1;
const IDX_T_INDIRECT: usize = IDX_D_INDIRECT + 1;

#[derive(Debug, FromBytes, ToBytes)]
pub struct Tree<B: Backend> {
    ids: Vec<B::Id>,
    nblocks: u64,
    #[nuts_bytes(skip)]
    cache: Cache<B>,
}

impl<B: Backend> Tree<B> {
    pub fn size() -> usize {
        let id_size = B::Id::size() as usize;

        let direct = NUM_DIRECT as usize * id_size;
        let indirect = 3 * id_size;
        let nblocks = mem::size_of::<u64>();

        direct + indirect + nblocks
    }

    pub fn new() -> Tree<B> {
        Tree {
            ids: vec![],
            nblocks: 0,
            cache: Cache::new(),
        }
    }

    pub fn nblocks(&self) -> u64 {
        self.nblocks
    }

    pub fn aquire(&mut self, pager: &mut Pager<B>) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(pager) as u64; // ids per node

        if self.nblocks < NUM_DIRECT as u64 {
            self.aquire_direct(pager)
        } else if self.nblocks < NUM_DIRECT as u64 + ipn {
            self.aquire_indirect(pager)
        } else if self.nblocks < NUM_DIRECT as u64 + ipn + ipn * ipn {
            self.aquire_d_indirect(pager)
        } else if self.nblocks < NUM_DIRECT as u64 + ipn + ipn * ipn + ipn * ipn * ipn {
            self.aquire_t_indirect(pager)
        } else {
            Err(Error::Full)
        }
    }

    pub fn lookup(&mut self, pager: &mut Pager<B>, idx: usize) -> Option<ArchiveResult<&B::Id, B>> {
        if idx >= self.nblocks as usize {
            return None;
        }

        let ipn = ids_per_node(pager) as usize; // ids per node

        let result = if idx < NUM_DIRECT as usize {
            self.lookup_direct(idx)
        } else if idx < NUM_DIRECT as usize + ipn {
            self.lookup_indirect(pager, idx - NUM_DIRECT as usize)
        } else if idx < NUM_DIRECT as usize + ipn + ipn * ipn {
            self.lookup_d_indirect(pager, idx - NUM_DIRECT as usize - ipn)
        } else {
            self.lookup_t_indirect(pager, idx - NUM_DIRECT as usize - ipn - ipn * ipn)
        };

        match result {
            Ok(Some(id)) => Some(Ok(id)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }

    fn lookup_direct(&mut self, idx: usize) -> ArchiveResult<Option<&B::Id>, B> {
        assert!(idx < NUM_DIRECT as usize);

        let id = self.ids.get(idx);

        debug!(
            "lookup_direct: idx={}, nblocks={}, id={:?}",
            idx, self.nblocks, id
        );

        Ok(id)
    }

    fn aquire_direct(&mut self, pager: &mut Pager<B>) -> ArchiveResult<&B::Id, B> {
        assert!(self.nblocks < NUM_DIRECT as u64);

        self.ids.push(pager.aquire()?);
        self.nblocks += 1;

        let id = &self.ids[self.nblocks as usize - 1];

        debug!("aquire_direct: nblocks={} => {}", self.nblocks, id);

        Ok(id)
    }

    fn lookup_indirect(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
    ) -> ArchiveResult<Option<&B::Id>, B> {
        let id = self
            .cache
            .resolve(pager, self.ids.get(IDX_INDIRECT), &[idx])?;

        debug!(
            "loopup_indirect: idx={}, nblocks={}, {:?}",
            idx, self.nblocks, id
        );

        Ok(id)
    }

    fn aquire_indirect(&mut self, pager: &mut Pager<B>) -> ArchiveResult<&B::Id, B> {
        while self.ids.get(IDX_INDIRECT).is_none() {
            self.ids.push(pager.aquire()?);
        }

        let idx = self.nblocks as usize - NUM_DIRECT as usize;
        let id = self.cache.aquire(pager, &self.ids[IDX_INDIRECT], &[idx])?;

        self.nblocks += 1;

        debug!(
            "aquire_indirect: idx={}, nblocks={} => {}",
            idx, self.nblocks, id
        );

        Ok(id)
    }

    fn lookup_d_indirect(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
    ) -> ArchiveResult<Option<&B::Id>, B> {
        let ipn = ids_per_node(pager) as usize; // ids per node
        let d_idx = [(idx / ipn) % ipn, idx % ipn];
        let d_indirect = self.ids.get(IDX_D_INDIRECT);

        let id = self.cache.resolve(pager, d_indirect, &d_idx)?;

        debug!(
            "loopup_d_indirect: idx={} => {:?}, nblocks={} => {:?}",
            idx, d_idx, self.nblocks, id
        );

        Ok(id)
    }

    fn aquire_d_indirect(&mut self, pager: &mut Pager<B>) -> ArchiveResult<&B::Id, B> {
        while self.ids.get(IDX_D_INDIRECT).is_none() {
            self.ids.push(pager.aquire()?);
        }

        let ipn = ids_per_node(pager) as usize; // ids per node
        let idx = self.nblocks as usize - NUM_DIRECT as usize - ipn;

        let d_idx = [(idx / ipn) % ipn, idx % ipn];
        let d_indirect = &self.ids[IDX_D_INDIRECT];

        let id = self.cache.aquire(pager, d_indirect, &d_idx)?;

        self.nblocks += 1;

        debug!(
            "aquire_d_indirect: idx={} => {:?}, nblocks={} => {}",
            idx, d_idx, self.nblocks, id
        );

        Ok(id)
    }

    fn lookup_t_indirect(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
    ) -> ArchiveResult<Option<&B::Id>, B> {
        let ipn = ids_per_node(pager) as usize; // ids per node
        let t_idx = [(idx / (ipn * ipn)) % ipn, (idx / ipn) % ipn, idx % ipn];
        let t_indirect = self.ids.get(IDX_T_INDIRECT);

        let id = self.cache.resolve(pager, t_indirect, &t_idx)?;

        debug!(
            "loopup_t_indirect: idx={} => {:?}, nblocks={} => {:?}",
            idx, t_idx, self.nblocks, id
        );

        Ok(id)
    }

    fn aquire_t_indirect(&mut self, pager: &mut Pager<B>) -> ArchiveResult<&B::Id, B> {
        while self.ids.get(IDX_T_INDIRECT).is_none() {
            self.ids.push(pager.aquire()?);
        }

        let ipn = ids_per_node(pager) as usize; // ids per node
        let idx = self.nblocks as usize - NUM_DIRECT as usize - ipn - ipn * ipn;

        let t_idx = [(idx / (ipn * ipn)) % ipn, (idx / ipn) % ipn, idx % ipn];
        let t_indirect = &self.ids[IDX_T_INDIRECT];

        let id = self.cache.aquire(pager, &t_indirect, &t_idx)?;

        self.nblocks += 1;

        debug!(
            "aquire_t_indirect: idx={} => {:?}, nblocks={} => {}",
            idx, t_idx, self.nblocks, id
        );

        Ok(id)
    }
}

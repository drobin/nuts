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
mod node;
#[cfg(test)]
mod tests;

use log::{debug, warn};
use nuts_backend::{Backend, BlockId};
use nuts_bytes::{FromBytes, ToBytes};
use nuts_container::container::Container;
use std::mem;

use crate::error::{ArchiveResult, Error};
use crate::pager::Pager;
use crate::tree::cache::Cache;
use crate::tree::node::Node;

fn ids_per_node<B: Backend>(container: &Container<B>) -> u32 {
    container.block_size() / B::Id::size() as u32
}

const NUM_DIRECT: u32 = 12;

fn make_cache<B: Backend>() -> Vec<Cache<B>> {
    vec![]
}

#[derive(Debug, FromBytes, ToBytes)]
pub struct Tree<B: Backend> {
    direct: [B::Id; NUM_DIRECT as usize],
    indirect: B::Id,
    d_indirect: B::Id,
    t_indirect: B::Id,
    nblocks: u64,
    #[nuts_bytes(skip, default = make_cache)]
    cache: Vec<Cache<B>>,
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
            direct: [
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
                B::Id::null(),
            ],
            indirect: B::Id::null(),
            d_indirect: B::Id::null(),
            t_indirect: B::Id::null(),
            nblocks: 0,
            cache: vec![],
        }
    }

    pub fn nblocks(&self) -> u64 {
        self.nblocks
    }

    pub fn aquire(&mut self, pager: &mut Pager<B>) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(pager) as u64; // ids per node

        if self.nblocks < NUM_DIRECT as u64 + ipn + ipn * ipn + ipn * ipn * ipn {
            self.lookup_cache(pager, self.nblocks as usize, true)
        } else {
            Err(Error::Full)
        }
    }

    pub fn lookup(&mut self, pager: &mut Pager<B>, idx: usize) -> Option<ArchiveResult<&B::Id, B>> {
        if idx < self.nblocks as usize {
            match self.lookup_cache(pager, idx, false) {
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
        pager: &mut Pager<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(pager) as usize; // ids per node

        if idx < NUM_DIRECT as usize {
            self.lookup_direct(pager, idx, aquire)
        } else if idx < NUM_DIRECT as usize + ipn {
            self.lookup_indirect(pager, idx - NUM_DIRECT as usize, aquire)
        } else if idx < NUM_DIRECT as usize + ipn + ipn * ipn {
            self.lookup_d_indirect(pager, idx - NUM_DIRECT as usize - ipn, aquire)
        } else {
            self.lookup_t_indirect(pager, idx - NUM_DIRECT as usize - ipn - ipn * ipn, aquire)
        }
    }

    fn lookup_direct(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        if aquire {
            if self.direct[idx].is_null() {
                self.direct[idx] = pager.aquire()?;
                self.nblocks += 1;
            } else {
                warn!("lookup_direct: already aquired at {}", idx);
            }
        }

        debug!(
            "lookup_direct: idx={}, aquire={}, nblocks={}, id={}",
            idx, aquire, self.nblocks, self.direct[idx]
        );

        Ok(&self.direct[idx])
    }

    fn lookup_indirect(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        if self.indirect.is_null() {
            self.indirect = Node::aquire(pager)?;
        }

        self.cache.resize_with(1, || Cache::new(pager));
        self.cache[0].refresh(pager, &self.indirect)?;

        debug!("lookup_indirect: cache={}", self.cache[0].id());

        if aquire {
            if self.cache[0].aquire(pager, idx, true)? {
                self.nblocks += 1;
            } else {
                warn!("lookup_indirect: already aquired at {}", idx);
            }
        }

        debug!(
            "loopup_indirect: idx={}, aquire={}, nblocks={}, id={}",
            idx, aquire, self.nblocks, self.cache[0][idx]
        );

        Ok(&self.cache[0][idx])
    }

    fn lookup_d_indirect(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(pager) as usize; // ids per node

        if self.d_indirect.is_null() {
            self.d_indirect = Node::aquire(pager)?;
        }

        self.cache.resize_with(2, || Cache::new(pager));

        let d_idx = ((idx / ipn) % ipn, idx % ipn);

        // level 0

        self.cache[0].refresh(pager, &self.d_indirect)?;
        debug!("lookup_d_indirect: cache[0]={}", self.cache[0].id());

        if aquire {
            self.cache[0].aquire(pager, d_idx.0, false)?;
        } else if self.cache[0][d_idx.0].is_null() {
            return Ok(&self.cache[0][d_idx.0]);
        }

        // level 1

        let id = self.cache[0][d_idx.0].clone();
        self.cache[1].refresh(pager, &id)?;
        debug!("lookup_d_indirect: cache[1]={}", self.cache[1].id());

        if aquire {
            if self.cache[1].aquire(pager, d_idx.1, true)? {
                self.nblocks += 1;
            } else {
                warn!("lookup_d_indirect: already aquired at {}", d_idx.1);
            }
        }

        debug!(
            "loopup_d_indirect: idx={} => ({}, {}), aquire={}, nblocks={}, id={}",
            idx, d_idx.0, d_idx.1, aquire, self.nblocks, self.cache[1][d_idx.1]
        );

        Ok(&self.cache[1][d_idx.1])
    }

    fn lookup_t_indirect(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(pager) as usize; // ids per node

        if self.t_indirect.is_null() {
            self.t_indirect = Node::aquire(pager)?;
        }

        self.cache.resize_with(3, || Cache::new(pager));

        let t_idx = ((idx / (ipn * ipn)) % ipn, (idx / ipn) % ipn, idx % ipn);

        // level 0

        self.cache[0].refresh(pager, &self.t_indirect)?;
        debug!("lookup_t_indirect: cache[0]={}", self.cache[0].id());

        if aquire {
            self.cache[0].aquire(pager, t_idx.0, false)?;
        } else if self.cache[0][t_idx.0].is_null() {
            return Ok(&self.cache[0][t_idx.0]);
        }

        // level 1

        let id = self.cache[0][t_idx.0].clone();
        self.cache[1].refresh(pager, &id)?;
        debug!("lookup_t_indirect: cache[1]={}", self.cache[1].id());

        if aquire {
            self.cache[1].aquire(pager, t_idx.1, false)?;
        } else if self.cache[1][t_idx.1].is_null() {
            return Ok(&self.cache[1][t_idx.1]);
        }

        // level 2

        let id = self.cache[1][t_idx.1].clone();
        self.cache[2].refresh(pager, &id)?;
        debug!("lookup_t_indirect: cache[2]={}", self.cache[2].id());

        if aquire {
            if self.cache[2].aquire(pager, t_idx.2, true)? {
                self.nblocks += 1;
            } else {
                warn!("lookup_t_indirect: already aquired at {}", t_idx.2);
            }
        }

        debug!(
            "loopup_t_indirect: idx={} => ({}, {}, {}), aquire={}, nblocks={}, id={}",
            idx, t_idx.0, t_idx.1, t_idx.2, aquire, self.nblocks, self.cache[2][t_idx.2]
        );

        Ok(&self.cache[2][t_idx.2])
    }
}

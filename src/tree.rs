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

use chrono::{DateTime, TimeZone, Utc};
use log::{debug, warn};
use nuts_container::backend::{Backend, BlockId};
use nuts_container::container::Container;
use std::mem;

use crate::container::BufContainer;
use crate::error::{ArchiveResult, Error};
use crate::tree::cache::Cache;
use crate::tree::node::Node;

fn ids_per_node<B: Backend>(container: &Container<B>) -> u32 {
    container.block_size() / B::Id::size() as u32
}

const NUM_DIRECT: u32 = 12;

fn into_datetime(millis: i64) -> DateTime<Utc> {
    Utc.timestamp_millis_opt(millis).unwrap()
}

fn from_datetime(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp_millis()
}

#[derive(Debug)]
pub struct Tree<B: Backend> {
    direct: Vec<B::Id>,
    indirect: B::Id,
    d_indirect: B::Id,
    t_indirect: B::Id,
    nblocks: u64,
    nfiles: u64,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    cache: Vec<Cache<B>>,
}

impl<B: Backend> Tree<B> {
    pub fn new() -> Tree<B> {
        let now = Utc::now();

        Tree {
            direct: vec![B::Id::null(); NUM_DIRECT as usize],
            indirect: B::Id::null(),
            d_indirect: B::Id::null(),
            t_indirect: B::Id::null(),
            nblocks: 0,
            nfiles: 0,
            created: now,
            modified: now,
            cache: vec![],
        }
    }

    pub fn nblocks(&self) -> u64 {
        self.nblocks
    }

    pub fn nfiles(&self) -> u64 {
        self.nfiles
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn modified(&self) -> DateTime<Utc> {
        self.modified
    }

    pub fn inc_nfiles(&mut self) {
        self.nfiles += 1;
        self.modified = Utc::now();
    }

    pub fn load(container: &mut BufContainer<B>, id: &B::Id) -> ArchiveResult<Tree<B>, B> {
        let min_size = 15 * B::Id::size() + // 12 * direct + 1 * indirect + 1 * d_indirect + 1 * t_indirect
                2 * mem::size_of::<u64>(); // nblocks + nfiles

        if (container.block_size() as usize) < min_size {
            return Err(Error::InvalidBlockSize);
        }

        let mut reader = container.read_buf(id)?;
        let mut tree = Self::new();

        for i in 0..NUM_DIRECT as usize {
            tree.direct[i] = reader.deserialize()?;
        }

        tree.indirect = reader.deserialize()?;
        tree.d_indirect = reader.deserialize()?;
        tree.t_indirect = reader.deserialize()?;
        tree.nblocks = reader.deserialize()?;
        tree.nfiles = reader.deserialize()?;
        tree.created = into_datetime(reader.deserialize()?);
        tree.modified = into_datetime(reader.deserialize()?);

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
        writer.serialize(&self.nfiles)?;
        writer.serialize(&from_datetime(&self.created))?;
        writer.serialize(&from_datetime(&self.modified))?;

        container.write_buf(id)?;

        Ok(())
    }

    pub fn aquire(&mut self, container: &mut BufContainer<B>) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container) as u64; // ids per node

        if self.nblocks < NUM_DIRECT as u64 + ipn + ipn * ipn + ipn * ipn * ipn {
            self.lookup_cache(container, self.nblocks as usize, true)
        } else {
            Err(Error::Full)
        }
    }

    pub fn lookup(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
    ) -> Option<ArchiveResult<&B::Id, B>> {
        if idx < self.nblocks as usize {
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
        let ipn = ids_per_node(container) as usize; // ids per node

        if idx < NUM_DIRECT as usize {
            self.lookup_direct(container, idx, aquire)
        } else if idx < NUM_DIRECT as usize + ipn {
            self.lookup_indirect(container, idx - NUM_DIRECT as usize, aquire)
        } else if idx < NUM_DIRECT as usize + ipn + ipn * ipn {
            self.lookup_d_indirect(container, idx - NUM_DIRECT as usize - ipn, aquire)
        } else {
            self.lookup_t_indirect(
                container,
                idx - NUM_DIRECT as usize - ipn - ipn * ipn,
                aquire,
            )
        }
    }

    fn lookup_direct(
        &mut self,
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        if aquire {
            if self.direct[idx].is_null() {
                self.direct[idx] = container.aquire()?;
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
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        if self.indirect.is_null() {
            self.indirect = Node::aquire(container)?;
        }

        self.cache.resize_with(1, || Cache::new(container));
        self.cache[0].refresh(container, &self.indirect)?;

        debug!("lookup_indirect: cache={}", self.cache[0].id());

        if aquire {
            if self.cache[0].aquire(container, idx, true)? {
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
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container) as usize; // ids per node

        if self.d_indirect.is_null() {
            self.d_indirect = Node::aquire(container)?;
        }

        self.cache.resize_with(2, || Cache::new(container));

        let d_idx = ((idx / ipn) % ipn, idx % ipn);

        // level 0

        self.cache[0].refresh(container, &self.d_indirect)?;
        debug!("lookup_d_indirect: cache[0]={}", self.cache[0].id());

        if aquire {
            self.cache[0].aquire(container, d_idx.0, false)?;
        } else if self.cache[0][d_idx.0].is_null() {
            return Ok(&self.cache[0][d_idx.0]);
        }

        // level 1

        let id = self.cache[0][d_idx.0].clone();
        self.cache[1].refresh(container, &id)?;
        debug!("lookup_d_indirect: cache[1]={}", self.cache[1].id());

        if aquire {
            if self.cache[1].aquire(container, d_idx.1, true)? {
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
        container: &mut BufContainer<B>,
        idx: usize,
        aquire: bool,
    ) -> ArchiveResult<&B::Id, B> {
        let ipn = ids_per_node(container) as usize; // ids per node

        if self.t_indirect.is_null() {
            self.t_indirect = Node::aquire(container)?;
        }

        self.cache.resize_with(3, || Cache::new(container));

        let t_idx = ((idx / (ipn * ipn)) % ipn, (idx / ipn) % ipn, idx % ipn);

        // level 0

        self.cache[0].refresh(container, &self.t_indirect)?;
        debug!("lookup_t_indirect: cache[0]={}", self.cache[0].id());

        if aquire {
            self.cache[0].aquire(container, t_idx.0, false)?;
        } else if self.cache[0][t_idx.0].is_null() {
            return Ok(&self.cache[0][t_idx.0]);
        }

        // level 1

        let id = self.cache[0][t_idx.0].clone();
        self.cache[1].refresh(container, &id)?;
        debug!("lookup_t_indirect: cache[1]={}", self.cache[1].id());

        if aquire {
            self.cache[1].aquire(container, t_idx.1, false)?;
        } else if self.cache[1][t_idx.1].is_null() {
            return Ok(&self.cache[1][t_idx.1]);
        }

        // level 2

        let id = self.cache[1][t_idx.1].clone();
        self.cache[2].refresh(container, &id)?;
        debug!("lookup_t_indirect: cache[2]={}", self.cache[2].id());

        if aquire {
            if self.cache[2].aquire(container, t_idx.2, true)? {
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

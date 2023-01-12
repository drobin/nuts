// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

mod block;
mod error;
mod read;
#[cfg(test)]
mod tests;
mod write;

use log::{debug, warn};
use std::{any, cmp, fmt, io};

use nuts_backend::Backend;

use crate::container::Container;
use crate::stream::block::Block;

pub use error::{StreamError, StreamResult};

macro_rules! init_last {
    ($self:expr) => {
        if let Err(err) = $self.init_last() {
            return Some(Err(err));
        }
    };
}

impl<'a, B: 'static + Backend> From<StreamError<B>> for io::Error {
    fn from(cause: StreamError<B>) -> Self {
        io::Error::new(io::ErrorKind::Other, cause)
    }
}

pub struct Stream<'a, B: Backend> {
    container: &'a mut Container<B>,
    first: Option<B::Id>,
    last: Option<B::Id>,
    cur: Option<Block<B>>,
    offs: usize,
}

impl<'a, B: Backend> Stream<'a, B> {
    pub fn new(container: &'a mut Container<B>, id: &B::Id) -> Stream<'a, B> {
        Stream {
            container,
            first: Some(id.clone()),
            last: None,
            cur: None,
            offs: 0,
        }
    }

    pub fn create(container: &'a mut Container<B>) -> Stream<'a, B> {
        Stream {
            container,
            first: None,
            last: None,
            cur: None,
            offs: 0,
        }
    }

    pub fn first_id(&self) -> Option<&B::Id> {
        self.first.as_ref()
    }

    pub fn current_id(&self) -> Option<&B::Id> {
        self.cur.as_ref().map(|cur| cur.id())
    }

    pub fn current_payload(&self) -> Option<&[u8]> {
        self.cur.as_ref().map(|cur| cur.payload())
    }

    pub fn set_current_payload(&mut self, payload: &[u8]) -> StreamResult<usize, B> {
        let num_bytes = cmp::min(self.max_payload(), payload.len());

        if let Some(cur_block) = self.cur.as_mut() {
            cur_block.set_payload(&payload[..num_bytes]);

            if num_bytes > 0 {
                cur_block.write(&mut self.container)?;
            }

            debug!(
                "{} bytes written into {}, requested: {}",
                num_bytes,
                cur_block.id(),
                payload.len()
            );

            Ok(num_bytes)
        } else {
            warn!("cannot update payload, no current block");
            return Ok(0);
        }
    }

    fn flush_current_block(&mut self) -> StreamResult<(), B> {
        if let Some(cur_block) = self.cur.as_mut() {
            cur_block.write(&mut self.container)?;
        }

        Ok(())
    }

    fn append_payload(&mut self, payload: &[u8]) -> usize {
        let max = self.max_payload();
        let len = cmp::min(max - self.offs, payload.len());

        if let Some(cur_block) = self.cur.as_mut() {
            cur_block.add_payload(&payload[..len]);
            self.offs += len;

            len
        } else {
            0
        }
    }

    fn remaining_payload(&self) -> Option<&[u8]> {
        if let Some(payload) = self.current_payload() {
            let offs = cmp::min(self.offs, payload.len());
            let len = payload.len() - offs;

            if len > 0 {
                payload.get(self.offs..self.offs + len)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn copy_remaining_payload(&mut self, buf: &mut [u8]) -> usize {
        match self.remaining_payload() {
            Some(payload) => {
                let len = cmp::min(payload.len(), buf.len());
                let source = &payload[..len];
                let target = &mut buf[..len];

                target.copy_from_slice(source);
                self.offs += len;

                len
            }
            None => 0,
        }
    }

    pub fn max_payload(&self) -> usize {
        self.container.backend().block_size() as usize - Block::<B>::header_size()
    }

    pub fn available_payload(&self) -> usize {
        let max = self.max_payload();

        match self.current_payload() {
            Some(payload) => {
                let len = cmp::min(payload.len(), max);
                max - len
            }
            None => 0,
        }
    }

    pub fn first_block(&mut self) -> Option<StreamResult<&B::Id, B>> {
        init_last!(self);

        if let Some(first_id) = self.first.as_ref() {
            match Block::load(&mut self.container, first_id.clone()) {
                Ok(block) => {
                    debug!("loading first block {}", block.id());
                    self.set_current(block).map(|id| Ok(id))
                }
                Err(err) => Some(Err(err)),
            }
        } else {
            debug!("empty stream, no first block");
            None
        }
    }

    pub fn last_block(&mut self) -> Option<StreamResult<&B::Id, B>> {
        init_last!(self);

        if let Some(last_id) = self.last.as_ref() {
            match Block::load(&mut self.container, last_id.clone()) {
                Ok(block) => {
                    debug!("loading last block {}", block.id());
                    self.set_current(block).map(|id| Ok(id))
                }
                Err(err) => Some(Err(err)),
            }
        } else {
            debug!("empty stream, no last block");
            None
        }
    }

    pub fn next_block(&mut self) -> Option<StreamResult<&B::Id, B>> {
        init_last!(self);

        if let Some(cur_block) = self.cur.as_ref() {
            match cur_block.next() {
                Some(next_id) => match Block::load(&mut self.container, next_id.clone()) {
                    Ok(block) => {
                        debug!("loading next block {}", block.id());
                        self.set_current(block).map(|id| Ok(id))
                    }
                    Err(err) => Some(Err(err)),
                },
                None => {
                    debug!("No next block, end of stream reached");
                    None
                }
            }
        } else {
            debug!("no current block, switch to first block");
            self.first_block()
        }
    }

    pub fn prev_block(&mut self) -> Option<StreamResult<&B::Id, B>> {
        init_last!(self);

        if let Some(cur_block) = self.cur.as_ref() {
            if cur_block.is_first() {
                debug!("this is the first block, cannot go to previous");
                return None;
            }

            match cur_block.prev() {
                Some(prev) => match Block::load(&mut self.container, prev.clone()) {
                    Ok(block) => {
                        debug!("loading next block {}", block.id());
                        self.set_current(block).map(|id| Ok(id))
                    }
                    Err(err) => Some(Err(err)),
                },
                None => {
                    debug!("No previous block, start of stream reached");
                    None
                }
            }
        } else {
            debug!("No current block, cannot go to previous");
            None
        }
    }

    pub fn insert_front(&mut self, id: Option<B::Id>) -> StreamResult<&B::Id, B> {
        let mut block = Block::new(&mut self.container, id, true)?;

        if let Some(first_id) = &self.first {
            let mut first_block = Block::load(&mut self.container, first_id.clone())?;

            // block -> first_block

            block.set_prev_opt(first_block.prev().cloned()); // copy last-block from previous to new first block
            block.set_next(first_block.id().clone());

            first_block.set_first(false);
            first_block.set_prev(block.id().clone());

            block.write(&mut self.container)?;
            first_block.write(&mut self.container)?;

            debug!("replaced front to {} from {}", block.id(), first_block.id());
        } else {
            block.set_prev(block.id().clone());
            block.write(&mut self.container)?;

            debug!("empty stream, inserted {} at front", block.id())
        }

        self.first = Some(block.id().clone());

        Ok(self.set_current(block).unwrap())
    }

    pub fn insert(&mut self, id: Option<B::Id>) -> StreamResult<&B::Id, B> {
        if self.first.is_none() {
            debug!("empty stream, insert {:?} at front", id);
            return self.insert_front(id);
        }

        // block to be inserted after cur_block
        let mut block = Block::new(&mut self.container, id, false)?;

        if let Some(cur_block) = &mut self.cur {
            let mut next_block = match cur_block.next().cloned() {
                Some(id) => Some(Block::load(&mut self.container, id)?),
                None => None,
            };

            // cur_block -> block -> next_block

            block.set_prev(cur_block.id().clone());
            block.set_next_opt(next_block.as_ref().map(|block| block.id()).cloned());

            cur_block.set_next(block.id().clone());

            if let Some(next) = next_block.as_mut() {
                next.set_prev(block.id().clone());
                next.write(&mut self.container)?;
            }

            cur_block.write(&mut self.container)?;
            block.write(&mut self.container)?;

            debug!(
                "{} inserted: {} <-> {:?}",
                block.id(),
                cur_block.id(),
                next_block.as_ref().map(|b| b.id())
            );

            Ok(self.set_current(block).unwrap())
        } else {
            debug!("no current block, insert {} at front", block.id());
            self.insert_front(Some(block.id().clone()))
        }
    }

    fn init_last(&mut self) -> StreamResult<(), B> {
        if self.last.is_none() {
            if let Some(first) = &self.first {
                let block = Block::load(&mut self.container, first.clone())?;
                self.last = Some(block.prev().unwrap().clone());
            }
        }

        Ok(())
    }

    fn set_current(&mut self, block: Block<B>) -> Option<&B::Id> {
        self.cur = Some(block);
        self.offs = 0;
        self.current_id()
    }
}

impl<'a, B: Backend> fmt::Debug for Stream<'a, B> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Stream")
            .field("container", &any::type_name::<Container<B>>())
            .field("first", &self.first)
            .field("last", &self.last)
            .field("cur", &self.cur.as_ref().map(|b| b.id()))
            .field("offs", &self.offs)
            .finish()
    }
}

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

use std::io::Cursor;
use std::{cmp, mem};

use nuts_backend::{Backend, BlockId};
use nuts_bytes::{FromBytesExt, ToBytesExt};

use crate::container::Container;
use crate::stream::error::{StreamError, StreamResult};

const MAGIC_0: [u8; 7] = *b"stream0";
const MAGIC_N: [u8; 7] = *b"streamn";

fn read_magic<B: Backend>(src: &mut Cursor<&[u8]>) -> StreamResult<bool, B> {
    let mut magic = [0; MAGIC_0.len()];

    src.read_bytes(&mut magic)?;

    if magic == MAGIC_0 {
        Ok(true)
    } else if magic == MAGIC_N {
        Ok(false)
    } else {
        Err(StreamError::InvalidMagic)
    }
}

fn write_magic<B: Backend>(cursor: &mut Cursor<Vec<u8>>, first: bool) -> StreamResult<(), B> {
    let magic = if first { MAGIC_0 } else { MAGIC_N };

    Ok(cursor.write_bytes(&magic)?)
}

fn read_id<B: Backend>(src: &mut Cursor<&[u8]>) -> StreamResult<Option<B::Id>, B> {
    let id = src.from_bytes::<B::Id>()?;

    if id.is_null() {
        Ok(None)
    } else {
        Ok(Some(id))
    }
}

fn write_id<B: Backend>(cursor: &mut Cursor<Vec<u8>>, id: &Option<B::Id>) -> StreamResult<(), B> {
    match id {
        Some(id) => Ok(cursor.to_bytes(id)?),
        None => Ok(cursor.to_bytes(&B::Id::null())?),
    }
}

#[derive(Debug)]
pub struct Block<B: Backend> {
    id: B::Id,
    prev: Option<B::Id>,
    next: Option<B::Id>,
    first: bool,
    payload: Vec<u8>,
}

impl<B: Backend> Block<B> {
    fn load_block(container: &mut Container<B>, id: &B::Id) -> StreamResult<Vec<u8>, B> {
        let block_size = container.backend().block_size() as usize;
        let mut block = vec![0; block_size];

        container.read(id, &mut block)?;

        Ok(block)
    }

    pub fn new(
        container: &mut Container<B>,
        id: Option<B::Id>,
        first: bool,
    ) -> StreamResult<Block<B>, B> {
        let id = match id {
            Some(id) => id,
            None => container.aquire()?,
        };

        Ok(Block {
            id,
            prev: None,
            next: None,
            first,
            payload: vec![],
        })
    }

    pub fn load(container: &mut Container<B>, id: B::Id) -> StreamResult<Block<B>, B> {
        let block = Self::load_block(container, &id)?;
        let mut cursor = Cursor::new(block.as_slice());

        let first = read_magic::<B>(&mut cursor)?;
        let prev = read_id::<B>(&mut cursor)?;
        let next = read_id::<B>(&mut cursor)?;
        let length = cursor.from_bytes::<u32>()? as usize;

        let pos = cursor.position() as usize;
        let remaining = block.len() - pos;
        let len = cmp::min(remaining, length);
        let payload = cursor.get_ref()[pos..pos + len].to_vec();

        Ok(Block {
            id,
            prev,
            next,
            first,
            payload,
        })
    }

    pub fn write(&self, container: &mut Container<B>) -> StreamResult<(), B> {
        let block_size = container.backend().block_size() as usize;
        let mut cursor = Cursor::new(Vec::with_capacity(block_size));

        write_magic(&mut cursor, self.first)?;
        write_id(&mut cursor, &self.prev)?;
        write_id(&mut cursor, &self.next)?;

        let pos = cursor.position() as usize;
        let remaining = block_size - pos - 4;
        let len = cmp::min(remaining, self.payload.len());
        let payload = &self.payload[..len];

        cursor.to_bytes(&(len as u32))?;
        cursor.write_bytes(payload)?;

        container.write(&self.id, &cursor.into_inner())?;

        Ok(())
    }

    pub fn is_first(&self) -> bool {
        self.first
    }

    pub fn set_first(&mut self, first: bool) {
        self.first = first
    }

    pub fn id(&self) -> &B::Id {
        &self.id
    }

    pub fn prev(&self) -> Option<&B::Id> {
        self.prev.as_ref()
    }

    pub fn set_prev(&mut self, id: B::Id) {
        self.prev = Some(id);
    }

    pub fn set_prev_opt(&mut self, id: Option<B::Id>) {
        self.prev = id;
    }

    pub fn next(&self) -> Option<&B::Id> {
        self.next.as_ref()
    }

    pub fn set_next(&mut self, id: B::Id) {
        self.next = Some(id);
    }

    pub fn set_next_opt(&mut self, id: Option<B::Id>) {
        self.next = id;
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn set_payload(&mut self, payload: &[u8]) {
        self.payload.clear();
        self.add_payload(payload);
    }

    pub fn add_payload(&mut self, payload: &[u8]) {
        self.payload.extend_from_slice(payload);
    }

    pub fn header_size() -> usize {
        MAGIC_0.len() + 2 * B::Id::size() + mem::size_of::<u32>()
    }
}

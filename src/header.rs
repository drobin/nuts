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
use nuts_bytes::{Reader, Writer};
use nuts_container::backend::Backend;
use nuts_container::container::Container;
use serde::{Deserialize, Serialize};
use std::{cmp, fmt};

use crate::magic::{magic_type, MAGIC};
use crate::ArchiveResult;

magic_type!(Magic, "invalid header-magic");

struct Block(Vec<u8>);

impl Block {
    fn read<B: Backend>(container: &mut Container<B>, id: &B::Id) -> ArchiveResult<Block, B> {
        let mut vec = vec![0; container.block_size() as usize];

        container.read(id, &mut vec)?;

        Ok(Block(vec))
    }

    fn is_empty(&self) -> bool {
        let n = cmp::min(self.0.len(), MAGIC.len());

        self.0[..n].iter().all(|n| *n == 0)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Header<B: Backend> {
    magic: Magic, // nuts-archive
    revision: u8,
    first: Option<B::Id>,
    last: Option<B::Id>,
}

impl<B: Backend> Header<B> {
    fn new() -> Header<B> {
        Header {
            magic: Magic::new(),
            revision: 1,
            first: None,
            last: None,
        }
    }

    pub fn load_or_create(container: &mut Container<B>, id: &B::Id) -> ArchiveResult<Header<B>, B> {
        let block = Block::read(container, id)?;

        let header = if block.is_empty() {
            debug!("header block is empty, create a new header");

            let h = Header::<B>::new();

            h.write(container, id)?;

            h
        } else {
            debug!("read already existing header");
            Self::read(container, id)?
        };

        Ok(header)
    }

    pub fn read(container: &mut Container<B>, id: &B::Id) -> ArchiveResult<Header<B>, B> {
        let block = Block::read(container, id)?;
        let mut reader = Reader::new(&block.0[..]);

        Ok(reader.deserialize()?)
    }

    pub fn write(&self, container: &mut Container<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let mut writer = Writer::new(vec![]);

        // FIXME Header too large
        writer.serialize(self)?;
        container.write(id, &writer.into_target())?;

        Ok(())
    }
}

impl<B: Backend> fmt::Debug for Header<B> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Header")
            .field("revision", &self.revision)
            .field("first", &self.first)
            .field("last", &self.last)
            .finish()
    }
}

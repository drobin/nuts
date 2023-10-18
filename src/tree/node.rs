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

use nuts_container::backend::{Backend, BlockId};
use std::ops::{Index, IndexMut};

use crate::container::BufContainer;
use crate::error::ArchiveResult;
use crate::error::Error;
use crate::tree::ids_per_node;

#[derive(Debug, PartialEq)]
pub struct Node<B: Backend>(Vec<B::Id>);

impl<B: Backend> Node<B> {
    pub fn new(len: usize) -> Node<B> {
        Node(vec![B::Id::null(); len])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn aquire(container: &mut BufContainer<B>) -> ArchiveResult<B::Id, B> {
        let ipn = ids_per_node(container);
        let id = container.aquire()?;

        let mut node = Node::new(ipn);

        node.0.resize(ipn, B::Id::null());
        node.flush(container, &id).map(|()| id)
    }

    pub fn fill(&mut self, container: &mut BufContainer<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let ipn = ids_per_node(container);
        let mut reader = container.read_buf(id)?;

        self.0.clear();

        for _ in 0..ipn {
            self.0.push(reader.deserialize::<B::Id>()?);
        }

        Ok(())
    }

    pub fn flush(&self, container: &mut BufContainer<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let block_size = container.block_size() as usize;
        let mut writer = container.create_writer();
        let mut n = 0;
        for id in self.0.iter() {
            n += writer.serialize(id)?;
        }

        if n + B::Id::size() <= block_size {
            return Err(Error::InvalidBlockSize);
        }

        container.write_buf(id)
    }
}

impl<B: Backend> Index<usize> for Node<B> {
    type Output = B::Id;

    fn index(&self, index: usize) -> &B::Id {
        &self.0[index]
    }
}

impl<B: Backend> IndexMut<usize> for Node<B> {
    fn index_mut(&mut self, index: usize) -> &mut B::Id {
        &mut self.0[index]
    }
}

impl<B: Backend, T: AsRef<[B::Id]>> PartialEq<T> for Node<B> {
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}

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

#[cfg(test)]
mod tests;

use nuts_backend::{Backend, BlockId};
use std::ops::{Index, IndexMut};

use crate::error::ArchiveResult;
use crate::pager::Pager;
use crate::tree::ids_per_node;

#[derive(Debug, PartialEq)]
pub struct Node<B: Backend>(Vec<B::Id>);

impl<B: Backend> Node<B> {
    pub fn new(pager: &Pager<B>) -> Node<B> {
        let ipn = ids_per_node(pager) as usize;

        Node(vec![B::Id::null(); ipn])
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn aquire(pager: &mut Pager<B>) -> ArchiveResult<B::Id, B> {
        let id = pager.aquire()?;
        Node::new(pager).flush(pager, &id).map(|()| id)
    }

    pub fn fill(&mut self, pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let ipn = ids_per_node(pager);
        let mut reader = pager.read_buf(id)?;

        self.0.clear();

        for _ in 0..ipn {
            self.0.push(reader.read::<B::Id>()?);
        }

        Ok(())
    }

    pub fn flush(&self, pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<(), B> {
        let block_size = pager.block_size() as usize;
        let expected_size = self.0.len() * B::Id::size();

        if expected_size > block_size {
            panic!(
                "node overflow detected, about to write {} ids ({} bytes each) into {} bytes",
                self.0.len(),
                B::Id::size(),
                block_size
            );
        }

        if expected_size + B::Id::size() <= block_size {
            panic!(
                "node underflow detected, about to write {} ids ({} bytes each) into {} bytes",
                self.0.len(),
                B::Id::size(),
                block_size
            );
        }

        let mut writer = pager.create_writer();

        for id in self.0.iter() {
            writer.write(id)?;
        }

        pager.write_buf(id)
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

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
use std::ops::Deref;

use crate::error::ArchiveResult;
use crate::pager::Pager;
use crate::tree::node::Node;

#[derive(Debug)]
pub struct Cache<B: Backend> {
    id: B::Id,
    node: Node<B>,
}

impl<'a, B: Backend> Cache<B> {
    pub fn new(pager: &Pager<B>) -> Cache<B> {
        Cache {
            id: B::Id::null(),
            node: Node::new(pager),
        }
    }

    pub fn id(&self) -> &B::Id {
        &self.id
    }

    pub fn refresh(&mut self, pager: &mut Pager<B>, id: &B::Id) -> ArchiveResult<bool, B> {
        if &self.id != id {
            self.id = id.clone();
            self.node.fill(pager, id)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn aquire(
        &mut self,
        pager: &mut Pager<B>,
        idx: usize,
        leaf: bool,
    ) -> ArchiveResult<bool, B> {
        if self.node[idx].is_null() {
            self.node[idx] = if leaf {
                pager.aquire()?
            } else {
                Node::aquire(pager)?
            };

            self.node.flush(pager, &self.id)?;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl<B: Backend> Deref for Cache<B> {
    type Target = Node<B>;

    fn deref(&self) -> &Node<B> {
        &self.node
    }
}

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

use nuts_backend::Backend;

use crate::error::ArchiveResult;
use crate::pager::Pager;
use crate::tree::node::Node;

#[derive(Debug)]
struct Inner<B: Backend> {
    id: Option<B::Id>,
    node: Node<B>,
}

impl<'a, B: Backend> Inner<B> {
    fn new() -> Inner<B> {
        Inner {
            id: None,
            node: Node::new(),
        }
    }

    fn refresh(&mut self, id: &B::Id, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        let must_refresh = match self.id.as_ref() {
            Some(in_id) => in_id != id,
            None => true,
        };

        if must_refresh {
            self.id = Some(id.clone());
            self.node.load(id, pager)?;
        }

        Ok(())
    }

    fn flush(&mut self, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        if let Some(id) = self.id.as_ref() {
            self.node.flush(id, pager)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Cache<B: Backend>(Vec<Inner<B>>);

impl<B: Backend> Cache<B> {
    pub fn new() -> Cache<B> {
        Cache(vec![])
    }

    pub fn resolve<'a>(
        &'a mut self,
        pager: &mut Pager<B>,
        start: Option<&'a B::Id>,
        idxs: &[usize],
    ) -> ArchiveResult<Option<&'a B::Id>, B> {
        self.0.resize_with(idxs.len(), || Inner::new());

        let mut id_opt = start;

        for (entry, idx) in self.0.iter_mut().zip(idxs) {
            match id_opt {
                Some(id) => {
                    entry.refresh(id, pager)?;
                    id_opt = entry.node.get(*idx);
                }
                None => return Ok(None),
            }
        }

        Ok(id_opt)
    }

    pub fn aquire<'a>(
        &'a mut self,
        pager: &mut Pager<B>,
        start: &'a B::Id,
        idxs: &[usize],
    ) -> ArchiveResult<&'a B::Id, B> {
        self.0.resize_with(idxs.len(), || Inner::new());

        let mut id = start;

        for (entry, idx) in self.0.iter_mut().zip(idxs) {
            entry.refresh(id, pager)?;

            if entry.node.get(*idx).is_none() {
                entry.node.aquire(pager)?;
                entry.flush(pager)?;
            }

            id = &entry.node[*idx];
        }

        Ok(id)
    }
}

impl<B: Backend> Default for Cache<B> {
    fn default() -> Self {
        Cache::new()
    }
}

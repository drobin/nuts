// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use std::ops::Deref;

use nuts_backend::Backend;
use nuts_bytes::{Reader, Writer};

use crate::error::ArchiveResult;
use crate::pager::Pager;

#[derive(Debug)]
pub struct Node<B: Backend> {
    buf: Vec<u8>,
    vec: Vec<B::Id>,
}

impl<B: Backend> Node<B> {
    pub fn new() -> Node<B> {
        Node {
            buf: vec![],
            vec: vec![],
        }
    }

    pub fn load(&mut self, id: &B::Id, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        self.buf.resize(pager.block_size() as usize, 0);
        pager.read(id, &mut self.buf)?;

        self.vec.clear();

        let mut reader = Reader::new(self.buf.as_slice());
        let count = reader.read::<u32>()?;

        for _ in 0..count {
            self.vec.push(reader.read()?);
        }

        Ok(())
    }

    pub fn add(&mut self, id: B::Id) {
        self.vec.push(id)
    }

    pub fn flush(&mut self, id: &B::Id, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        self.buf.resize(pager.block_size() as usize, 0);

        let mut writer = Writer::new(self.buf.as_mut_slice());

        writer.write(&(self.vec.len() as u32))?;

        for id in &self.vec {
            writer.write(id)?;
        }

        pager.write(id, &self.buf)?;

        Ok(())
    }
}

impl<B: Backend> Deref for Node<B> {
    type Target = [B::Id];

    fn deref(&self) -> &[B::Id] {
        &self.vec
    }
}

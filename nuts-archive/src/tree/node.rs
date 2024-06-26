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

#[cfg(test)]
mod tests;

use nuts_backend::Backend;
use nuts_bytes::{PutBytesError, Reader, Writer};
use std::ops::Deref;

use crate::error::{ArchiveResult, Error};
use crate::id::Id;
use crate::pager::Pager;

const MAGIC: [u8; 4] = *b"node";

#[derive(Debug)]
pub struct Node<B: Backend> {
    buf: Vec<u8>,
    vec: Vec<Id<B>>,
}

impl<B: Backend> Node<B> {
    pub fn new() -> Node<B> {
        Node {
            buf: vec![],
            vec: vec![],
        }
    }

    pub fn load(&mut self, id: &Id<B>, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        self.buf.resize(pager.block_size() as usize, 0);
        pager.read(id, &mut self.buf)?;

        self.vec.clear();

        let mut reader = Reader::new(self.buf.as_slice());

        let magic = reader.read::<[u8; MAGIC.len()]>()?;

        if magic != MAGIC {
            return Err(crate::Error::InvalidNode(id.as_ref().clone()));
        }

        let count = reader.read::<u32>()?;

        for _ in 0..count {
            self.vec.push(reader.read()?);
        }

        Ok(())
    }

    pub fn aquire(&mut self, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        let id = pager.aquire()?;

        Node::<B>::new().flush(&id, pager)?;
        self.vec.push(id);

        Ok(())
    }

    pub fn flush(&mut self, id: &Id<B>, pager: &mut Pager<B>) -> ArchiveResult<(), B> {
        self.buf.resize(pager.block_size() as usize, 0);

        if let Err(err) = self.flush_to_buf() {
            let err: Error<B> = match err {
                nuts_bytes::Error::PutBytes(PutBytesError::NoSpace) => Error::InvalidBlockSize,
                _ => err.into(),
            };
            return Err(err);
        }

        pager.write(id, &self.buf)?;

        Ok(())
    }

    fn flush_to_buf(&mut self) -> Result<(), nuts_bytes::Error> {
        let mut writer = Writer::new(self.buf.as_mut_slice());

        writer.write(&MAGIC)?;
        writer.write(&(self.vec.len() as u32))?;

        for id in &self.vec {
            writer.write(id)?;
        }

        Ok(())
    }
}

impl<B: Backend> Deref for Node<B> {
    type Target = [Id<B>];

    fn deref(&self) -> &[Id<B>] {
        &self.vec
    }
}

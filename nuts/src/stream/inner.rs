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

pub(super) mod stream;
#[cfg(test)]
mod tests;

use nuts_bytes::{Reader, Writer};
use std::cmp;
use std::ops::{Deref, DerefMut};

use crate::backend::{Backend, BlockId};
use crate::container::Container;
use crate::stream::error::Error;
use crate::svec::SecureVec;

enum EncodeOption<'a> {
    Include(&'a [u8]),
    Skip(usize),
}

struct Buffer<B: Backend> {
    container: Container<B>,
    buf: SecureVec,
}

impl<B: Backend> Buffer<B> {
    fn new(container: Container<B>) -> Buffer<B> {
        let buf = vec![0; container.backend().block_size() as usize];

        Buffer {
            container,
            buf: buf.into(),
        }
    }

    fn read(&mut self, id: &B::Id) -> Result<usize, Error<B>> {
        self.container
            .read(id, &mut self.buf)
            .map_err(|err| err.into())
    }

    fn read_block(&mut self, id: &B::Id) -> Result<(B::Id, B::Id, usize, usize), Error<B>> {
        self.read(id)
            .and_then(|_| self.decode_block().map_err(|err| err.into()))
    }

    fn write(&mut self, id: &B::Id) -> Result<usize, Error<B>> {
        self.container
            .write(id, &self.buf)
            .map_err(|err| err.into())
    }

    fn write_block(
        &mut self,
        id: &B::Id,
        id1: &B::Id,
        id2: &B::Id,
        option: EncodeOption,
    ) -> Result<usize, Error<B>> {
        let n = self.encode_block(id1, id2, option)?;
        self.write(id).map(|_| n)
    }

    fn decode_block(&self) -> Result<(B::Id, B::Id, usize, usize), nuts_bytes::Error> {
        let mut reader = Reader::new(self.buf.as_slice());

        let id1 = reader.deserialize()?;
        let id2 = reader.deserialize()?;
        let len = reader.deserialize::<u32>()?;

        let pos = self.buf.len() - reader.as_ref().len();
        let nbytes = cmp::min(len as usize, reader.as_ref().len());

        Ok((id1, id2, pos, nbytes))
    }

    fn encode_block(
        &mut self,
        id1: &B::Id,
        id2: &B::Id,
        option: EncodeOption,
    ) -> Result<usize, nuts_bytes::Error> {
        let buf_len = self.buf.len();
        let mut writer = Writer::new(self.buf.as_mut_slice());

        writer.serialize(id1)?;
        writer.serialize(id2)?;

        let max = writer.as_ref().len() - 4;

        match option {
            EncodeOption::Include(buf) => {
                let len = cmp::min(max, buf.len());
                let payload = &buf[..len];

                writer.serialize(&(len as u32))?;
                writer.write_bytes(payload)?;
            }
            EncodeOption::Skip(size) => {
                let len = cmp::min(max, size);

                writer.serialize(&(len as u32))?;
            }
        };

        Ok(buf_len - writer.as_ref().len())
    }
}

impl<B: Backend> Deref for Buffer<B> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.buf
    }
}

impl<B: Backend> DerefMut for Buffer<B> {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buf
    }
}

struct Current<B: Backend> {
    id: B::Id,
    prev: B::Id,
    next: B::Id,
    start: usize,
    len: usize,
    offs: usize,
}

pub struct Inner<B: Backend> {
    buf: Buffer<B>,
    first: B::Id,
    last: B::Id,
    cur: Option<Current<B>>,
}

impl<B: Backend> Inner<B> {
    /// Opens an existing stream which starts at the given `id`.
    ///
    /// The [first](`Self::first`) and the [last](`Self::last`) block are
    /// evaluated by reading and parsing the first block (which has the id
    /// `id`).
    ///
    /// The stream is positioned *before* the first block, so the
    /// [current](Self::current) block will be [`None`] and the first
    /// invocation of [`Self::goto_next()`] will switch the stream to the first
    /// block.
    fn open(container: Container<B>, id: B::Id) -> Result<Inner<B>, Error<B>> {
        let mut buf = Buffer::new(container);
        let (last, ..) = buf.read_block(&id)?;

        Ok(Inner {
            buf,
            first: id,
            last,
            cur: None,
        })
    }

    /// Creates a new stream whch should start at the given `id`.
    ///
    /// The given id will be initially the [first](`Self::first`) and the
    /// [last](`Self::last`) block of the stream.
    ///
    /// The stream is positioned *before* the first block, so the
    /// [current](Self::current) block will be [`None`] and the first
    /// invocation of [`Self::goto_next()`] will switch the stream to the first
    /// block.
    fn create(container: Container<B>, id: B::Id) -> Result<Inner<B>, Error<B>> {
        let mut buf = Buffer::new(container);

        buf.write_block(&id, &id, &B::Id::null(), EncodeOption::Include(&[]))?;

        Ok(Inner {
            buf,
            first: id.clone(),
            last: id,
            cur: None,
        })
    }

    /// Consumes this stream and returns the inner [`Container`].
    pub fn into_container(self) -> Container<B> {
        self.buf.container
    }

    /// Returns the first block of the stream.
    pub fn first(&self) -> &B::Id {
        &self.first
    }

    /// Returns the last block of the stream.
    pub fn last(&self) -> &B::Id {
        &self.last
    }

    /// Returns the current block of the stream.
    ///
    /// The current block is [`None`], if the stream was created just now,
    /// before the first invocation of one of the `goto_*` methods. In this
    /// case the position of the stream is *before* the first block.
    pub fn current(&self) -> Option<&B::Id> {
        self.cur.as_ref().map(|cur| &cur.id)
    }

    /// Tests whether the [current](Self::current) block is the first in the
    /// stream.
    pub fn is_first(&self) -> bool {
        self.cur.as_ref().map_or(false, |cur| cur.id == self.first)
    }

    /// Tests whether the [current](Self::current) block is the last in the
    /// stream.
    pub fn is_last(&self) -> bool {
        self.cur.as_ref().map_or(false, |cur| cur.id == self.last)
    }

    /// Switches to the first block of the stream.
    ///
    /// The [current](Self::current) block will be the first one, the id of the
    /// new current resp. first block is returned.
    pub fn goto_first(&mut self) -> Result<&B::Id, Error<B>> {
        self.goto(self.first.clone())
    }

    /// Switches to the last block of the stream.
    ///
    /// The [current](Self::current) block will be the last one, the id of the
    /// new current resp. last block is returned.
    pub fn goto_last(&mut self) -> Result<&B::Id, Error<B>> {
        self.goto(self.last.clone())
    }

    /// Switches to the previous block of the stream.
    ///
    /// If the [current](Self::current) block is the [first](Self::last) one,
    /// no switch is performed and [`None`] is returned. Otherwise it switches
    /// the stream to the previous block and returns its id.
    pub fn goto_prev(&mut self) -> Option<Result<&B::Id, Error<B>>> {
        let cur = self.cur.as_ref().map(|cur| (&cur.id, cur.prev.clone()));

        match cur {
            Some((cur, prev)) => {
                if cur == &self.first {
                    None
                } else {
                    Some(self.goto(prev))
                }
            }
            None => None,
        }
    }

    /// Switches to the next block of the stream.
    ///
    /// If the [current](Self::current) block is the [last](Self::last) one,
    /// no switch is performed and [`None`] is returned. Otherwise it switches
    /// the stream to the next block and returns its id.
    pub fn goto_next(&mut self) -> Option<Result<&B::Id, Error<B>>> {
        let cur = self.cur.as_ref().map(|cur| (&cur.id, cur.next.clone()));

        match cur {
            Some((cur, next)) => {
                if cur == &self.last {
                    None
                } else {
                    Some(self.goto(next))
                }
            }
            None => Some(self.goto_first()),
        }
    }

    fn goto(&mut self, id: B::Id) -> Result<&B::Id, Error<B>> {
        let (id1, id2, start, len) = self.buf.read_block(&id)?;

        self.cur = Some(Current {
            id,
            prev: id1,
            next: id2,
            start,
            len,
            offs: 0,
        });

        Ok(&self.cur.as_ref().unwrap().id)
    }

    /// Inserts a new block at the front of the stream.
    ///
    /// The stream will be positioned at the new block so the payload can be
    /// modified directly with one of the `payload_*` methods.
    ///
    /// Returns the id of the new block, which is also the new
    /// [first](Self::first) block.
    pub fn insert_front(&mut self) -> Result<&B::Id, Error<B>> {
        // new -> self.first
        let new = self.buf.container.aquire()?;

        let (last, old_next, _, old_len) = self.buf.read_block(&self.first)?;

        self.buf
            .write_block(&self.first, &new, &old_next, EncodeOption::Skip(old_len))?;

        self.buf
            .write_block(&new, &last, &self.first, EncodeOption::Include(&[]))?;

        self.first = new;
        self.goto(self.first.clone())
    }

    /// Inserts a new block at the back of the stream.
    ///
    /// The stream will be positioned at the new block so the payload can be
    /// modified directly with one of the `payload_*` methods.
    ///
    /// Returns the id of the new block, which is also the new
    /// [last](Self::last) block.
    pub fn insert_back(&mut self) -> Result<&B::Id, Error<B>> {
        // self.last -> new
        let new = self.buf.container.aquire()?;

        let (old_prev, _, _, old_len) = self.buf.read_block(&self.last)?;

        self.buf
            .write_block(&self.last, &old_prev, &new, EncodeOption::Skip(old_len))?;

        let (_, old_next, _, old_len) = self.buf.read_block(&self.first)?;
        self.buf
            .write_block(&self.first, &new, &old_next, EncodeOption::Skip(old_len))?;

        self.buf
            .write_block(&new, &self.last, &B::Id::null(), EncodeOption::Include(&[]))?;

        self.last = new;
        self.goto(self.last.clone())
    }

    /// Inserts a new block previous to the [current](Self::current) of the
    /// stream.
    ///
    /// If the stream is positioned before the first block, the new block is
    /// inserted at the front of the stream.
    ///
    /// The stream will be positioned at the new block so the payload can be
    /// modified directly with one of the `payload_*` methods.
    ///
    /// Returns the id of the new block.
    pub fn insert_prev(&mut self) -> Result<&B::Id, Error<B>> {
        match self.cur.as_ref() {
            Some(cur) => {
                if cur.id == self.first {
                    return self.insert_front();
                }

                // cur.prev -> new -> cur.id
                let new = self.buf.container.aquire()?;

                self.buf
                    .write_block(&cur.id, &new, &cur.next, EncodeOption::Skip(cur.len))?;

                let (prev_prev, _, _, prev_len) = self.buf.read_block(&cur.prev)?;
                self.buf
                    .write_block(&cur.prev, &prev_prev, &new, EncodeOption::Skip(prev_len))?;

                self.buf
                    .write_block(&new, &cur.prev, &cur.id, EncodeOption::Include(&[]))?;

                self.goto(new)
            }
            None => self.insert_front(),
        }
    }

    /// Inserts a new block next to the [current](Self::current) of the stream.
    ///
    /// If the stream is positioned before the first block, the new block is
    /// inserted at the front of the stream.
    ///
    /// The stream will be positioned at the new block so the payload can be
    /// modified directly with one of the `payload_*` methods.
    ///
    /// Returns the id of the new block.
    pub fn insert_next(&mut self) -> Result<&B::Id, Error<B>> {
        match self.cur.as_ref() {
            Some(cur) => {
                if cur.id == self.last {
                    return self.insert_back();
                }

                // cur.id -> new -> cur.next
                let new = self.buf.container.aquire()?;

                self.buf
                    .write_block(&cur.id, &cur.prev, &new, EncodeOption::Skip(cur.len))?;

                let (_, next_next, _, next_len) = self.buf.read_block(&cur.next)?;
                self.buf
                    .write_block(&cur.next, &new, &next_next, EncodeOption::Skip(next_len))?;

                self.buf
                    .write_block(&new, &cur.id, &cur.next, EncodeOption::Include(&[]))?;

                self.goto(new)
            }
            None => self.insert_front(),
        }
    }

    /// Returns the payload of the [current](Self::current) block.
    ///
    /// If there is no current block, [`None`] is returned.
    pub fn payload(&self) -> Option<&[u8]> {
        self.cur
            .as_ref()
            .map(|cur| self.buf.get(cur.start..cur.start + cur.len))
            .flatten()
    }

    /// Updates the payload of the [current](Self::current) block.
    ///
    /// On success returns the number of bytes assigned to the current block.
    /// If there is no current block `0` is returned.
    ///
    /// **Note:** You need to call [`Self::flush()`] to save your changes!
    pub fn payload_set(&mut self, payload: &[u8]) -> Result<usize, Error<B>> {
        match self.cur.as_mut() {
            Some(cur) => {
                let target = &mut self.buf[cur.start..];
                let nbytes = cmp::min(target.len(), payload.len());

                target[..nbytes].copy_from_slice(&payload[..nbytes]);
                cur.len = nbytes;

                self.buf
                    .encode_block(&cur.prev, &cur.next, EncodeOption::Skip(nbytes))?;

                Ok(nbytes)
            }
            None => Ok(0),
        }
    }

    /// Appends payload to the [current](Self::current) block.
    ///
    /// On success returns the number of bytes appended to the current block.
    /// If there is no current block or the block is full `0` is returned.
    ///
    /// **Note:** You need to call [`Self::flush()`] to save your changes!
    pub fn payload_add(&mut self, payload: &[u8]) -> Result<usize, Error<B>> {
        match self.cur.as_mut() {
            Some(cur) => {
                let target = &mut self.buf[cur.start + cur.len..];
                let nbytes = cmp::min(target.len(), payload.len());

                target[..nbytes].copy_from_slice(&payload[..nbytes]);
                cur.len += nbytes;

                self.buf
                    .encode_block(&cur.prev, &cur.next, EncodeOption::Skip(cur.len))?;

                Ok(nbytes)
            }
            None => Ok(0),
        }
    }

    /// Saves the changes of the [current](Self::current) block.
    pub fn flush(&mut self) -> Result<(), Error<B>> {
        match self.cur.as_ref() {
            Some(cur) => self.buf.write(&cur.id).map(|_| ()),
            None => Ok(()),
        }
    }
}

impl<B: Backend> AsRef<Container<B>> for Inner<B> {
    fn as_ref(&self) -> &Container<B> {
        &self.buf.container
    }
}

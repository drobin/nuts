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

use std::convert::TryInto;

use crate::buffer::Buffer;
use crate::buffer::BufferError;
use crate::buffer::BufferMut;
use crate::header::{rev0, HeaderError};

const MAGIC: [u8; 7] = *b"nuts-io";

#[derive(Debug)]
pub enum Revision {
    Rev0(rev0::Data),
}

impl Revision {
    pub fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Revision, HeaderError> {
        let b = buf.get_u32()?;

        match b {
            0 => rev0::Data::get_from_buffer(buf).map(Revision::Rev0),
            _ => Err(BufferError::InvalidIndex("Revision".to_string(), b).into()),
        }
    }

    pub fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        match self {
            Revision::Rev0(data) => {
                buf.put_u32(0)?;
                data.put_into_buffer(buf)?;

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Inner {
    magic: [u8; 7],
    pub rev: Revision,
}

impl Inner {
    pub fn new(rev: Revision) -> Inner {
        Inner { magic: MAGIC, rev }
    }

    pub fn get_from_buffer<T: Buffer>(buf: &mut T) -> Result<Inner, HeaderError> {
        let magic: [u8; 7] = buf.get_chunk(7)?.try_into().unwrap();

        if magic != MAGIC {
            return Err(HeaderError::InvalidHeader);
        }

        let rev = Revision::get_from_buffer(buf)?;

        Ok(Inner { magic, rev })
    }

    pub(crate) fn put_into_buffer<T: BufferMut>(&self, buf: &mut T) -> Result<(), HeaderError> {
        buf.put_chunk(&self.magic)?;
        self.rev.put_into_buffer(buf)?;

        Ok(())
    }
}

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

use std::convert::TryInto;
use std::mem;
use std::num::TryFromIntError;
use thiserror::Error;

/// Errors while (de-) serializing binary data.
#[derive(Debug, Error)]
pub enum BufferError {
    #[error("unexpected eof")]
    UnexpectedEof,

    #[error("write zero")]
    WriteZero,

    #[error("invalid usize: {0}")]
    InvalidUsize(TryFromIntError),

    #[error("no {0} at {1}")]
    InvalidIndex(String, u32),
}

macro_rules! get_func {
    ($name:ident, $ty:ty) => {
        fn $name(&mut self) -> Result<$ty, BufferError> {
            self.get_chunk(mem::size_of::<$ty>())
                .map(|buf| <$ty>::from_be_bytes(buf.try_into().unwrap()))
        }
    };
}

pub trait Buffer: Sized {
    fn get_chunk(&mut self, len: usize) -> Result<&[u8], BufferError>;

    get_func!(get_u8, u8);
    get_func!(get_u16, u16);
    get_func!(get_u32, u32);
    get_func!(get_u64, u64);

    fn get_usize(&mut self) -> Result<usize, BufferError> {
        self.get_u64()?
            .try_into()
            .map_err(BufferError::InvalidUsize)
    }

    fn get_vec(&mut self) -> Result<Vec<u8>, BufferError> {
        let len = self.get_usize()?;

        self.get_chunk(len).map(|buf| buf.to_vec())
    }
}

pub trait BufferMut: Sized {
    fn put_chunk(&mut self, buf: &[u8]) -> Result<(), BufferError>;

    fn put_u8(&mut self, value: u8) -> Result<(), BufferError> {
        self.put_chunk(&value.to_be_bytes())
    }

    fn put_u16(&mut self, value: u16) -> Result<(), BufferError> {
        self.put_chunk(&value.to_be_bytes())
    }

    fn put_u32(&mut self, value: u32) -> Result<(), BufferError> {
        self.put_chunk(&value.to_be_bytes())
    }

    fn put_u64(&mut self, value: u64) -> Result<(), BufferError> {
        self.put_chunk(&value.to_be_bytes())
    }

    fn put_usize(&mut self, value: usize) -> Result<(), BufferError> {
        self.put_u64(value as u64)
    }

    fn put_vec(&mut self, buf: &[u8]) -> Result<(), BufferError> {
        self.put_usize(buf.len()).and_then(|()| self.put_chunk(buf))
    }
}

impl Buffer for &[u8] {
    fn get_chunk(&mut self, len: usize) -> Result<&[u8], BufferError> {
        if self.len() >= len {
            let buf = &self[..len];

            *self = &self[len..];

            Ok(buf)
        } else {
            Err(BufferError::UnexpectedEof)
        }
    }
}

impl BufferMut for Vec<u8> {
    fn put_chunk(&mut self, buf: &[u8]) -> Result<(), BufferError> {
        self.extend_from_slice(buf);
        Ok(())
    }
}

impl BufferMut for &mut [u8] {
    fn put_chunk(&mut self, buf: &[u8]) -> Result<(), BufferError> {
        if self.len() >= buf.len() {
            let target = &mut self[..buf.len()];

            target.copy_from_slice(buf);

            // Lifetime dance taken from `impl Write for &mut [u8]`.
            let (_, b) = mem::take(self).split_at_mut(buf.len());

            *self = b;

            Ok(())
        } else {
            Err(BufferError::WriteZero)
        }
    }
}

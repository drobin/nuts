// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

use crate::error::Error;
use crate::result::Result;
use crate::types::DiskType;

pub struct IO {
    pub bsize: u32,
    pub blocks: u64,
    pub ablocks: u64,
    pub dtype: DiskType,
}

impl IO {
    pub fn new(bsize: u32, blocks: u64, ablocks: u64, dtype: DiskType) -> IO {
        IO {
            bsize,
            blocks,
            ablocks,
            dtype,
        }
    }

    pub fn read<T>(&self, source: &mut T, target: &mut [u8], id: u64) -> Result<u32>
    where
        T: Read + Seek,
    {
        let len = std::cmp::min(target.len(), self.bsize as usize);
        let buf = target.get_mut(0..len).unwrap();

        self.seek(source, id)?;
        source
            .read_exact(buf)
            .or_else(|err| Err(Error::IoError(err)))?;

        Ok(len as u32)
    }

    pub fn write<T>(&self, source: &[u8], target: &mut T, id: u64) -> Result<u32>
    where
        T: Write + Seek,
    {
        let len = std::cmp::min(source.len(), self.bsize as usize);
        let remaining = self.bsize as usize - len;

        let buf = source.get(0..len).unwrap();
        let pad = &vec![0; remaining][..];
        let block = [buf, pad].concat();

        self.seek(target, id)?;

        let mut n = 0;

        while n < block.len() {
            n += target
                .write(&block[n..])
                .or_else(|err| Err(Error::IoError(err)))?;
        }

        Ok(n as u32)
    }

    fn seek<T>(&self, fd: &mut T, id: u64) -> Result<()>
    where
        T: Seek,
    {
        let pos = id * self.bsize as u64;
        let pos2 = fd
            .seek(SeekFrom::Start(pos))
            .or_else(|err| Err(Error::IoError(err)))?;

        if pos != pos2 {
            let err = std::io::Error::new(
                ErrorKind::UnexpectedEof,
                format!("failed to seek to position {}, is {}", pos, pos2),
            );
            Err(Error::IoError(err))
        } else {
            Ok(())
        }
    }
}

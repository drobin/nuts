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

use log::debug;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::error::Error;
use crate::header::Header;
use crate::rand::random;
use crate::result::Result;
use crate::types::{DiskType, Options, BLOCK_MIN_SIZE};

pub struct Inner {
    pub header: Header,
    pub ablocks: u64,
    fh: File,
}

impl Inner {
    pub fn create(
        path: &dyn AsRef<Path>,
        options: &Options,
        callback: Option<&impl Fn() -> Result<Vec<u8>>>,
    ) -> Result<Inner> {
        let header = Header::create(options)?;

        debug!("header: {:?}", header);

        let fh = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)?;
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        let offset = header.write(&mut buf, callback)?;
        let end = offset as usize;

        let mut inner = Inner {
            header,
            ablocks: 0,
            fh,
        };

        inner.write_block_unchecked(&buf[..end], 0, true)?;

        Ok(inner)
    }

    pub fn open(
        path: &dyn AsRef<Path>,
        callback: Option<&impl Fn() -> Result<Vec<u8>>>,
    ) -> Result<Inner> {
        let mut fh = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)?;

        // Create a temp. block with a size of BLOCK_MIN_SIZE bytes.
        // This is enough to read the header.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        fh.read_exact(&mut buf)?;

        let header = Header::read(&buf, callback).map(|(header, _)| header)?;
        debug!("header: {:?}", header);

        let pos = fh.seek(SeekFrom::End(0))?;
        let bsize = header.bsize;
        let ablocks = if bsize > 0 { pos / bsize as u64 } else { 0 };

        Ok(Inner {
            header,
            ablocks,
            fh,
        })
    }

    fn seek_block(&mut self, id: u64) -> Result<()> {
        let pos = id * self.header.bsize as u64;
        let pos2 = self.fh.seek(SeekFrom::Start(pos))?;

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

    pub fn read_block(&mut self, buf: &mut [u8], id: u64) -> Result<u32> {
        self.assert_id(id)?;

        let len = std::cmp::min(buf.len(), self.header.bsize as usize);
        let buf = buf.get_mut(0..len).unwrap();

        if id < self.ablocks {
            // Read an allocated block.
            // Seek to the related position and read the buffer.

            self.seek_block(id)?;
            self.fh.read_exact(buf)?;
        } else {
            // Read an existing but unallocated block.
            // Fill the target buffer with data which fits to the dtype.

            match self.header.dtype {
                DiskType::FatZero | DiskType::ThinZero => {
                    for e in buf.iter_mut() {
                        *e = 0;
                    }
                }
                DiskType::FatRandom | DiskType::ThinRandom => {
                    random(buf)?;
                }
            }
        }

        Ok(len as u32)
    }

    pub fn write_block(&mut self, buf: &[u8], id: u64) -> Result<u32> {
        self.write_block_unchecked(buf, id, false)
    }

    fn write_block_unchecked(&mut self, buf: &[u8], id: u64, allow_header: bool) -> Result<u32> {
        if !allow_header && id == 0 {
            let err =
                std::io::Error::new(ErrorKind::Other, String::from("cannot overwrite header"));
            return Err(Error::IoError(err));
        }

        self.assert_id(id)?;
        self.ensure_capacity(id + 1)?;

        let len = std::cmp::min(buf.len(), self.header.bsize as usize);
        let pad_len = self.header.bsize as usize - len;

        let buf = buf.get(0..len).unwrap();

        let pad = match self.header.dtype {
            DiskType::FatZero | DiskType::ThinZero => vec![0; pad_len],
            DiskType::FatRandom | DiskType::ThinRandom => {
                let mut rnd: Vec<u8> = vec![0; pad_len];
                random(&mut rnd[..])?;
                rnd
            }
        };

        let block = [buf, &pad[..]].concat();

        self.seek_block(id)?;
        self.fh.write_all(&block)?;

        debug!("block {} written, len = {}, pad = {}", id, len, pad.len());

        Ok(len as u32)
    }

    fn ensure_capacity(&mut self, blocks: u64) -> Result<()> {
        let blocks = std::cmp::min(blocks, self.header.blocks);

        match self.header.dtype {
            DiskType::FatZero | DiskType::FatRandom => {
                if blocks > 0 {
                    // Fat containers are extended to its size.
                    self.extend_container(self.header.blocks - self.ablocks)
                } else {
                    Ok(())
                }
            }
            DiskType::ThinZero | DiskType::ThinRandom => {
                if blocks > 0 && blocks > self.ablocks {
                    // This containers are extended to the requested block.
                    self.extend_container(blocks - self.ablocks)
                } else {
                    Ok(())
                }
            }
        }
    }

    fn extend_container(&mut self, count: u64) -> Result<()> {
        if count > 0 {
            debug!(
                "extending container by {} blocks, ablocks: {}",
                count, self.ablocks
            );
        }

        self.seek_block(self.ablocks)?;

        let mut data = vec![0; self.header.bsize as usize];

        for _i in 0..count {
            if self.header.dtype == DiskType::FatRandom || self.header.dtype == DiskType::ThinRandom
            {
                random(&mut data)?;
            }

            self.fh.write_all(&data)?;
            self.ablocks += 1;
        }

        if count > 0 {
            debug!(
                "container extended by {} blocks, ablocks: {}",
                count, self.ablocks
            );
        }

        Ok(())
    }

    fn assert_id(&self, id: u64) -> Result<()> {
        if id < self.header.blocks {
            Ok(())
        } else {
            let err =
                std::io::Error::new(ErrorKind::Other, format!("unable to locate block {}", id));
            Err(Error::IoError(err))
        }
    }
}

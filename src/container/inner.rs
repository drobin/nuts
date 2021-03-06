// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

use crate::error::Error;
use crate::header::Header;
use crate::password::PasswordStore;
use crate::rand::random;
use crate::result::Result;
use crate::types::{DiskType, Options, BLOCK_MIN_SIZE};

pub struct Inner<T: Read + Write + Seek> {
    pub header: Header,
    pub ablocks: u64,
    fh: T,
}

impl<T: Read + Write + Seek> Inner<T> {
    pub fn create(fh: T, options: Options, store: &mut PasswordStore) -> Result<Inner<T>> {
        let header = Header::create(options)?;

        debug!("header: {:?}", header);

        let mut inner = Inner {
            header,
            ablocks: 0,
            fh,
        };

        inner.flush_header(store)?;

        Ok(inner)
    }

    pub fn open(mut fh: T, store: &mut PasswordStore) -> Result<Inner<T>> {
        // Create a temp. block with a size of BLOCK_MIN_SIZE bytes.
        // This is enough to read the header.
        let mut buf = [0; BLOCK_MIN_SIZE as usize];
        fh.read_exact(&mut buf)?;

        let header = Header::read(&buf, store).map(|(header, _)| header)?;
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

    pub(crate) fn flush_header(&mut self, store: &mut PasswordStore) -> Result<()> {
        let mut buf = [0; BLOCK_MIN_SIZE as usize];

        let offset = self.header.write(&mut buf, store)?;
        let end = offset as usize;

        self.write_header_block(&buf[..end])?;

        Ok(())
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

        // The header block is always unencrypted, so you expect bsize bytes,
        // even if encryption is turned on (and bsize_net is smaller).
        let bsize = if id == 0 {
            self.header.bsize as usize
        } else {
            self.header.bsize_net() as usize
        };

        let len = std::cmp::min(buf.len(), bsize);
        let buf = buf.get_mut(0..len).unwrap();

        if id < self.ablocks {
            self.read_allocated(buf, id)
        } else {
            self.read_unallocated(buf)
        }
    }

    fn read_allocated(&mut self, buf: &mut [u8], id: u64) -> Result<u32> {
        // Read an allocated block.
        // Seek to the related position and read the buffer.
        // buf is already clipped at the block-size.

        self.seek_block(id)?;

        if id > 0 {
            let mut cipher_block = vec![0; self.header.bsize as usize];

            self.fh.read_exact(&mut cipher_block)?;

            let plain_block = self.header.cipher.decrypt(
                &cipher_block,
                &self.header.key_for(id),
                &self.header.iv_for(id),
            )?;

            buf.copy_from_slice(&plain_block.get(..buf.len()).unwrap());
        } else {
            self.fh.read_exact(buf)?;
        }

        Ok(buf.len() as u32)
    }

    fn read_unallocated(&mut self, buf: &mut [u8]) -> Result<u32> {
        // Read an existing but unallocated block.
        // Fill the target buffer with data which fits to the dtype.
        // buf is already clipped at the block-size.

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

        Ok(buf.len() as u32)
    }

    pub fn write_block(&mut self, buf: &[u8], id: u64) -> Result<u32> {
        if id == 0 {
            let err =
                std::io::Error::new(ErrorKind::Other, String::from("cannot overwrite header"));
            return Err(Error::IoError(err));
        }

        self.assert_id(id)?;
        self.ensure_capacity(id + 1)?;

        let bsize_net = self.header.bsize_net() as usize;
        let len = std::cmp::min(buf.len(), bsize_net);

        let buf = buf.get(0..len).unwrap();
        let pad = self.make_padding(bsize_net, len)?;
        let plain_block = [buf, &pad].concat();

        assert_eq!(plain_block.len(), bsize_net);

        let cipher_block = self.header.cipher.encrypt(
            &plain_block,
            &self.header.key_for(id),
            &self.header.iv_for(id),
        )?;

        self.seek_block(id)?;
        self.fh.write_all(&cipher_block)?;

        debug!("block {} written, len = {}, pad = {}", id, len, pad.len());

        Ok(len as u32)
    }

    fn write_header_block(&mut self, buf: &[u8]) -> Result<u32> {
        self.ensure_capacity(1)?;

        let bsize = self.header.bsize as usize;
        let len = std::cmp::min(buf.len(), bsize);

        let buf = buf.get(0..len).unwrap();
        let pad = self.make_padding(bsize, len)?;
        let block = [buf, &pad].concat();

        assert_eq!(block.len(), bsize);

        self.seek_block(0)?;
        self.fh.write_all(&block)?;

        debug!("header-block written, len = {}, pad = {}", len, pad.len());

        Ok(len as u32)
    }

    fn make_padding(&self, bsize: usize, len: usize) -> Result<Vec<u8>> {
        let pad_len = bsize - len;

        let pad = match self.header.dtype {
            DiskType::FatZero | DiskType::ThinZero => vec![0; pad_len],
            DiskType::FatRandom | DiskType::ThinRandom => {
                let mut rnd: Vec<u8> = vec![0; pad_len];
                random(&mut rnd[..])?;
                rnd
            }
        };

        Ok(pad)
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

impl<T: Read + Write + Seek> AsRef<T> for Inner<T> {
    fn as_ref(&self) -> &T {
        &self.fh
    }
}

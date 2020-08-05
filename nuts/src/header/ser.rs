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

use log::error;
use std::cmp;
use std::io::Read;

use crate::error::{Error, InvalHeaderKind};
use crate::io::{ReadBasics, ReadExt};
use crate::result::Result;
use crate::types::{Cipher, Digest, DiskType};
use crate::wkey::WrappingKeyData;

const MAGIC: [u8; 7] = [b'n', b'u', b't', b's', b'-', b'i', b'o'];

pub struct HeaderReader<'a> {
    pub data: &'a [u8],
    pub offs: usize,
}

impl<'a> HeaderReader<'a> {
    pub fn new(data: &[u8]) -> HeaderReader {
        HeaderReader { data, offs: 0 }
    }

    pub fn read_revision(&mut self) -> Result<u8> {
        let revision = self.read_u8()?;

        if revision == 1 {
            Ok(revision)
        } else {
            Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
        }
    }

    pub fn read_magic(&mut self) -> Result<()> {
        let magic = self.read_array(MAGIC.len() as u32)?;

        if magic == MAGIC {
            Ok(())
        } else {
            error!("invalid magic: {:x?}", magic);
            Err(Error::InvalHeader(InvalHeaderKind::InvalMagic))
        }
    }

    pub fn read_cipher(&mut self) -> Result<Cipher> {
        match self.read_u8()? {
            0 => Ok(Cipher::None),
            1 => Ok(Cipher::Aes128Ctr),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalCipher)),
        }
    }

    pub fn read_digest(&mut self) -> Result<Option<Digest>> {
        match self.read_u8()? {
            1 => Ok(Some(Digest::Sha1)),
            0xFF => Ok(None),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDigest)),
        }
    }

    pub fn read_dtype(&mut self) -> Result<DiskType> {
        match self.read_u8()? {
            0 => Ok(DiskType::FatZero),
            1 => Ok(DiskType::FatRandom),
            2 => Ok(DiskType::ThinZero),
            3 => Ok(DiskType::ThinRandom),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDiskType)),
        }
    }

    pub fn read_wrapping_key(&mut self) -> Result<Option<WrappingKeyData>> {
        match self.read_u8()? {
            1 => {
                let iterations = self.read_u32()?;
                let salt = self.read_vec()?;

                Ok(Some(WrappingKeyData::pbkdf2(iterations, &salt)))
            }
            0xFF => Ok(None),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalWrappingKey)),
        }
    }
}

impl<'a> Read for HeaderReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.data.len() - self.offs;
        let nbytes = cmp::min(buf.len(), remaining);

        let source = self.data.get(self.offs..self.offs + nbytes).unwrap();
        let target = buf.get_mut(..nbytes).unwrap();

        target.copy_from_slice(source);
        self.offs += nbytes;

        Ok(nbytes)
    }
}

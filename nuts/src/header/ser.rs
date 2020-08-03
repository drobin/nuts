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

use byteorder::{ByteOrder, NetworkEndian};
use log::error;
use std::io::Read;

use crate::error::{Error, InvalHeaderKind};
use crate::result::Result;
use crate::types::{Cipher, Digest, DiskType};
use crate::wkey::WrappingKeyData;

const MAGIC: [u8; 7] = [b'n', b'u', b't', b's', b'-', b'i', b'o'];

fn map_error<T>(err: std::io::Error) -> Result<T> {
    if err.kind() == std::io::ErrorKind::UnexpectedEof {
        Err(Error::NoData)
    } else {
        Err(Error::IoError(err))
    }
}

pub trait ReadHeader: Read {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];

        self.read_exact(&mut buf).or_else(map_error)?;
        Ok(buf[0])
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];

        self.read_exact(&mut buf).or_else(map_error)?;
        Ok(NetworkEndian::read_u32(&buf))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buf = [0; 8];

        self.read_exact(&mut buf).or_else(map_error)?;
        Ok(NetworkEndian::read_u64(&mut buf))
    }

    fn read_array(&mut self, size: u32) -> Result<Vec<u8>> {
        let mut arr = vec![0; size as usize];

        for elem in arr.iter_mut() {
            *elem = self.read_u8()?;
        }

        Ok(arr)
    }

    fn read_vec(&mut self) -> Result<Vec<u8>> {
        self.read_u32().and_then(|u| self.read_array(u))
    }

    fn read_revision(&mut self) -> Result<u8> {
        let revision = self.read_u8()?;

        if revision == 1 {
            Ok(revision)
        } else {
            Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
        }
    }

    fn read_magic(&mut self) -> Result<()> {
        let magic = self.read_array(MAGIC.len() as u32)?;

        if magic == MAGIC {
            Ok(())
        } else {
            error!("invalid magic: {:x?}", magic);
            Err(Error::InvalHeader(InvalHeaderKind::InvalMagic))
        }
    }

    fn read_cipher(&mut self) -> Result<Cipher> {
        match self.read_u8()? {
            0 => Ok(Cipher::None),
            1 => Ok(Cipher::Aes128Ctr),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalCipher)),
        }
    }

    fn read_digest(&mut self) -> Result<Option<Digest>> {
        match self.read_u8()? {
            1 => Ok(Some(Digest::Sha1)),
            0xFF => Ok(None),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDigest)),
        }
    }

    fn read_dtype(&mut self) -> Result<DiskType> {
        match self.read_u8()? {
            0 => Ok(DiskType::FatZero),
            1 => Ok(DiskType::FatRandom),
            2 => Ok(DiskType::ThinZero),
            3 => Ok(DiskType::ThinRandom),
            _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDiskType)),
        }
    }

    fn read_wrapping_key(&mut self) -> Result<Option<WrappingKeyData>> {
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

// The u8-slice get methods defined in `ReadHeader`.
impl ReadHeader for &[u8] {}

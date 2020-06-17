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

use crate::binary;
use crate::data::WrappingKeyData;
use crate::error::{Error, InvalHeaderKind};
use crate::result::Result;
use crate::types::{Cipher, Digest, WrappingKey};

const MAGIC: [u8; 7] = [b'n', b'u', b't', b's', b'-', b'i', b'o'];

#[derive(Debug)]
pub struct Header {
    pub revision: u8,
    pub cipher: Cipher,
    pub digest: Option<Digest>,
    pub wrapping_key: Option<WrappingKeyData>,
    pub hmac: Option<Vec<u8>>,
    pub secret: Option<Vec<u8>>,
}

impl Header {
    pub fn read(source: &[u8], offset: &mut u32) -> Result<Header> {
        binary::read_vec_as(source, offset, 7, validate_magic)?;

        let header = Header {
            revision: binary::read_u8_as(source, offset, u8_to_revision)?,
            cipher: binary::read_u8_as(source, offset, u8_to_cipher)?,
            digest: binary::read_u8_as(source, offset, u8_to_digest)?,
            wrapping_key: read_wrapping_key(source, offset)?,
            hmac: read_vec(source, offset)?,
            secret: read_vec(source, offset)?,
        };

        Ok(header)
    }
}

fn validate_magic(slice: &[u8]) -> Result<()> {
    if slice == MAGIC {
        Ok(())
    } else {
        Err(Error::InvalHeader(InvalHeaderKind::InvalMagic))
    }
}

fn u8_to_revision(revision: u8) -> Result<u8> {
    if revision == 1 {
        Ok(revision)
    } else {
        Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
    }
}

fn u8_to_cipher(i: u8) -> Result<Cipher> {
    match i {
        0 => Ok(Cipher::None),
        1 => Ok(Cipher::Aes128Ctr),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalCipher)),
    }
}

fn u8_to_digest(i: u8) -> Result<Option<Digest>> {
    match i {
        1 => Ok(Some(Digest::Sha1)),
        0xFF => Ok(None),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDigest)),
    }
}

fn read_wrapping_key(data: &[u8], offset: &mut u32) -> Result<Option<WrappingKeyData>> {
    let algorithm = binary::read_u8(data, offset)?;

    match algorithm {
        1 => Ok(read_wrapping_key_pbkdf2(data, offset)?),
        0xFF => Ok(None),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalWrappingKey)),
    }
}

fn read_wrapping_key_pbkdf2(data: &[u8], offset: &mut u32) -> Result<Option<WrappingKeyData>> {
    let iterations = binary::read_u32(data, offset)?;
    let salt_len = binary::read_u32(data, offset)?;
    let salt = binary::read_vec(data, offset, salt_len)?;

    let key = WrappingKey::Pbkdf2 {
        iterations,
        salt_len,
    };

    Ok(Some(WrappingKeyData::Pbkdf2Data {
        salt: salt.to_vec(),
        key,
    }))
}

fn read_vec(data: &[u8], offset: &mut u32) -> Result<Option<Vec<u8>>> {
    let size = binary::read_u32(data, offset)?;
    let vec = binary::read_vec(data, offset, size)?;
    Ok(Some(vec.to_vec()))
}

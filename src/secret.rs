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

use std::fmt;
use std::ops;

use crate::binary;
use crate::error::{Error, InvalHeaderKind};
use crate::result::Result;
use crate::types::{DiskType, BLOCK_MIN_SIZE};

macro_rules! reset_slice {
    ($buf:expr) => {
        for e in $buf.iter_mut() {
            *e = 0;
        }
    };
}

pub struct Secret {
    pub dtype: DiskType,
    pub bsize: u32,
    pub blocks: u64,
    pub master_key: Vec<u8>,
    pub master_iv: Vec<u8>,
    pub hmac_key: Vec<u8>,
    pub userdata: Vec<u8>,
}

impl Secret {
    pub fn new() -> Secret {
        Secret {
            dtype: DiskType::FatRandom,
            bsize: BLOCK_MIN_SIZE,
            blocks: 0,
            master_key: vec![],
            master_iv: vec![],
            hmac_key: vec![],
            userdata: vec![],
        }
    }

    pub fn read(source: &[u8]) -> Result<(Secret, u32)> {
        let mut offset = 0;

        let dtype = binary::read_u8_as(source, &mut offset, u8_to_disk_type)?;
        let bsize = binary::read_u32_as(source, &mut offset, validate_block_size)?;
        let blocks = binary::read_u64(source, &mut offset)?;
        let master_key = binary::read_vec(source, &mut offset)?;
        let master_iv = binary::read_vec(source, &mut offset)?;
        let hmac_key = binary::read_vec(source, &mut offset)?;
        let userdata = binary::read_vec(source, &mut offset)?;

        let secret = Secret {
            dtype,
            bsize,
            blocks,
            master_key,
            master_iv,
            hmac_key,
            userdata,
        };

        Ok((secret, offset))
    }

    pub fn write(&self, target: &mut [u8]) -> Result<u32> {
        let mut offset = 0;

        binary::write_u8_as(target, &mut offset, self.dtype, disk_type_to_u8)?;
        binary::write_u32(target, &mut offset, self.bsize)?;
        binary::write_u64(target, &mut offset, self.blocks)?;
        binary::write_vec(target, &mut offset, &self.master_key)?;
        binary::write_vec(target, &mut offset, &self.master_iv)?;
        binary::write_vec(target, &mut offset, &self.hmac_key)?;
        binary::write_vec(target, &mut offset, &self.userdata)?;

        Ok(offset)
    }

    fn zero(&mut self) {
        reset_slice!(self.master_key);
        reset_slice!(self.master_iv);
        reset_slice!(self.hmac_key);
    }
}

impl ops::Drop for Secret {
    fn drop(&mut self) {
        self.zero();
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let master_key = format!("<{} bytes>", self.master_key.len());
        let master_iv = format!("<{} bytes>", self.master_iv.len());
        let hmac_key = format!("<{} bytes>", self.hmac_key.len());
        let userdata = format!("<{} bytes>", self.userdata.len());

        fmt.debug_struct("Secret")
            .field("dtype", &self.dtype)
            .field("bsize", &self.bsize)
            .field("blocks", &self.blocks)
            .field("master_key", &master_key)
            .field("master_iv", &master_iv)
            .field("hmac_key", &hmac_key)
            .field("userdata", &userdata)
            .finish()
    }
}

fn u8_to_disk_type(i: u8) -> Result<DiskType> {
    match i {
        0 => Ok(DiskType::FatZero),
        1 => Ok(DiskType::FatRandom),
        2 => Ok(DiskType::ThinZero),
        3 => Ok(DiskType::ThinRandom),
        _ => Err(Error::InvalHeader(InvalHeaderKind::InvalDiskType)),
    }
}

fn disk_type_to_u8(dtype: DiskType) -> Result<u8> {
    match dtype {
        DiskType::FatZero => Ok(0),
        DiskType::FatRandom => Ok(1),
        DiskType::ThinZero => Ok(2),
        DiskType::ThinRandom => Ok(3),
    }
}

fn validate_block_size(i: u32) -> Result<u32> {
    if i >= BLOCK_MIN_SIZE && i % BLOCK_MIN_SIZE == 0 {
        Ok(i)
    } else {
        Err(Error::InvalHeader(InvalHeaderKind::InvalBlockSize))
    }
}

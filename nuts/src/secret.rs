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
use std::fmt;

use crate::binary;
use crate::error::{Error, InvalHeaderKind};
use crate::rand::random;
use crate::result::Result;
use crate::types::{Cipher, Digest, DiskType, Options, BLOCK_MIN_SIZE};
use crate::utils::SecureVec;

pub struct Secret {
    pub dtype: DiskType,
    pub bsize: u32,
    pub blocks: u64,
    pub master_key: SecureVec<u8>,
    pub master_iv: SecureVec<u8>,
    pub hmac_key: SecureVec<u8>,
    pub userdata: Vec<u8>,
}

impl Secret {
    pub fn create(options: &Options) -> Result<Secret> {
        let key_size = options.cipher.key_size() as usize;
        let iv_size = options.cipher.iv_size() as usize;
        let hmac_size = options.md.map_or_else(|| 0, |d| d.size()) as usize;

        let mut secret = Secret {
            dtype: options.dtype,
            bsize: options.bsize(),
            blocks: options.blocks(),
            master_key: secure_vec![0; key_size],
            master_iv: secure_vec![0; iv_size],
            hmac_key: secure_vec![0; hmac_size],
            userdata: vec![],
        };

        random(&mut secret.master_key)?;
        random(&mut secret.master_iv)?;
        random(&mut secret.hmac_key)?;

        Ok(secret)
    }

    pub fn read(source: &[u8]) -> Result<(Secret, u32)> {
        let mut offset = 0;

        let dtype = binary::read_u8_as(source, &mut offset, u8_to_disk_type)?;
        let bsize = binary::read_u32(source, &mut offset)?;
        let blocks = binary::read_u64(source, &mut offset)?;
        let master_key = binary::read_vec(source, &mut offset)?;
        let master_iv = binary::read_vec(source, &mut offset)?;
        let hmac_key = binary::read_vec(source, &mut offset)?;
        let userdata = binary::read_vec(source, &mut offset)?;

        let secret = Secret {
            dtype,
            bsize,
            blocks,
            master_key: SecureVec::new(master_key),
            master_iv: SecureVec::new(master_iv),
            hmac_key: SecureVec::new(hmac_key),
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

    pub fn validate(&self, cipher: Cipher, digest: Option<Digest>) -> Result<()> {
        if cipher == Cipher::None && digest.is_some() {
            let message = format!("digest cannot be {} for cipher {}", digest.unwrap(), cipher);
            return Err(Error::InvalArg(message));
        }

        if cipher != Cipher::None && digest.is_none() {
            let message = format!("digest cannot be None for cipher {}", cipher);
            return Err(Error::InvalArg(message));
        }

        self.validate_block_size()?;
        self.validate_blocks()?;
        self.validate_master_key(cipher)?;
        self.validate_master_iv(cipher)?;
        self.validate_hmac_key(digest)?;

        Ok(())
    }

    fn validate_block_size(&self) -> Result<()> {
        if self.bsize >= BLOCK_MIN_SIZE && self.bsize % BLOCK_MIN_SIZE == 0 {
            Ok(())
        } else {
            error!("invalid block size: {}", self.bsize);
            Err(Error::InvalHeader(InvalHeaderKind::InvalBlockSize))
        }
    }

    fn validate_blocks(&self) -> Result<()> {
        if self.blocks >= 1 {
            Ok(())
        } else {
            error!("invalid number of blocks: {}", self.blocks);
            Err(Error::InvalHeader(InvalHeaderKind::InvalBlocks))
        }
    }

    fn validate_master_key(&self, cipher: Cipher) -> Result<()> {
        if self.master_key.len() != cipher.key_size() as usize {
            error!(
                "invalid master key, len: {}, expected: {} ({})",
                self.master_key.len(),
                cipher.key_size(),
                cipher
            );
            Err(Error::InvalHeader(InvalHeaderKind::InvalMasterKey))
        } else {
            Ok(())
        }
    }

    fn validate_master_iv(&self, cipher: Cipher) -> Result<()> {
        if self.master_iv.len() != cipher.iv_size() as usize {
            error!(
                "invalid master iv, len: {}, expected: {} ({})",
                self.master_iv.len(),
                cipher.iv_size(),
                cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalMasterIv))
        } else {
            Ok(())
        }
    }

    fn validate_hmac_key(&self, digest: Option<Digest>) -> Result<()> {
        let size = match digest {
            Some(md) => md.size() as usize,
            None => 0,
        };

        if self.hmac_key.len() != size {
            error!(
                "invalid hmac key, len: {}, expected: {} ({})",
                self.hmac_key.len(),
                size,
                digest_to_string(digest)
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalHmacKey))
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (master_key, master_iv, hmac_key, userdata) =
            if cfg!(feature = "debug-plain-keys") && cfg!(debug_assertions) {
                (
                    format!("{:?}", self.master_key),
                    format!("{:?}", self.master_iv),
                    format!("{:?}", self.hmac_key),
                    format!("{:?}", self.userdata),
                )
            } else {
                (
                    format!("<{} bytes>", self.master_key.len()),
                    format!("<{} bytes>", self.master_iv.len()),
                    format!("<{} bytes>", self.hmac_key.len()),
                    format!("<{} bytes>", self.userdata.len()),
                )
            };

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

fn digest_to_string(digest: Option<Digest>) -> String {
    match digest {
        Some(md) => format!("{}", md),
        None => String::from("None"),
    }
}

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

pub(crate) mod ser;

use ::openssl::memcmp;
use ::openssl::pkey::PKey;
use ::openssl::sign::Signer;
use log::{debug, error};
use std::fmt;

use crate::error::{Error, InvalHeaderKind};
use crate::header::ser::{HeaderReader, HeaderWriter};
use crate::io::{ReadBasics, ReadExt, WriteBasics, WriteExt};
use crate::rand::random;
use crate::result::Result;
use crate::types::{Cipher, Digest, DiskType, Options, WrappingKey, BLOCK_MIN_SIZE};
use crate::utils::SecureVec;

pub struct Header {
    pub revision: u8,
    pub cipher: Cipher,
    pub digest: Option<Digest>,
    pub wrapping_key: Option<WrappingKey>,
    pub wrapping_iv: Vec<u8>,
    pub dtype: DiskType,           // part of secret
    pub bsize: u32,                // part of secret
    pub blocks: u64,               // part of secret
    pub master_key: SecureVec<u8>, // part of secret
    pub master_iv: SecureVec<u8>,  // part of secret
    pub hmac_key: SecureVec<u8>,   // part of secret
    pub userdata: Vec<u8>,         // part of secret
}

impl Header {
    fn new() -> Header {
        Header {
            revision: 0,
            cipher: Cipher::None,
            digest: None,
            wrapping_key: None,
            wrapping_iv: vec![],
            dtype: DiskType::FatZero,
            bsize: 0,
            blocks: 0,
            master_key: secure_vec![],
            master_iv: secure_vec![],
            hmac_key: secure_vec![],
            userdata: vec![],
        }
    }

    pub fn create(options: &Options) -> Result<Header> {
        let key_size = options.cipher.key_size() as usize;
        let iv_size = options.cipher.iv_size() as usize;
        let hmac_size = options.md.map_or_else(|| 0, |d| d.size()) as usize;

        let mut wrapping_iv = vec![0; iv_size];
        let mut master_key = secure_vec![0; key_size];
        let mut master_iv = secure_vec![0; iv_size];

        random(&mut wrapping_iv)?;
        random(&mut master_key)?;
        random(&mut master_iv)?;

        Ok(Header {
            revision: 1,
            cipher: options.cipher,
            digest: options.md,
            wrapping_key: options.wkey.clone(),
            wrapping_iv,
            dtype: options.dtype,
            bsize: options.bsize(),
            blocks: options.blocks(),
            master_key,
            master_iv,
            hmac_key: secure_vec![0; hmac_size],
            userdata: vec![],
        })
    }

    pub fn read(
        source: &[u8],
        callback: Option<impl Fn() -> Result<Vec<u8>>>,
    ) -> Result<(Header, u32)> {
        let mut header = Header::new();

        let (hmac, cipher_secret, offset) = header.read_header(source)?;

        // Let's validate the header (except secret),
        // so you can create the wrapping key.
        header.validate(false)?;

        let mut plain_secret = secure_vec![0; cipher_secret.len()];
        let wrapping_key = header.create_wrapping_key(callback)?;

        header.cipher.decrypt(
            &cipher_secret,
            &mut plain_secret,
            &wrapping_key,
            &header.wrapping_iv,
        )?;

        header.read_secret(&plain_secret)?;
        header.validate(true)?;
        header.verify_hmac(&plain_secret, &hmac)?;

        Ok((header, offset))
    }

    fn read_header(&mut self, source: &[u8]) -> Result<(Vec<u8>, Vec<u8>, u32)> {
        let mut reader = HeaderReader::new(source);

        reader.read_magic()?;
        self.revision = reader.read_revision()?;
        self.cipher = reader.read_cipher()?;
        self.digest = reader.read_digest()?;
        self.wrapping_key = reader.read_wrapping_key()?;
        self.wrapping_iv = reader.read_vec()?;

        let hmac = reader.read_vec()?;
        let secret = reader.read_vec()?;

        Ok((hmac, secret, reader.offs as u32))
    }

    fn read_secret(&mut self, source: &[u8]) -> Result<()> {
        let mut reader = HeaderReader::new(source);

        self.dtype = reader.read_dtype()?;
        self.bsize = reader.read_u32()?;
        self.blocks = reader.read_u64()?;
        self.master_key = SecureVec::new(reader.read_vec()?);
        self.master_iv = SecureVec::new(reader.read_vec()?);
        self.hmac_key = SecureVec::new(reader.read_vec()?);
        self.userdata = reader.read_vec()?;

        Ok(())
    }

    pub fn write(
        &self,
        target: &mut [u8],
        callback: Option<impl Fn() -> Result<Vec<u8>>>,
    ) -> Result<u32> {
        self.validate(true)?;

        let mut plain_secret = secure_vec![0; BLOCK_MIN_SIZE as usize];
        let secret_size = self.write_secret(&mut plain_secret)?;

        plain_secret.resize(secret_size, 0);

        let wrapping_key = self.create_wrapping_key(callback)?;
        let mut cipher_secret = vec![0; secret_size];

        self.cipher.encrypt(
            &plain_secret,
            &mut cipher_secret,
            &wrapping_key,
            &self.wrapping_iv,
        )?;

        let hmac = self.create_hmac(&plain_secret)?;
        Ok(self.write_header(target, &hmac, &cipher_secret)?)
    }

    fn write_header(&self, target: &mut [u8], hmac: &[u8], secret: &[u8]) -> Result<u32> {
        let mut writer = HeaderWriter::new(target);

        writer.write_magic()?;
        writer.write_revision(self.revision)?;
        writer.write_cipher(self.cipher)?;
        writer.write_digest(self.digest)?;
        writer.write_wrapping_key(self.wrapping_key.as_ref())?;
        writer.write_vec(&self.wrapping_iv)?;
        writer.write_vec(hmac)?;
        writer.write_vec(secret)?;

        Ok(writer.offs as u32)
    }

    fn write_secret(&self, target: &mut [u8]) -> Result<usize> {
        let mut writer = HeaderWriter::new(target);

        writer.write_dtype(self.dtype)?;
        writer.write_u32(self.bsize)?;
        writer.write_u64(self.blocks)?;
        writer.write_vec(&self.master_key)?;
        writer.write_vec(&self.master_iv)?;
        writer.write_vec(&self.hmac_key)?;
        writer.write_vec(&self.userdata)?;

        Ok(writer.offs)
    }

    fn create_wrapping_key(
        &self,
        callback: Option<impl Fn() -> Result<Vec<u8>>>,
    ) -> Result<SecureVec<u8>> {
        let wrapping_key = match self.wrapping_key.as_ref() {
            Some(wkey_data) => {
                let digest = self
                    .digest
                    .ok_or(Error::InvalHeader(InvalHeaderKind::InvalDigest))?;
                let callback = callback.ok_or(Error::NoPassword)?;
                let password = SecureVec::new((callback)()?);
                wkey_data.create_wrapping_key(&password, digest)?
            }
            None => secure_vec![],
        };

        debug!("wrapping_key calculated, {} bytes", wrapping_key.len());

        Ok(wrapping_key)
    }

    fn validate(&self, include_secure: bool) -> Result<()> {
        self.validate_revision()?;
        self.validate_digest()?;
        self.validate_wrapping_key_data()?;
        self.validate_wrapping_iv()?;

        if include_secure {
            self.validate_bsize()?;
            self.validate_blocks()?;
            self.validate_master_key()?;
            self.validate_master_iv()?;
            self.validate_hmac_key()?;
        }

        Ok(())
    }

    fn create_hmac(&self, plain_secret: &[u8]) -> Result<Vec<u8>> {
        if let Some(md) = self.digest {
            let pkey = PKey::hmac(&self.hmac_key)?;
            let mut signer = Signer::new(md.to_openssl(), &pkey)?;

            let mut hmac = vec![0; md.size() as usize];
            let len = signer.sign_oneshot(&mut hmac, plain_secret)?;
            assert_eq!(len, md.size() as usize);

            debug!("HMAC created, {} bytes", md.size());

            Ok(hmac)
        } else {
            debug!("HMAC creation skipped");
            Ok(vec![])
        }
    }

    fn verify_hmac(&self, plain_secret: &[u8], hmac: &[u8]) -> Result<()> {
        if let Some(md) = self.digest {
            if hmac.len() != md.size() as usize {
                error!(
                    "invalid hmac, len: {}, expected: {} ({:?})",
                    hmac.len(),
                    md.size(),
                    md
                );

                return Err(Error::InvalHeader(InvalHeaderKind::InvalHmac));
            }

            let pkey = PKey::hmac(&self.hmac_key)?;
            let mut signer = Signer::new(md.to_openssl(), &pkey)?;

            let calculated_hmac = signer.sign_oneshot_to_vec(plain_secret)?;

            if memcmp::eq(&calculated_hmac, hmac) {
                debug!("HMAC verified");
                Ok(())
            } else {
                Err(Error::HmacMismatch)
            }
        } else {
            debug!("HMAC verification skipped");
            Ok(())
        }
    }

    fn validate_revision(&self) -> Result<()> {
        if self.revision == 1 {
            Ok(())
        } else {
            error!("invalid revision: {}", self.revision);
            Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
        }
    }

    fn validate_digest(&self) -> Result<()> {
        if self.cipher == Cipher::None && self.digest.is_some() {
            error!(
                "invalid digest {:?} for cipher {:?}",
                self.digest.unwrap(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalDigest))
        } else if self.cipher != Cipher::None && self.digest.is_none() {
            error!("invalid digest None for cipher {:?}", self.cipher);
            Err(Error::InvalHeader(InvalHeaderKind::InvalDigest))
        } else {
            Ok(())
        }
    }

    fn validate_wrapping_key_data(&self) -> Result<()> {
        if self.cipher == Cipher::None && self.wrapping_key.is_some() {
            error!(
                "invalid wrapping key data {:?} for cipher {:?}",
                self.wrapping_key.as_ref().unwrap(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalWrappingKey))
        } else if self.cipher != Cipher::None && self.wrapping_key.is_none() {
            error!(
                "invalid wrapping key data None for cipher {:?}",
                self.cipher
            );
            Err(Error::InvalHeader(InvalHeaderKind::InvalWrappingKey))
        } else {
            Ok(())
        }
    }

    fn validate_wrapping_iv(&self) -> Result<()> {
        if self.wrapping_iv.len() != self.cipher.iv_size() as usize {
            error!(
                "invalid iv, len: {}, expected: {} ({:?})",
                self.wrapping_iv.len(),
                self.cipher.iv_size(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalIv))
        } else {
            Ok(())
        }
    }

    fn validate_bsize(&self) -> Result<()> {
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

    fn validate_master_key(&self) -> Result<()> {
        if self.master_key.len() != self.cipher.key_size() as usize {
            error!(
                "invalid master key, len: {}, expected: {} ({:?})",
                self.master_key.len(),
                self.cipher.key_size(),
                self.cipher
            );
            Err(Error::InvalHeader(InvalHeaderKind::InvalMasterKey))
        } else {
            Ok(())
        }
    }

    fn validate_master_iv(&self) -> Result<()> {
        if self.master_iv.len() != self.cipher.iv_size() as usize {
            error!(
                "invalid master iv, len: {}, expected: {} ({:?})",
                self.master_iv.len(),
                self.cipher.iv_size(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalMasterIv))
        } else {
            Ok(())
        }
    }

    fn validate_hmac_key(&self) -> Result<()> {
        let size = match self.digest {
            Some(md) => md.size() as usize,
            None => 0,
        };

        if self.hmac_key.len() != size {
            error!(
                "invalid hmac key, len: {}, expected: {} ({:?})",
                self.hmac_key.len(),
                size,
                self.digest
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalHmacKey))
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (wrapping_iv, master_key, master_iv, hmac_key, userdata) =
            if cfg!(feature = "debug-plain-keys") && cfg!(debug_assertions) {
                (
                    format!("{:?}", self.wrapping_iv),
                    format!("{:?}", self.master_key),
                    format!("{:?}", self.master_iv),
                    format!("{:?}", self.hmac_key),
                    format!("{:?}", self.userdata),
                )
            } else {
                (
                    format!("<{} bytes>", self.wrapping_iv.len()),
                    format!("<{} bytes>", self.master_key.len()),
                    format!("<{} bytes>", self.master_iv.len()),
                    format!("<{} bytes>", self.hmac_key.len()),
                    format!("<{} bytes>", self.userdata.len()),
                )
            };

        fmt.debug_struct("Header")
            .field("revision", &self.revision)
            .field("cipher", &self.cipher)
            .field("digest", &self.digest)
            .field("wrapping_key", &self.wrapping_key)
            .field("wrapping_iv", &wrapping_iv)
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

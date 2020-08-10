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
use crate::io::{ReadExt, WriteExt};
use crate::rand::random;
use crate::result::Result;
use crate::secret::Secret;
use crate::types::{Cipher, Digest, Options, WrappingKey, BLOCK_MIN_SIZE};
use crate::wkey::WrappingKeyData;

pub struct Header {
    pub revision: u8,
    pub cipher: Cipher,
    pub digest: Option<Digest>,
    pub wrapping_key: Option<WrappingKeyData>,
    pub wrapping_iv: Vec<u8>,
    pub hmac: Vec<u8>,
    pub secret: Vec<u8>,
}

impl Header {
    pub fn create(options: &Options) -> Result<Header> {
        let wkey_data = match options.wkey {
            Some(WrappingKey::Pbkdf2 {
                iterations,
                salt_len,
            }) => {
                let mut salt = vec![0; salt_len as usize];
                random(&mut salt)?;

                Some(WrappingKeyData::pbkdf2(iterations, &salt))
            }
            None => None,
        };

        let mut iv = vec![0; options.cipher.iv_size() as usize];
        random(&mut iv)?;

        Ok(Header {
            revision: 1,
            cipher: options.cipher,
            digest: options.md,
            wrapping_key: wkey_data,
            wrapping_iv: iv,
            hmac: Vec::new(),
            secret: Vec::new(),
        })
    }

    pub fn read(source: &[u8]) -> Result<(Header, u32)> {
        let mut reader = HeaderReader::new(source);

        reader.read_magic()?;
        let revision = reader.read_revision()?;
        let cipher = reader.read_cipher()?;
        let digest = reader.read_digest()?;
        let wrapping_key = reader.read_wrapping_key()?;
        let wrapping_iv = reader.read_vec()?;
        let hmac = reader.read_vec()?;
        let secret = reader.read_vec()?;

        let header = Header {
            revision,
            cipher,
            digest,
            wrapping_key,
            wrapping_iv,
            hmac,
            secret,
        };

        Ok((header, reader.offs as u32))
    }

    pub fn read_secret(&self, wrapping_key: &[u8]) -> Result<(Secret, u32)> {
        let mut plain_secret = secure_vec![0; self.secret.len()];

        self.decrypt(&mut plain_secret, wrapping_key)?;

        let (secret, offset) = Secret::read(&plain_secret)?;
        self.verify_hmac(&secret, &plain_secret)?;

        Ok((secret, offset))
    }

    pub fn write(&self, target: &mut [u8]) -> Result<u32> {
        let mut writer = HeaderWriter::new(target);

        writer.write_magic()?;
        writer.write_revision(self.revision)?;
        writer.write_cipher(self.cipher)?;
        writer.write_digest(self.digest)?;
        writer.write_wrapping_key(self.wrapping_key.as_ref())?;
        writer.write_vec(&self.wrapping_iv)?;
        writer.write_vec(&self.hmac)?;
        writer.write_vec(&self.secret)?;

        Ok(writer.offs as u32)
    }

    pub fn write_secret(&mut self, secret: &Secret, wrapping_key: &[u8]) -> Result<u32> {
        let mut buf = secure_vec![0; BLOCK_MIN_SIZE as usize];
        let offset = secret.write(&mut buf)?;
        let plain_secret = &buf[..offset as usize];

        self.encrypt(plain_secret, wrapping_key)?;
        self.create_hmac(secret, plain_secret)?;

        Ok(offset)
    }

    fn encrypt(&mut self, plain_secret: &[u8], wrapping_key: &[u8]) -> Result<()> {
        self.secret.resize(plain_secret.len(), 0);
        self.cipher.encrypt(
            plain_secret,
            &mut self.secret,
            wrapping_key,
            &self.wrapping_iv,
        )
    }

    fn decrypt(&self, plain_secret: &mut [u8], wrapping_key: &[u8]) -> Result<()> {
        self.cipher
            .decrypt(&self.secret, plain_secret, wrapping_key, &self.wrapping_iv)
    }

    pub fn validate(&self) -> Result<()> {
        Header::validate_revision(self.revision)?;
        self.validate_digest()?;
        self.validate_wrapping_iv()?;
        self.validate_hmac()?;

        Ok(())
    }

    fn create_hmac(&mut self, secret: &Secret, plain_secret: &[u8]) -> Result<()> {
        if let Some(md) = self.digest {
            let pkey = PKey::hmac(&secret.hmac_key)?;
            let mut signer = Signer::new(md.to_openssl(), &pkey)?;

            self.hmac.resize(md.size() as usize, 0);
            let len = signer.sign_oneshot(&mut self.hmac, plain_secret)?;
            assert_eq!(len, md.size() as usize);

            debug!("HMAC created, {} bytes", md.size());
        } else {
            debug!("HMAC creation skipped");
        }

        Ok(())
    }

    fn verify_hmac(&self, secret: &Secret, plain_secret: &[u8]) -> Result<()> {
        if let Some(md) = self.digest {
            let pkey = PKey::hmac(&secret.hmac_key)?;
            let mut signer = Signer::new(md.to_openssl(), &pkey)?;

            let hmac = signer.sign_oneshot_to_vec(plain_secret)?;

            if memcmp::eq(&hmac, &self.hmac) {
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

    fn validate_revision(revision: u8) -> Result<()> {
        if revision == 1 {
            Ok(())
        } else {
            error!("invalid revision: {}", revision);
            Err(Error::InvalHeader(InvalHeaderKind::InvalRevision))
        }
    }

    fn validate_digest(&self) -> Result<()> {
        if self.cipher == Cipher::None && self.digest.is_some() {
            error!(
                "invalid digest {} for cipher {}",
                self.digest.unwrap(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalDigest))
        } else if self.cipher != Cipher::None && self.digest.is_none() {
            error!("invalid digest None for cipher {}", self.cipher);
            Err(Error::InvalHeader(InvalHeaderKind::InvalDigest))
        } else {
            Ok(())
        }
    }

    fn validate_wrapping_iv(&self) -> Result<()> {
        if self.wrapping_iv.len() != self.cipher.iv_size() as usize {
            error!(
                "invalid iv, len: {}, expected: {} ({})",
                self.wrapping_iv.len(),
                self.cipher.iv_size(),
                self.cipher
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalIv))
        } else {
            Ok(())
        }
    }

    fn validate_hmac(&self) -> Result<()> {
        let size = match self.digest {
            Some(md) => md.size() as usize,
            None => 0,
        };

        if self.hmac.len() != size {
            error!(
                "invalid hmac, len: {}, expected: {} ({})",
                self.hmac.len(),
                size,
                digest_to_string(self.digest)
            );

            Err(Error::InvalHeader(InvalHeaderKind::InvalHmac))
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (wrapping_iv, hmac, secret) =
            if cfg!(feature = "debug-plain-keys") && cfg!(debug_assertions) {
                (
                    format!("{:?}", self.wrapping_iv),
                    format!("{:?}", self.hmac),
                    format!("{:?}", self.secret),
                )
            } else {
                (
                    format!("<{} bytes>", self.wrapping_iv.len()),
                    format!("<{} bytes>", self.hmac.len()),
                    format!("<{} bytes>", self.secret.len()),
                )
            };

        fmt.debug_struct("Header")
            .field("revision", &self.revision)
            .field("cipher", &self.cipher)
            .field("digest", &self.digest)
            .field("wrapping_key", &self.wrapping_key)
            .field("wrapping_iv", &wrapping_iv)
            .field("hmac", &hmac)
            .field("secret", &secret)
            .finish()
    }
}

fn digest_to_string(digest: Option<Digest>) -> String {
    match digest {
        Some(md) => format!("{}", md),
        None => String::from("None"),
    }
}

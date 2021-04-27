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

use ::openssl::symm::{Crypter, Mode};
use log::error;
use openssl::pkcs5;
use openssl::{hash, symm};
use std::io::{self, Read, Write};
use std::{cmp, fmt};

use crate::error::{Error, InvalHeaderError};
use crate::io::{FromBinary, IntoBinary};
use crate::rand::random;
use crate::result::Result;
use crate::utils::SecureVec;

/// Supported cipher algorithms.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Cipher {
    /// No encryption
    None,

    /// AES with a 128-bit key in CTR mode
    Aes128Ctr,

    /// AES with a 128-bit key in GCM mode
    Aes128Gcm,
}

impl Cipher {
    /// Returns the key size of the cipher.
    pub fn key_size(&self) -> u32 {
        self.to_openssl()
            .map_or(0, |cipher| cipher.key_len() as u32)
    }

    /// Returns the IV size of the cipher.
    pub fn iv_size(&self) -> u32 {
        match self.to_openssl() {
            Some(cipher) => cipher.iv_len().unwrap_or(0) as u32,
            None => 0,
        }
    }

    /// Returns the block size of the cipher.
    pub fn block_size(&self) -> u32 {
        self.to_openssl()
            .map_or(1, |cipher| cipher.block_size() as u32)
    }

    /// Returns the tag size of the cipher.
    ///
    /// An AE-cipher results into a
    ///
    /// 1. ciphertext
    /// 2. tag
    ///
    /// Ciphertext and tag are both stored in a block of the container. Use
    /// this method to get the size of the tag. For a non-AE-cipher the
    /// tag-size is `0`.
    pub fn tag_size(&self) -> u32 {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 0,
            Cipher::Aes128Gcm => 16,
        }
    }

    pub(crate) fn encrypt(&self, input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let (input, _) = self.assert_input(input, Some(0))?;
        let mut output = vec![0; input.len()];

        if let Some(cipher) = self.to_openssl() {
            let mut crypter = Self::new_crypter(cipher, Mode::Encrypt, key, iv)?;

            let count = crypter.update(input, &mut output)?;
            assert_eq!(count, output.len());

            if self.tag_size() > 0 {
                let mut tag = vec![0; self.tag_size() as usize];

                crypter.finalize(&mut [])?;
                crypter.get_tag(&mut tag)?;

                output.extend(tag.iter());
            }
        } else {
            output.copy_from_slice(input);
        }

        Ok(output)
    }

    pub(crate) fn decrypt(&self, input: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
        let (input, tag) = self.assert_input(input, None)?;
        let mut output = vec![0; input.len()];

        if let Some(cipher) = self.to_openssl() {
            let mut crypter = Self::new_crypter(cipher, Mode::Decrypt, key, iv)?;

            let count = crypter.update(input, &mut output)?;
            assert_eq!(count, output.len());

            if self.tag_size() > 0 {
                crypter.set_tag(tag)?;
                crypter.finalize(&mut [])?;
            }
        } else {
            output.copy_from_slice(input);
        }

        Ok(output)
    }

    fn new_crypter(cipher: symm::Cipher, mode: Mode, key: &[u8], iv: &[u8]) -> Result<Crypter> {
        let key = Self::assert_key(&cipher, key)?;
        let iv = Self::assert_iv(&cipher, iv)?;

        let mut crypter = Crypter::new(cipher, mode, key, Some(iv))?;
        crypter.pad(false);

        Ok(crypter)
    }

    pub(crate) fn to_openssl(&self) -> Option<symm::Cipher> {
        match self {
            Cipher::Aes128Ctr => Some(symm::Cipher::aes_128_ctr()),
            Cipher::Aes128Gcm => Some(symm::Cipher::aes_128_gcm()),
            Cipher::None => None,
        }
    }

    fn assert_input<'a>(
        &self,
        buf: &'a [u8],
        tag_len: Option<usize>,
    ) -> Result<(&'a [u8], &'a [u8])> {
        let tag_len = tag_len.unwrap_or_else(|| self.tag_size() as usize);

        if buf.len() < tag_len {
            let msg = format!(
                "input too small, length: {}, needed: {}",
                buf.len(),
                tag_len
            );
            return Err(Error::InvalArg(msg));
        }

        let input_len = buf.len() - tag_len;

        if input_len % self.block_size() as usize != 0 {
            let msg = format!(
                "length of input {} mut be a multiple of block-size {}",
                buf.len(),
                self.block_size()
            );
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        } else {
            let input = &buf[..input_len];
            let tag = &buf[(buf.len() - tag_len)..];

            Ok((input, tag))
        }
    }

    fn assert_key<'a>(cipher: &symm::Cipher, key: &'a [u8]) -> Result<&'a [u8]> {
        key.get(..cipher.key_len()).ok_or_else(|| {
            let msg = format!(
                "key too short, at least {} bytes needed but got {}",
                cipher.key_len(),
                key.len()
            );
            error!("{}", msg);
            Error::InvalArg(msg)
        })
    }

    fn assert_iv<'a>(cipher: &symm::Cipher, iv: &'a [u8]) -> Result<&'a [u8]> {
        if let Some(len) = cipher.iv_len() {
            iv.get(..len).ok_or_else(|| {
                let msg = format!(
                    "iv too short, at least {} bytes needed but got {}",
                    len,
                    iv.len()
                );
                error!("{}", msg);
                Error::InvalArg(msg)
            })
        } else {
            panic!("no support for a cipher without iv");
        }
    }
}

impl FromBinary for Cipher {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        match u8::from_binary(r)? {
            0 => Ok(Cipher::None),
            1 => Ok(Cipher::Aes128Ctr),
            2 => Ok(Cipher::Aes128Gcm),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                InvalHeaderError::InvalCipher,
            )),
        }
    }
}

impl IntoBinary for Cipher {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            Cipher::None => 0u8,
            Cipher::Aes128Ctr => 1u8,
            Cipher::Aes128Gcm => 2u8,
        }
        .into_binary(w)
    }
}

/// Supported message digests.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Digest {
    /// SHA1
    Sha1,
}

impl Digest {
    /// Return the size of the message digest.
    ///
    /// This is the size of the resulting hash.
    pub fn size(&self) -> u32 {
        match self {
            Digest::Sha1 => 20,
        }
    }

    pub(crate) fn to_openssl(&self) -> hash::MessageDigest {
        match self {
            Digest::Sha1 => hash::MessageDigest::sha1(),
        }
    }
}

impl FromBinary for Digest {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        match u8::from_binary(r)? {
            1 => Ok(Digest::Sha1),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                InvalHeaderError::InvalDigest,
            )),
        }
    }
}

impl IntoBinary for Digest {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            Digest::Sha1 => 1u8,
        }
        .into_binary(w)
    }
}

/// Supported wrapping key algorithms.
///
/// Defines data used to calculate a wrapping key.
///
/// The wrapping key is created used by an algorithm defined as a variant of
/// this enum. The variants holds fields to customize the algorithm.
///
/// Based on a password provided by the user one of the algorithms are used to
/// calculate a wrapping key. The wrapping key then is used for encryption of
/// the secret in the header of the container.
#[derive(PartialEq)]
pub enum WrappingKey {
    /// PBKDF2
    Pbkdf2 {
        /// Digest used by PBKDF2.
        digest: Digest,

        /// Number of iterations used by PBKDF2.
        iterations: u32,

        /// A salt value used by PBKDF2.
        salt: Vec<u8>,
    },
}

impl WrappingKey {
    /// Creates a `WrappingKey` instance for the PBKDF2 algorithm.
    ///
    /// The `digest`, `iterations` and the `salt` values are used to customize
    /// the PBKDF2 algorithm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let WrappingKey::Pbkdf2 { digest, iterations, salt } =
    ///     WrappingKey::pbkdf2(Digest::Sha1, 5, &[1, 2, 3]);
    ///
    /// assert_eq!(digest, Digest::Sha1);
    /// assert_eq!(iterations, 5);
    /// assert_eq!(salt, [1, 2, 3]);
    /// ```
    pub fn pbkdf2(digest: Digest, iterations: u32, salt: &[u8]) -> WrappingKey {
        WrappingKey::Pbkdf2 {
            digest,
            iterations,
            salt: salt.to_vec(),
        }
    }

    /// Generates a `WrappingKey` instance for the PBKDF2 algorithm.
    ///
    /// The `digest`and `iterations` value is used to customize the PBKDF2
    /// algorithm. For the [`salt`] `salt_len` bytes of random data are
    /// generated.
    ///
    /// # Errors
    ///
    /// This method will return an [`Error::OpenSSL`] error if there was an
    /// error generating the random data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let WrappingKey::Pbkdf2 { digest, iterations, salt } =
    ///     WrappingKey::generate_pbkdf2(Digest::Sha1, 5, 3).unwrap();
    ///
    /// assert_eq!(digest, Digest::Sha1);
    /// assert_eq!(iterations, 5);
    /// assert_eq!(salt.len(), 3); // salt filled with random data
    /// ```
    ///
    /// [`salt`]: #variant.Pbkdf2.field.salt
    /// [`Error::OpenSSL`]: ../error/enum.Error.html#variant.OpenSSL
    pub fn generate_pbkdf2(digest: Digest, iterations: u32, salt_len: u32) -> Result<WrappingKey> {
        let mut salt = vec![0; salt_len as usize];
        random(&mut salt)?;

        Ok(WrappingKey::Pbkdf2 {
            digest,
            iterations,
            salt,
        })
    }

    pub(crate) fn create_wrapping_key(&self, password: &[u8]) -> Result<SecureVec<u8>> {
        if password.is_empty() {
            let msg = format!("invalid password, cannot be empty");
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let WrappingKey::Pbkdf2 {
            digest,
            iterations,
            salt,
        } = self;

        if salt.is_empty() {
            let msg = format!("invalid salt, cannot be empty");
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let hash = digest.to_openssl();
        let mut key = secure_vec![0; digest.size() as usize];

        pkcs5::pbkdf2_hmac(password, salt, *iterations as usize, hash, &mut key)?;

        Ok(key)
    }
}

impl Clone for WrappingKey {
    fn clone(&self) -> Self {
        let WrappingKey::Pbkdf2 {
            digest,
            iterations,
            salt,
        } = self;

        WrappingKey::Pbkdf2 {
            digest: digest.clone(),
            iterations: iterations.clone(),
            salt: salt.to_vec(),
        }
    }
}

impl fmt::Debug for WrappingKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let WrappingKey::Pbkdf2 {
            digest,
            iterations,
            salt,
        } = self;

        let salt = format!("<{} bytes>", salt.len());
        fmt.debug_struct("Pbkdf2")
            .field("digest", &digest)
            .field("iterations", &iterations)
            .field("salt", &salt)
            .finish()
    }
}

impl FromBinary for Option<WrappingKey> {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        match u8::from_binary(r)? {
            1 => {
                let digest = Digest::from_binary(r)?;
                let iterations = u32::from_binary(r)?;
                let salt = Vec::<u8>::from_binary(r)?;

                Ok(Some(WrappingKey::pbkdf2(digest, iterations, &salt)))
            }
            0xFF => Ok(None),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                InvalHeaderError::InvalWrappingKey,
            )),
        }
    }
}

impl IntoBinary for Option<WrappingKey> {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            Some(data) => {
                let WrappingKey::Pbkdf2 {
                    digest,
                    iterations,
                    salt,
                } = data;

                1u8.into_binary(w)?;
                digest.into_binary(w)?;
                iterations.into_binary(w)?;
                salt.into_binary(w)?;

                Ok(())
            }
            None => 0xFFu8.into_binary(w),
        }
    }
}

/// Container disk types.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DiskType {
    /// Space for the container is allocated once during creation of the
    /// container, unused blocks are initialized with all zeros.
    FatZero,

    /// Space for the container is allocated once during creation of the
    /// container, unused blocks are initialized with random data.
    FatRandom,

    /// Space for the container is allocated dynamically when needed, unused
    /// blocks are initialized with all zeros.
    ThinZero,

    /// Space for the container is allocated dynamically when needed, unused
    /// blocks are initialized with random data.
    ThinRandom,
}

impl FromBinary for DiskType {
    fn from_binary(r: &mut dyn Read) -> io::Result<Self> {
        match u8::from_binary(r)? {
            0 => Ok(DiskType::FatZero),
            1 => Ok(DiskType::FatRandom),
            2 => Ok(DiskType::ThinZero),
            3 => Ok(DiskType::ThinRandom),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                InvalHeaderError::InvalDiskType,
            )),
        }
    }
}

impl IntoBinary for DiskType {
    fn into_binary(&self, w: &mut dyn Write) -> io::Result<()> {
        match self {
            DiskType::FatZero => 0u8,
            DiskType::FatRandom => 1u8,
            DiskType::ThinZero => 2u8,
            DiskType::ThinRandom => 3u8,
        }
        .into_binary(w)
    }
}

/// The minimum size of a block.
pub const BLOCK_MIN_SIZE: u32 = 512;

const DEFAULT_CIPHER: Cipher = Cipher::Aes128Ctr;
const DEFAULT_DTYPE: DiskType = DiskType::FatRandom;
const DEFAULT_DIGEST: Digest = Digest::Sha1;
const DEFAULT_PBKDF2_ITERATIONS: u32 = 65536;
const DEFAULT_PBKDF2_SALT_LEN: u32 = 16;
const DEFAULT_BSIZE: u32 = BLOCK_MIN_SIZE;
const DEFAULT_BLOCKS: u64 = (1024 * 1024 / DEFAULT_BSIZE) as u64; // container of 1MB

/// Options to customize the creation of a container.
///
/// Use [`OptionsBuilder`] to create a new `Options` instance.
///
/// [`OptionsBuilder`]: struct.OptionsBuilder.html
#[derive(Debug)]
pub struct Options {
    pub(crate) dtype: DiskType,
    pub(crate) wkey: Option<WrappingKey>,
    pub(crate) cipher: Cipher,
    pub(crate) md: Option<Digest>,
    pub(crate) bsize: u32,
    pub(crate) blocks: u64,
}

impl Options {
    /// Returns the [`DiskType`] assigned to this `Options` instance.
    ///
    /// [`DiskType`]: enum.DiskType.html
    pub fn dtype(&self) -> DiskType {
        self.dtype
    }

    /// Returns the [`WrappingKey`] assigned to this `Options` instance.
    ///
    /// If encryption is enabled (the [cipher] is set to something other than
    /// [`Cipher::None`]), the wrapping key is wrapped into a [`Some`] value.
    /// If the [cipher] is set to [`Cipher::None`], no wrapping key is used and
    /// a [`None`] value is returned.
    ///
    /// [`WrappingKey`]: enum.WrappingKey.html
    /// [`Cipher::None`]: ../types/enum.Cipher.html#variant.None
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#Some.v
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#None.v
    /// [cipher]: #method.cipher
    pub fn wkey(&self) -> Option<&WrappingKey> {
        self.wkey.as_ref()
    }

    /// Returns the [`Cipher`] assigned to this `Options` instance.
    ///
    /// [`Cipher`]: ../types/enum.Cipher.html
    pub fn cipher(&self) -> Cipher {
        self.cipher
    }

    /// Returns the [`Digest`] assigned to this `Options` instance.
    ///
    /// If encryption is enabled (the [cipher] is set to something other than
    /// [`Cipher::None`]), the digest is wrapped into a [`Some`] value. If the
    /// [cipher] is set to [`Cipher::None`], no digest is used and a [`None`]
    /// value is returned.
    ///
    /// [`Digest`]: ../types/enum.Digest.html
    /// [`Cipher::None`]: ../types/enum.Cipher.html#variant.None
    /// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#Some.v
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#None.v
    /// [cipher]: #method.cipher
    pub fn digest(&self) -> Option<Digest> {
        self.md
    }

    /// Returns the block size.
    pub fn bsize(&self) -> u32 {
        self.bsize
    }

    /// Returns the number of blocks.
    pub fn blocks(&self) -> u64 {
        self.blocks
    }
}

/// A builder for [`Options`].
///
/// Use [`default()`] to create a `OptionsBuilder` populated with some default
/// values. The [`new()`] method creates a `OptionBuilder` with default
/// parameters for the given [`Cipher`].
///
/// There are plenty of `with_*` methods which are used to push custom
/// configuration into the builder. Finally, call [`build()`] to create an
/// [`Options`] instance.
///
/// [`Options`]: struct.Options.html
/// [`default()`]: #method.default
/// [`new()`]: #method.new
/// [`build()`]: #method.build
/// [`Cipher`]: enum.Cipher.html
#[derive(Debug)]
pub struct OptionsBuilder {
    dtype: DiskType,
    wkey: Option<WrappingKey>,
    cipher: Cipher,
    md: Option<Digest>,
    bsize: u32,
    blocks: Option<u64>,
    size: Option<u64>,
}

impl OptionsBuilder {
    /// Creates a set of defaults with the given `cipher`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = OptionsBuilder::new(Cipher::Aes128Ctr).build().unwrap();
    ///
    /// let WrappingKey::Pbkdf2 {
    ///     digest,
    ///     iterations,
    ///     salt,
    /// } = options.wkey().unwrap();
    ///
    /// assert_eq!(*digest, Digest::Sha1);
    /// assert_eq!(*iterations, 65536);
    /// assert_eq!(salt.len(), 16); // salt is filled with random data
    ///
    /// assert_eq!(options.cipher(), Cipher::Aes128Ctr);
    /// assert_eq!(options.digest(), Some(Digest::Sha1));
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = OptionsBuilder::new(Cipher::None).build().unwrap();
    ///
    /// assert_eq!(options.dtype(), DiskType::FatRandom);
    /// assert_eq!(options.wkey(), None);
    /// assert_eq!(options.cipher(), Cipher::None);
    /// assert_eq!(options.digest(), None);
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    pub fn new(cipher: Cipher) -> OptionsBuilder {
        OptionsBuilder {
            dtype: DEFAULT_DTYPE,
            wkey: None,
            cipher,
            md: None,
            bsize: DEFAULT_BSIZE,
            blocks: None,
            size: None,
        }
    }

    /// Creates a set of defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = OptionsBuilder::default().build().unwrap();
    ///
    /// let WrappingKey::Pbkdf2 {
    ///     digest,
    ///     iterations,
    ///     salt,
    /// } = options.wkey().unwrap();
    ///
    /// assert_eq!(*digest, Digest::Sha1);
    /// assert_eq!(*iterations, 65536);
    /// assert_eq!(salt.len(), 16); // salt is filled with random data
    ///
    /// assert_eq!(options.dtype(), DiskType::FatRandom);
    /// assert_eq!(options.cipher(), Cipher::Aes128Ctr);
    /// assert_eq!(options.digest(), Some(Digest::Sha1));
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    pub fn default() -> OptionsBuilder {
        Self::new(DEFAULT_CIPHER)
    }

    /// Creates an [`Options`] instance based on the values of this builder.
    ///
    /// # Errors
    ///
    /// This method will return an [`Error::InvalArg`] error if there is an
    /// invalid configuration item.
    ///
    /// [`Options`]: struct.Options.html
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    pub fn build(&self) -> Result<Options> {
        Ok(Options {
            dtype: self.dtype,
            cipher: self.cipher,
            md: self.build_digest(),
            wkey: self.build_wkey()?,
            bsize: self.build_bsize()?,
            blocks: self.build_blocks()?,
        })
    }

    /// Selects a new [`DiskType`].
    ///
    /// [`DiskType`]: enum.DiskType.html
    pub fn with_dtype(&mut self, dtype: DiskType) -> &mut Self {
        self.dtype = dtype;
        self
    }

    /// Selects a new [`WrappingKey`].
    ///
    /// **Note**: If encryption is disabled (the cipher is set to
    /// [`Cipher::None`]), the [`WrappingKey`] is ignored.
    ///
    /// [`WrappingKey`]: enum.WrappingKey.html
    /// [`Cipher::None`]: ../types/enum.Cipher.html#variant.None
    pub fn with_wkey(&mut self, wkey: WrappingKey) -> &mut Self {
        self.wkey = Some(wkey);
        self
    }

    fn build_wkey(&self) -> Result<Option<WrappingKey>> {
        if self.cipher != Cipher::None {
            let wkey = self.wkey.as_ref().map_or_else(
                || {
                    WrappingKey::generate_pbkdf2(
                        DEFAULT_DIGEST,
                        DEFAULT_PBKDF2_ITERATIONS,
                        DEFAULT_PBKDF2_SALT_LEN,
                    )
                },
                |wkey| Ok(wkey.clone()),
            );
            Ok(Some(wkey?))
        } else {
            Ok(None)
        }
    }

    /// Selects a new [`Digest`].
    ///
    /// **Note**: If encryption is disabled (the cipher is set to
    /// [`Cipher::None`]), the [`Digest`] is ignored.
    ///
    /// [`Digest`]: enum.Digest.html
    /// [`Cipher::None`]: ../types/enum.Cipher.html#variant.None
    pub fn with_digest(&mut self, digest: Digest) -> &mut Self {
        self.md = Some(digest);
        self
    }

    fn build_digest(&self) -> Option<Digest> {
        if self.cipher != Cipher::None {
            self.md.or(Some(DEFAULT_DIGEST))
        } else {
            None
        }
    }

    /// Selects a new block-size.
    ///
    /// The block size must be a multiple of [`BLOCK_MIN_SIZE`] bytes. You
    /// cannot have a block size less that [`BLOCK_MIN_SIZE`] bytes!
    ///
    /// This method accepts all values (even invalid). The final [`build()`]
    /// call will validate the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::OptionsBuilder;
    ///
    /// assert!(OptionsBuilder::default().with_bsize(1).build().is_err());
    /// assert!(OptionsBuilder::default().with_bsize(513).build().is_err());
    ///
    /// let options = OptionsBuilder::default().with_bsize(1024).build().unwrap();
    /// assert_eq!(options.bsize(), 1024);
    /// ```
    ///
    /// [`build()`]: #method.build
    /// [`BLOCK_MIN_SIZE`]: constant.BLOCK_MIN_SIZE.html
    pub fn with_bsize(&mut self, bsize: u32) -> &mut Self {
        self.bsize = bsize;
        self
    }

    fn build_bsize(&self) -> Result<u32> {
        if self.bsize < BLOCK_MIN_SIZE {
            let message = format!(
                "Invalid block size, got {} but must be at least {}.",
                self.bsize, BLOCK_MIN_SIZE
            );
            return Err(Error::InvalArg(message));
        }

        if self.bsize % BLOCK_MIN_SIZE != 0 {
            let message = format!(
                "Invalid block size, got {} but must be a multiple of {}.",
                self.bsize, BLOCK_MIN_SIZE
            );
            return Err(Error::InvalArg(message));
        }

        Ok(self.bsize)
    }

    /// Selects a new number of blocks.
    ///
    /// This is the number of blocks, which should be allocated for the
    /// container. It must be a greater than `0`.
    ///
    /// The product of the block size and the number of blocks specifies the
    /// size of the container.
    ///
    /// **Note**: Selecting a container size with [`with_size()`] has an higher
    /// priority than selecting the number of blocks with this method. It
    /// means, that a `with_blocks()` call is ignored, if you additionally call
    /// [`with_size()`].
    ///
    /// This method accepts all values (even invalid). The final [`build()`]
    /// call will validate the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::OptionsBuilder;
    ///
    /// assert!(OptionsBuilder::default().with_blocks(0).build().is_err());
    ///
    /// let options = OptionsBuilder::default().with_blocks(16).build().unwrap();
    /// assert_eq!(options.blocks(), 16);
    /// ```
    ///
    /// [`build()`]: #method.build
    /// [`with_size()`]: #method.with_size
    pub fn with_blocks(&mut self, blocks: u64) -> &mut Self {
        self.blocks = Some(blocks);
        self
    }

    /// Convenient method to select the number of blocks by specifying the
    /// container size.
    ///
    /// The method calculates the number of blocks based on the current block
    /// size and the given `size` argument. If `size` is not a multiple of the
    /// block size, then the size is rounded down to the nearest multiple. If
    /// `size` is less than the block size, then one block is created.
    ///
    /// **Note**: Selecting a container size with this method has an higher
    /// priority than selecting the number of blocks with [`with_blocks()`]. It
    /// means, that a [`with_blocks()`] call is ignored, if you additionally
    /// call `with_size()`.
    ///
    /// # Examples
    /// ```rust
    /// use nuts::types::OptionsBuilder;
    ///
    /// let options = OptionsBuilder::default().with_size(511).build().unwrap();
    /// assert_eq!(options.blocks(), 1);
    ///
    /// let options = OptionsBuilder::default().with_size(1024).build().unwrap();
    /// assert_eq!(options.blocks(), 2);
    ///
    /// let options = OptionsBuilder::default().with_size(1025).build().unwrap();
    /// assert_eq!(options.blocks(), 2);
    /// ```
    ///
    /// [block size]: #method.bsize
    /// [`with_blocks()`]: #method.with_blocks
    pub fn with_size(&mut self, size: u64) -> &mut Self {
        self.size = Some(size);
        self
    }

    fn build_blocks(&self) -> Result<u64> {
        let blocks = if let Some(size) = self.size {
            let bsize = self.build_bsize()?;
            cmp::max(size / bsize as u64, 1)
        } else {
            let blocks = self.blocks.unwrap_or(DEFAULT_BLOCKS);

            if blocks == 0 {
                let message = format!("Invalid number of blocks, got {}, expected > 0.", blocks);
                return Err(Error::InvalArg(message));
            }

            blocks
        };

        Ok(blocks)
    }
}

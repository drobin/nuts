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

use ::openssl::symm::{Crypter, Mode};
use log::error;
use openssl::pkcs5;
use openssl::{hash, symm};
use std::fmt;

use crate::error::Error;
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
}

impl Cipher {
    /// Returns the key size of the cipher.
    pub fn key_size(&self) -> u32 {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 16,
        }
    }

    /// Returns the IV size of the cipher.
    pub fn iv_size(&self) -> u32 {
        match self {
            Cipher::None => 0,
            Cipher::Aes128Ctr => 16,
        }
    }

    /// Returns the block size of the cipher.
    pub fn block_size(&self) -> u32 {
        match self {
            Cipher::None => 1,
            Cipher::Aes128Ctr => 1,
        }
    }

    /// Converts the given `str` into a [`Cipher`] variant.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `str` is not
    /// a valid cipher.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    /// use nuts::error::*;
    ///
    /// assert_eq!(Cipher::from_string("none").unwrap(), Cipher::None);
    /// assert_eq!(
    ///     Cipher::from_string("aes128-ctr").unwrap(),
    ///     Cipher::Aes128Ctr
    /// );
    ///
    /// let err = Cipher::from_string("xxx").unwrap_err();
    ///
    /// if let Error::InvalArg(msg) = err {
    ///     assert_eq!(msg, "invalid cipher: xxx");
    /// } else {
    ///     panic!("invalid error: {:?}", err);
    /// }
    /// ```
    ///
    /// [`Cipher`]: enum.Cipher.html
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    pub fn from_string(str: &str) -> Result<Cipher> {
        match str {
            "none" => Ok(Cipher::None),
            "aes128-ctr" => Ok(Cipher::Aes128Ctr),
            _ => {
                let message = format!("invalid cipher: {}", str);
                Err(Error::InvalArg(message))
            }
        }
    }

    pub(crate) fn encrypt(
        &self,
        input: &[u8],
        output: &mut [u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<()> {
        self.crypt(Mode::Encrypt, input, output, key, iv)
    }

    pub(crate) fn decrypt(
        &self,
        input: &[u8],
        output: &mut [u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<()> {
        self.crypt(Mode::Decrypt, input, output, key, iv)
    }

    fn crypt(
        &self,
        mode: Mode,
        input: &[u8],
        output: &mut [u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<()> {
        if let Some(cipher) = self.to_openssl() {
            Cipher::crypt_with_cipher(cipher, mode, input, output, key, iv)
        } else {
            assert_eq!(self, &Cipher::None);
            Cipher::crypt_none(input, output)
        }
    }

    fn crypt_with_cipher(
        cipher: symm::Cipher,
        mode: Mode,
        input: &[u8],
        output: &mut [u8],
        key: &[u8],
        iv: &[u8],
    ) -> Result<()> {
        if input.len() % cipher.block_size() != 0 {
            let msg = format!(
                "length of input {} mut be a multiple of block-size {}",
                input.len(),
                cipher.block_size()
            );
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let key = key.get(..cipher.key_len()).ok_or_else(|| {
            let msg = format!(
                "key too short, at least {} bytes needed but got {}",
                cipher.key_len(),
                key.len()
            );
            error!("{}", msg);
            Error::InvalArg(msg)
        })?;

        let iv = if let Some(len) = cipher.iv_len() {
            iv.get(..len).ok_or_else(|| {
                let msg = format!(
                    "iv too short, at least {} bytes needed but got {}",
                    len,
                    iv.len()
                );
                error!("{}", msg);
                Error::InvalArg(msg)
            })?
        } else {
            panic!("no support for a cipher without iv");
        };

        let mut crypter = Crypter::new(cipher, mode, key, Some(iv))?;
        crypter.pad(false);

        let count = crypter.update(input, output)?;
        assert_eq!(count, output.len());

        Ok(())
    }

    fn crypt_none(input: &[u8], output: &mut [u8]) -> Result<()> {
        output.copy_from_slice(input);
        Ok(())
    }

    pub(crate) fn to_openssl(&self) -> Option<symm::Cipher> {
        match self {
            Cipher::Aes128Ctr => Some(symm::Cipher::aes_128_ctr()),
            Cipher::None => None,
        }
    }
}

impl std::fmt::Display for Cipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cipher::None => write!(f, "none"),
            Cipher::Aes128Ctr => write!(f, "aes128-ctr"),
        }
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

    /// Converts the given `str` into a [`Digest`] variant.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `str` is not
    /// a valid digest.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    /// use nuts::error::*;
    ///
    /// assert_eq!(Digest::from_string("none").unwrap(), None);
    /// assert_eq!(Digest::from_string("sha1").unwrap(), Some(Digest::Sha1));
    ///
    /// let err = Digest::from_string("xxx").unwrap_err();
    ///
    /// if let Error::InvalArg(msg) = err {
    ///     assert_eq!(msg, "invalid digest: xxx");
    /// } else {
    ///     panic!("invalid error: {:?}", err);
    /// }
    /// ```
    ///
    /// [`Digest`]: enum.Digest.html
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    pub fn from_string(str: &str) -> Result<Option<Digest>> {
        match str {
            "none" => Ok(None),
            "sha1" => Ok(Some(Digest::Sha1)),
            _ => {
                let message = format!("invalid digest: {}", str);
                Err(Error::InvalArg(message))
            }
        }
    }

    pub(crate) fn to_openssl(&self) -> hash::MessageDigest {
        match self {
            Digest::Sha1 => hash::MessageDigest::sha1(),
        }
    }
}

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Digest::Sha1 => write!(f, "sha1"),
        }
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
pub enum WrappingKeyData {
    /// PBKDF2
    Pbkdf2 {
        /// Number of iterations used by PBKDF2.
        iterations: u32,

        /// A salt value used by PBKDF2.
        salt: Vec<u8>,
    },
}

impl WrappingKeyData {
    /// Creates a `WrappingKeyData` instance for the PBKDF2 algorithm.
    ///
    /// The `iterations` and the `salt` values are used to customize the PBKDF2
    /// algorithm.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let WrappingKeyData::Pbkdf2 { iterations, salt } = WrappingKeyData::pbkdf2(5, &[1, 2, 3]);
    ///
    /// assert_eq!(iterations, 5);
    /// assert_eq!(salt, [1, 2, 3]);
    /// ```
    pub fn pbkdf2(iterations: u32, salt: &[u8]) -> WrappingKeyData {
        WrappingKeyData::Pbkdf2 {
            iterations,
            salt: salt.to_vec(),
        }
    }

    /// Generates a `WrappingKeyData` instance for the PBKDF2 algorithm.
    ///
    /// The `iterations` value is used to customize the PBKDF2 algorithm.
    /// For the [`salt`] `salt_len` bytes of random data are generated.
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
    /// let WrappingKeyData::Pbkdf2 { iterations, salt } =
    /// WrappingKeyData::generate_pbkdf2(5, 3).unwrap();
    ///
    /// assert_eq!(iterations, 5);
    /// assert_eq!(salt.len(), 3); // salt filled with random data
    /// ```
    ///
    /// [`salt`]: #variant.Pbkdf2.field.salt
    /// [`Error::OpenSSL`]: ../error/enum.Error.html#variant.OpenSSL
    pub fn generate_pbkdf2(iterations: u32, salt_len: u32) -> Result<WrappingKeyData> {
        let mut salt = vec![0; salt_len as usize];
        random(&mut salt)?;

        Ok(WrappingKeyData::Pbkdf2 { iterations, salt })
    }

    pub(crate) fn create_wrapping_key(
        &self,
        password: &[u8],
        digest: Digest,
    ) -> Result<SecureVec<u8>> {
        if password.is_empty() {
            let msg = format!("invalid password, cannot be empty");
            error!("{}", msg);
            return Err(Error::InvalArg(msg));
        }

        let WrappingKeyData::Pbkdf2 { iterations, salt } = self;

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

impl Clone for WrappingKeyData {
    fn clone(&self) -> Self {
        let WrappingKeyData::Pbkdf2 { iterations, salt } = self;

        WrappingKeyData::Pbkdf2 {
            iterations: iterations.clone(),
            salt: salt.to_vec(),
        }
    }
}

impl fmt::Debug for WrappingKeyData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WrappingKeyData::Pbkdf2 { iterations, salt } => {
                let salt = format!("<{} bytes>", salt.len());
                fmt.debug_struct("Pbkdf2")
                    .field("iterations", &iterations)
                    .field("salt", &salt)
                    .finish()
            }
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

impl DiskType {
    /// Converts the given `str` into a [`DiskType`] variant.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `str` is not
    /// a valid disk type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    /// use nuts::error::*;
    ///
    /// assert_eq!(
    ///     DiskType::from_string("fat-zero").unwrap(),
    ///     DiskType::FatZero
    /// );
    /// assert_eq!(
    ///     DiskType::from_string("fat-random").unwrap(),
    ///     DiskType::FatRandom
    /// );
    /// assert_eq!(
    ///     DiskType::from_string("thin-zero").unwrap(),
    ///     DiskType::ThinZero
    /// );
    /// assert_eq!(
    ///     DiskType::from_string("thin-random").unwrap(),
    ///     DiskType::ThinRandom
    /// );
    ///
    /// let err = DiskType::from_string("xxx").unwrap_err();
    ///
    /// if let Error::InvalArg(msg) = err {
    ///     assert_eq!(msg, "invalid disk-type: xxx");
    /// } else {
    ///     panic!("invalid error: {:?}", err);
    /// }
    /// ```
    ///
    /// [`DiskType`]: enum.DiskType.html
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    pub fn from_string(str: &str) -> Result<DiskType> {
        match str {
            "fat-zero" => Ok(DiskType::FatZero),
            "fat-random" => Ok(DiskType::FatRandom),
            "thin-zero" => Ok(DiskType::ThinZero),
            "thin-random" => Ok(DiskType::ThinRandom),
            _ => {
                let message = format!("invalid disk-type: {}", str);
                Err(Error::InvalArg(message))
            }
        }
    }
}

impl std::fmt::Display for DiskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiskType::FatZero => write!(f, "fat-zero"),
            DiskType::FatRandom => write!(f, "fat-random"),
            DiskType::ThinZero => write!(f, "thin-zero"),
            DiskType::ThinRandom => write!(f, "thin-random"),
        }
    }
}

/// The minimum size of a block.
pub const BLOCK_MIN_SIZE: u32 = 512;

/// Options to customize the creation of a container.
///
/// Use [`default()`] to create a set of default parameters. The
/// [`default_with_sizes()`] functions creates a set to default parameters with
/// some custom sizes.
///
/// [`default()`]: #method.default
/// [`default_with_sizes()`]: #method.default_with_sizes
#[derive(Debug)]
pub struct Options {
    /// The disk type.
    pub dtype: DiskType,

    /// The wrapping key algorithm.
    pub wkey: Option<WrappingKeyData>,

    /// Cipher used by the container.
    pub cipher: Cipher,

    /// Message digest used by the container.
    pub md: Option<Digest>,

    /// The size of a single block.
    bsize: u32,

    /// Number of blocks.
    blocks: u64,
}

impl Options {
    /// Creates a set of defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = Options::default().unwrap();
    ///
    /// let WrappingKeyData::Pbkdf2 { iterations, salt } = options.wkey.as_ref().unwrap();
    /// assert_eq!(*iterations, 65536);
    /// assert_eq!(salt.len(), 16); // salt is filled with random data
    ///
    /// assert_eq!(options.dtype, DiskType::FatRandom);
    /// assert_eq!(options.cipher, Cipher::Aes128Ctr);
    /// assert_eq!(options.md, Some(Digest::Sha1));
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    pub fn default() -> Result<Options> {
        Options::default_with_cipher(Cipher::Aes128Ctr)
    }

    /// Creates a set of defaults with the given `cipher`.
    ///
    /// If the `cipher` is [`Cipher::None`], then digest and wrapping_key are set
    /// to [`Option::None`] as they are not used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = Options::default_with_cipher(Cipher::Aes128Ctr).unwrap();
    ///
    /// assert_eq!(options.cipher, Cipher::Aes128Ctr);
    /// assert_eq!(options.md, Some(Digest::Sha1));
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = Options::default_with_cipher(Cipher::None).unwrap();
    ///
    /// assert_eq!(options.dtype, DiskType::FatRandom);
    /// assert_eq!(options.wkey, None);
    /// assert_eq!(options.cipher, Cipher::None);
    /// assert_eq!(options.md, None);
    /// assert_eq!(options.bsize(), 512);
    /// assert_eq!(options.blocks(), 2048);
    /// ```
    ///
    /// [`Cipher::None`]: enum.Cipher.html#variant.None
    /// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    pub fn default_with_cipher(cipher: Cipher) -> Result<Options> {
        let options = Options {
            dtype: DiskType::FatRandom,
            cipher: Cipher::None,
            md: None,
            wkey: None,
            bsize: BLOCK_MIN_SIZE,
            blocks: (1024 * 1024 / BLOCK_MIN_SIZE) as u64, // container of 1MB
        };

        if cipher != Cipher::None {
            Ok(Options {
                cipher: cipher,
                md: Some(Digest::Sha1),
                wkey: Some(WrappingKeyData::generate_pbkdf2(65536, 16)?),
                ..options
            })
        } else {
            Ok(options)
        }
    }

    /// Creates a set of defaults with some custom sizes.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `bsize` or
    /// `blocks` are invalid.
    ///
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::*;
    ///
    /// let options = Options::default_with_sizes(1024, 2).unwrap();
    ///
    /// let WrappingKeyData::Pbkdf2 { iterations, salt } = options.wkey.as_ref().unwrap();
    /// assert_eq!(*iterations, 65536);
    /// assert_eq!(salt.len(), 16); // salt is filled with random data
    ///
    /// assert_eq!(options.dtype, DiskType::FatRandom);
    /// assert_eq!(options.cipher, Cipher::Aes128Ctr);
    /// assert_eq!(options.md, Some(Digest::Sha1));
    /// assert_eq!(options.bsize(), 1024);
    /// assert_eq!(options.blocks(), 2);
    /// ```
    pub fn default_with_sizes(bsize: u32, blocks: u64) -> Result<Options> {
        let mut options = Options::default()?;
        options.update_sizes(bsize, blocks)?;
        Ok(options)
    }

    /// Returns the block size.
    pub fn bsize(&self) -> u32 {
        self.bsize
    }

    /// Returns the number of blocks.
    pub fn blocks(&self) -> u64 {
        self.blocks
    }

    /// Updates both size attributes of the options.
    ///
    /// The `bsize` argument is the block size and must be a multiple of
    /// [`BLOCK_MIN_SIZE`] bytes. You cannot have a block size less that
    /// [`BLOCK_MIN_SIZE`] bytes!
    ///
    /// The `blocks` argument specifies the number of blocks, which should be
    /// allocated for the container. It must be a number greater than `0`.
    ///
    /// # Errors
    ///
    /// This function will return an [`Error::InvalArg`] error if `bsize` or
    /// `blocks` are invalid.
    ///
    /// [`Error::InvalArg`]: ../error/enum.Error.html#variant.InvalArg
    /// [`BLOCK_MIN_SIZE`]: constant.BLOCK_MIN_SIZE.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nuts::types::Options;
    ///
    /// let mut options = Options::default().unwrap();
    ///
    /// assert!(options.update_sizes(1024, 2).is_ok());
    /// assert_eq!(options.bsize(), 1024);
    /// assert_eq!(options.blocks(), 2);
    /// ```
    pub fn update_sizes(&mut self, bsize: u32, blocks: u64) -> Result<()> {
        if bsize < BLOCK_MIN_SIZE {
            let message = format!(
                "Invalid block size, got {} but must be at least {}.",
                bsize, BLOCK_MIN_SIZE
            );
            return Err(Error::InvalArg(message));
        }

        if bsize % BLOCK_MIN_SIZE != 0 {
            let message = format!(
                "Invalid block size, got {} but must be a multiple of {}.",
                bsize, BLOCK_MIN_SIZE
            );
            return Err(Error::InvalArg(message));
        }

        if blocks == 0 {
            let message = format!("Invalid number of blocks, got {}, expected > 0.", blocks);
            return Err(Error::InvalArg(message));
        }

        self.bsize = bsize;
        self.blocks = blocks;

        Ok(())
    }
}

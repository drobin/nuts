// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use nuts_container::backend::{Create, HeaderGet, HeaderSet, Open, HEADER_MAX_SIZE};

use crate::error::{Error, Result};
use crate::id::Id;
use crate::{read_header, write_header, DirectoryBackend};

const BLOCK_MIN_SIZE: u32 = 512;

/// [Options](nuts_container::backend::Backend::CreateOptions) needed to create
/// the backend.
///
/// You must pass the path, where the directory tree should be stored, to
/// [`CreateOptions::for_path()`], if creating a `CreateOptions` instance.
///
/// Furthermore the following options can be specified:
///
/// * [`CreateOptions::with_overwrite()`]: If set to `true` an already existing
///   path is reused. **Note**: If you overwrite an existing path, the content
///   is not removed! If set to `false` and the base path exists, the build
///   operation aborts with [`Error::Exists`]. The default is `false`.
/// * [`CreateOptions::with_bsize()`]: Specifies the block size of the backend.
///   This is the number of bytes, which can  be stored in an individual block.
///   The minimum block size is 512 bytes. The default is `512`.
#[derive(Clone, Debug)]
pub struct CreateOptions {
    path: PathBuf,
    bsize: u32,
    overwrite: bool,
}

impl CreateOptions {
    /// Creates a new `CreateOptions` instance.
    ///
    /// You must pass the `path`, where the directory tree should be stored, to
    /// the function.
    ///
    /// For further options default values are applied.
    pub fn for_path<P: AsRef<Path>>(path: P) -> Self {
        CreateOptions {
            path: path.as_ref().to_path_buf(),
            bsize: BLOCK_MIN_SIZE,
            overwrite: false,
        }
    }

    /// Assigns a new overwrite flag to the options.
    ///
    /// If set to `true` an already existing path is reused. **Note**: If you
    /// overwrite an existing path, the content is not removed! If set to
    /// `false` and the base path exists, the build operation aborts with
    /// [`Error::Exists`].
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Assigns a new block size to the options.
    ///
    /// This is the number of bytes, which can  be stored in an individual
    /// block.
    pub fn with_bsize(mut self, bsize: u32) -> Self {
        self.bsize = bsize;
        self
    }

    fn validate(&self) -> Result<()> {
        if self.bsize >= BLOCK_MIN_SIZE {
            Ok(())
        } else {
            Err(Error::InvalidBlockSize(self.bsize))
        }
    }
}

impl HeaderSet<DirectoryBackend> for CreateOptions {
    fn put_header_bytes(&mut self, bytes: &[u8; HEADER_MAX_SIZE]) -> Result<()> {
        self.validate()
            .and_then(|()| write_header(&self.path, self.bsize, bytes))
    }
}

impl Create<DirectoryBackend> for CreateOptions {
    fn settings(&self) -> Settings {
        self.clone().into()
    }

    fn build(self) -> Result<DirectoryBackend> {
        self.validate()?;

        if !self.overwrite {
            let header_path = Id::min().to_pathbuf(&self.path);

            if header_path.exists() {
                return Err(Error::Exists);
            }
        }

        Ok(DirectoryBackend {
            bsize: self.bsize,
            path: self.path,
        })
    }
}

/// [Options](nuts_container::backend::Backend::OpenOptions) needed to open the
/// backend.
///
/// You must pass the path, where the directory tree is stored, to
/// [`OpenOptions::for_path()`], if creating a `OpenOptions` instance.
pub struct OpenOptions {
    path: PathBuf,
}

impl OpenOptions {
    /// Creates a new `OpenOptions` instance.
    ///
    /// You must pass the `path`, where the directory tree should is stored, to
    /// the function.
    pub fn for_path<P: AsRef<Path>>(path: P) -> OpenOptions {
        OpenOptions {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl HeaderGet<DirectoryBackend> for OpenOptions {
    fn get_header_bytes(&mut self, bytes: &mut [u8; HEADER_MAX_SIZE]) -> Result<()> {
        read_header(&self.path, bytes)
    }
}

impl Open<DirectoryBackend> for OpenOptions {
    fn build(self, settings: Settings) -> Result<DirectoryBackend> {
        Ok(DirectoryBackend {
            bsize: settings.bsize,
            path: self.path,
        })
    }
}

/// [Settings](nuts_container::backend::Backend::Settings) used by the backend.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    bsize: u32,
}

impl From<CreateOptions> for Settings {
    fn from(options: CreateOptions) -> Self {
        Settings {
            bsize: options.bsize,
        }
    }
}

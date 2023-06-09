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

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use nuts::backend::{Options, BLOCK_MIN_SIZE};

use crate::error::{Error, Result};
use crate::DirectoryBackend;

#[derive(Debug)]
pub struct DirectoryCreateOptions {
    pub(crate) path: PathBuf,
    pub(crate) bsize: u32,
    pub(crate) overwrite: bool,
}

impl DirectoryCreateOptions {
    pub fn for_path<P: AsRef<Path>>(path: P) -> Self {
        DirectoryCreateOptions {
            path: path.as_ref().to_path_buf(),
            bsize: BLOCK_MIN_SIZE,
            overwrite: false,
        }
    }

    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    pub fn with_bsize(mut self, bsize: u32) -> Self {
        self.bsize = bsize;
        self
    }

    fn bsize_validate(&self) -> Result<()> {
        if self.bsize >= BLOCK_MIN_SIZE {
            Ok(())
        } else {
            Err(Error::InvalidBlockSize(self.bsize))
        }
    }
}

impl Options<DirectoryBackend> for DirectoryCreateOptions {
    fn validate(&self) -> Result<()> {
        self.bsize_validate()
    }
}

pub struct DirectoryOpenOptions {
    pub(crate) path: PathBuf,
}

impl DirectoryOpenOptions {
    pub fn for_path<P: AsRef<Path>>(path: P) -> DirectoryOpenOptions {
        DirectoryOpenOptions {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Options<DirectoryBackend> for DirectoryOpenOptions {
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DirectorySettings {
    pub(crate) bsize: u32,
}

impl From<DirectoryCreateOptions> for DirectorySettings {
    fn from(options: DirectoryCreateOptions) -> Self {
        DirectorySettings {
            bsize: options.bsize,
        }
    }
}

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

pub(crate) mod inner;

use std::ops;

use crate::container::inner::Inner;
use crate::error::Error;
use crate::result::Result;
use crate::types::{Cipher, Digest, DiskType, Options};

pub struct Container {
    password: Option<Vec<u8>>,
    inner: Option<Inner>,
}

impl Container {
    pub fn new() -> Container {
        Container {
            password: None,
            inner: None,
        }
    }

    pub fn set_password(&mut self, password: &[u8]) {
        self.password = Some(password.to_vec());
    }

    pub fn create(&mut self, path: &str, options: &Options) -> Result<()> {
        if self.inner.is_none() {
            let password = self.password.as_ref().map(|p| p.as_slice());
            self.inner = Some(Inner::create(path, password, options)?);
            Ok(())
        } else {
            Err(Error::Opened)
        }
    }

    pub fn open(&mut self, path: &str) -> Result<()> {
        if self.inner.is_none() {
            let password = self.password.as_ref().map(|p| p.as_slice());
            self.inner = Some(Inner::open(path, password)?);
            Ok(())
        } else {
            Err(Error::Opened)
        }
    }

    pub fn cipher(&self) -> Result<Cipher> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.header.cipher))
    }

    pub fn digest(&self) -> Result<Option<Digest>> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.header.digest))
    }

    pub fn dtype(&self) -> Result<DiskType> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.secret.dtype))
    }

    pub fn bsize(&self) -> Result<u32> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.secret.bsize))
    }

    pub fn blocks(&self) -> Result<u64> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.secret.blocks))
    }

    pub fn ablocks(&self) -> Result<u64> {
        self.inner
            .as_ref()
            .map_or(Err(Error::Closed), |inner| Ok(inner.io.ablocks))
    }
}

impl ops::Drop for Container {
    fn drop(&mut self) {
        if let Some(vec) = self.password.as_mut() {
            for e in vec.iter_mut() {
                *e = 0;
            }
        }
    }
}

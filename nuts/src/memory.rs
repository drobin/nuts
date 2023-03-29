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
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::{cmp, error, fmt, mem, result};

use nuts_backend::{Backend, BlockId, Options};

#[derive(Debug)]
pub struct MemError(String);

impl fmt::Display for MemError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl error::Error for MemError {}

pub struct MemOptions();

impl MemOptions {
    pub fn new() -> MemOptions {
        MemOptions()
    }
}

impl Options<MemoryBackend> for MemOptions {
    fn validate(&self) -> result::Result<(), MemError> {
        Ok(())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MemSettings();

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MemId(u32);

impl fmt::Display for MemId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl FromStr for MemId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FromStr::from_str(s).map(|n| MemId(n))
    }
}

impl BlockId for MemId {
    fn null() -> MemId {
        MemId(u32::MAX)
    }

    fn is_null(&self) -> bool {
        self.0 == u32::MAX
    }

    fn size() -> usize {
        mem::size_of::<u32>()
    }
}

pub struct MemoryBackend(HashMap<u32, [u8; 512]>);

impl MemoryBackend {
    fn new() -> MemoryBackend {
        let mut backend = MemoryBackend(HashMap::new());

        backend.aquire_id(0).unwrap();

        backend
    }

    fn aquire_id(&mut self, id: u32) -> result::Result<MemId, MemError> {
        match self.0.insert(id, [0; 512]) {
            Some(_) => return Err(MemError(format!("already aquired: {}", id))),
            None => Ok(MemId(id)),
        }
    }
}

impl Backend for MemoryBackend {
    type CreateOptions = MemOptions;
    type OpenOptions = MemOptions;
    type Settings = MemSettings;
    type Err = MemError;
    type Id = MemId;
    type Info = ();

    fn create(_options: MemOptions) -> result::Result<(Self, MemSettings), MemError> {
        Ok((MemoryBackend::new(), MemSettings()))
    }

    fn open(_options: MemOptions) -> result::Result<Self, MemError> {
        Ok(MemoryBackend::new())
    }

    fn configure(&mut self, _settings: MemSettings) {}

    fn info(&self) -> result::Result<(), MemError> {
        Ok(())
    }

    fn block_size(&self) -> u32 {
        512
    }

    fn header_id(&self) -> MemId {
        MemId(0)
    }

    fn aquire(&mut self) -> result::Result<MemId, MemError> {
        let id = match self.0.keys().max() {
            Some(n) => n + 1,
            None => 0,
        };

        self.aquire_id(id)
    }

    fn release(&mut self, id: MemId) -> result::Result<(), MemError> {
        self.0.remove(&id.0);
        Ok(())
    }

    fn read(&mut self, id: &MemId, buf: &mut [u8]) -> result::Result<usize, MemError> {
        match self.0.get(&id.0) {
            Some(src) => {
                let len = cmp::min(src.len(), buf.len());

                let source = &src[..len];
                let target = &mut buf[..len];

                target.copy_from_slice(source);

                Ok(len)
            }
            None => Err(MemError(format!("no such id: {}", id))),
        }
    }

    fn write(&mut self, id: &MemId, buf: &[u8]) -> result::Result<usize, MemError> {
        match self.0.get_mut(&id.0) {
            Some(target) => {
                let mut source = Cow::from(buf);
                let mut len = source.len();

                if len != 512 {
                    len = cmp::min(source.len(), 512);
                    source.to_mut().resize(512, 0);
                }

                target.copy_from_slice(&source);

                Ok(len)
            }
            None => Err(MemError(format!("no such id: {}", id))),
        }
    }
}

// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use nuts_container::{Container, Error, Migration, Service, ServiceFactory};
use nuts_memory::MemoryBackend;
use std::path::PathBuf;
use thiserror::Error;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[allow(dead_code)]
pub fn fixture_password() -> Result<Vec<u8>, String> {
    Ok(b"sample".to_vec())
}

pub fn fixture_path(name: &str) -> PathBuf {
    [MANIFEST_DIR, "data", name].iter().collect()
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct SampleError(#[from] pub Error<MemoryBackend>);

pub struct SampleMigration;

impl Migration for SampleMigration {
    fn migrate_rev0(&self, _userdata: &[u8]) -> Result<(u32, Vec<u8>), String> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct SampleService(Container<MemoryBackend>);

impl SampleService {
    #[allow(dead_code)]
    pub fn into_container(self) -> Container<MemoryBackend> {
        self.0
    }
}

impl Service<MemoryBackend> for SampleService {
    type Migration = SampleMigration;

    fn sid() -> u32 {
        666
    }

    fn need_top_id() -> bool {
        false
    }

    fn migration() -> SampleMigration {
        SampleMigration
    }
}

impl ServiceFactory<MemoryBackend> for SampleService {
    type Service = Self;
    type Err = SampleError;

    fn create(container: Container<MemoryBackend>) -> Result<Self::Service, Self::Err> {
        Ok(SampleService(container))
    }

    fn open(container: Container<MemoryBackend>) -> Result<Self::Service, Self::Err> {
        Ok(SampleService(container))
    }
}

// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use nuts_container::{Cipher, Container, CreateOptionsBuilder};
use nuts_memory::MemoryBackend;

use crate::{Archive, ArchiveFactory};

macro_rules! into_error {
    ($err:expr, $($path:ident)::+) => {
        match $err {
            $($path)::+(cause) => cause,
            _ => panic!("invalid error"),
        }
    };
}

pub fn setup_archive_with_bsize(bsize: u32) -> Archive<MemoryBackend> {
    let backend = MemoryBackend::new_with_bsize(bsize);
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();

    let container = Container::create(backend, options).unwrap();
    Container::create_service::<ArchiveFactory>(container).unwrap()
}

pub fn setup_container_with_bsize(bsize: u32) -> Container<MemoryBackend> {
    let backend = MemoryBackend::new_with_bsize(bsize);
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();

    Container::create(backend, options).unwrap()
}

pub(crate) use into_error;

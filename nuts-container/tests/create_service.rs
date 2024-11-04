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

mod common;

use nuts_container::{
    Cipher, Container, CreateOptionsBuilder, Error, HeaderError, OpenOptionsBuilder,
};
use nuts_memory::MemoryBackend;
use std::fs::File;

use crate::common::{fixture_path, SampleService};

#[test]
fn inval_revision() {
    // you need a rev0-container
    let path = fixture_path("compat", "0.6.8-none.json");
    let backend: MemoryBackend = serde_json::from_reader(File::open(path).unwrap()).unwrap();

    let options = OpenOptionsBuilder::new().build::<MemoryBackend>().unwrap();
    let container = Container::open(backend, options).unwrap();

    assert_eq!(container.info().unwrap().revision, 0);

    let err = Container::create_service::<SampleService>(container).unwrap_err();

    assert!(matches!(err.0, Error::Header(cause)
        if matches!(cause,HeaderError::InvalidRevision(expected, got)
            if expected == 2 && got == 0)));
}

#[test]
fn already_attached() {
    let backend = {
        let options = CreateOptionsBuilder::new(Cipher::None)
            .build::<MemoryBackend>()
            .unwrap();
        let container = Container::create(MemoryBackend::new(), options).unwrap();
        let service = Container::create_service::<SampleService>(container).unwrap();

        service.into_container().into_backend()
    };

    let options = OpenOptionsBuilder::new().build::<MemoryBackend>().unwrap();
    let container = Container::open(backend, options).unwrap();
    let err = Container::create_service::<SampleService>(container).unwrap_err();

    assert!(matches!(err.0, Error::Header(cause)
        if matches!(cause,HeaderError::UnexpectedSid { expected, got }
            if expected.is_none() && got.unwrap() == 666)));
}

// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use crate::container::{Cipher, Container, CreateOptionsBuilder, Digest, Kdf, OpenOptionsBuilder};
use crate::memory::MemoryBackend;

#[test]
fn create() {
    let backend = MemoryBackend::new();
    let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");

    // Let's create an encrypted container (with aes128-ctr).
    let options = CreateOptionsBuilder::<MemoryBackend>::new(backend, Cipher::Aes128Ctr)
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .with_kdf(kdf.clone())
        .build()
        .unwrap();
    let container = Container::create(options).unwrap();
    let info = container.info().unwrap();

    assert_eq!(info.cipher, Cipher::Aes128Ctr);
    assert_eq!(info.kdf, kdf);
}

#[test]
fn open() {
    let backend = MemoryBackend::new();

    // When opening a contaier with a MemoryBackend attached,
    // the container is always unencrypted.
    let options = OpenOptionsBuilder::<MemoryBackend>::new(backend)
        .build()
        .unwrap();
    let container = Container::open(options).unwrap();
    let info = container.info().unwrap();

    assert_eq!(info.cipher, Cipher::None);
    assert_eq!(info.kdf, Kdf::None);
}

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

use nuts_memory::MemoryBackend;

use crate::{Cipher, Container, CreateOptionsBuilder, Digest, Info, Kdf};

#[test]
fn none() {
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();
    let container = Container::<MemoryBackend>::create(MemoryBackend::new(), options).unwrap();

    assert_eq!(
        container.info().unwrap(),
        Info {
            backend: (),
            revision: 1,
            cipher: Cipher::None,
            kdf: Kdf::None,
            bsize_gross: 512,
            bsize_net: 512,
        }
    );
}

#[test]
fn aes128_ctr() {
    let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
    let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .with_kdf(kdf.clone())
        .build::<MemoryBackend>()
        .unwrap();
    let container = Container::<MemoryBackend>::create(MemoryBackend::new(), options).unwrap();

    assert_eq!(
        container.info().unwrap(),
        Info {
            backend: (),
            revision: 1,
            cipher: Cipher::Aes128Ctr,
            kdf,
            bsize_gross: 512,
            bsize_net: 512,
        }
    );
}

#[test]
fn aes128_gcm() {
    let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
    let options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .with_kdf(kdf.clone())
        .build::<MemoryBackend>()
        .unwrap();
    let container = Container::<MemoryBackend>::create(MemoryBackend::new(), options).unwrap();

    assert_eq!(
        container.info().unwrap(),
        Info {
            backend: (),
            revision: 1,
            cipher: Cipher::Aes128Gcm,
            kdf,
            bsize_gross: 512,
            bsize_net: 496,
        }
    );
}

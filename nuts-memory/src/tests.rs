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

#[test]
fn create() {
    use nuts_container::*;

    use crate::MemoryBackend;

    // Example creates an encrypted container with an attached MemoryBackend.

    let backend = MemoryBackend::new();
    let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");

    // Let's create an encrypted container (with aes128-ctr).
    let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .with_kdf(kdf.clone())
        .build::<MemoryBackend>()
        .unwrap();
    let container = Container::<MemoryBackend>::create(backend, options).unwrap();
    let info = container.info().unwrap();

    assert_eq!(info.cipher, Cipher::Aes128Ctr);
    assert_eq!(info.kdf, kdf);
}

#[test]
fn open() {
    use nuts_container::*;

    use crate::{Error as MemoryError, MemoryBackend};

    // Example tries to open a container with an attached MemoryBackend,
    // which cannot work because no header is available.

    let backend = MemoryBackend::new();
    let options = OpenOptionsBuilder::new().build::<MemoryBackend>().unwrap();
    let err = Container::<MemoryBackend>::open(backend, options).unwrap_err();

    assert!(matches!(err, Error::Backend(MemoryError::NoHeader)));
}

#[test]
fn reopen() {
    use nuts_container::*;

    use crate::MemoryBackend;

    let (backend, kdf) = {
        let backend = MemoryBackend::new();
        let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");

        // Let's create an encrypted container (with aes128-ctr).
        let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
            .with_password_callback(|| Ok(b"abc".to_vec()))
            .with_kdf(kdf.clone())
            .build::<MemoryBackend>()
            .unwrap();
        let container = Container::<MemoryBackend>::create(backend, options).unwrap();

        (container.into_backend(), kdf)
    };

    // When opening a container with an attached MemoryBackend,
    // the backend takes the header from a previous open attempt.
    let options = OpenOptionsBuilder::new()
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .build::<MemoryBackend>()
        .unwrap();
    let container = Container::<MemoryBackend>::open(backend, options).unwrap();
    let info = container.info().unwrap();

    assert_eq!(info.cipher, Cipher::Aes128Ctr);
    assert_eq!(info.kdf, kdf);
}

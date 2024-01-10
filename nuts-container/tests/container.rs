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
    use nuts_memory::MemoryBackend;

    // Create a container with a memory backend.
    let backend = MemoryBackend::new();

    // Let's create an encrypted container (with aes128-ctr).
    // Because you are encrypting the container, you need to assign a
    // password callback.
    let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
    let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .with_kdf(kdf.clone())
        .build::<MemoryBackend>()
        .unwrap();

    // Create the container and fetch information.
    // Here you can directly pass the backend instance to the create() method
    // because MemoryBackend implements the Backend::CreateOptions trait.
    let container = Container::<MemoryBackend>::create(backend, options).unwrap();
    let info = container.info().unwrap();

    assert_eq!(info.cipher, Cipher::Aes128Ctr);
    assert_eq!(info.kdf, kdf);
}

#[test]
fn open() {
    use nuts_container::*;
    use nuts_memory::MemoryBackend;

    let (backend, kdf) = {
        // In this example you create a container in a separate block.
        // So, the created container is closed again when leaving the scope.
        let backend = MemoryBackend::new();
        let kdf = Kdf::pbkdf2(Digest::Sha1, 65536, b"123");
        let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
            .with_password_callback(|| Ok(b"abc".to_vec()))
            .with_kdf(kdf.clone())
            .build::<MemoryBackend>()
            .unwrap();

        // Create the container.
        let container = Container::<MemoryBackend>::create(backend, options).unwrap();
        let backend = container.into_backend();

        (backend, kdf)
    };

    // Open the container and fetch information.
    // Here you can directly pass the backend instance to the open() method
    // because MemoryBackend implements the Backend::OpenOptions trait.
    let options = OpenOptionsBuilder::new()
        .with_password_callback(|| Ok(b"abc".to_vec()))
        .build::<MemoryBackend>()
        .unwrap();
    let container = Container::<MemoryBackend>::open(backend, options).unwrap();
    let info = container.info().unwrap();

    assert_eq!(info.cipher, Cipher::Aes128Ctr);
    assert_eq!(info.kdf, kdf);
}

#[test]
fn read() {
    use nuts_container::*;
    use nuts_memory::MemoryBackend;

    // Create a container with a memory backend.
    let mut backend = MemoryBackend::new();

    // Insert a block into the backend.
    // Note that the insert() method is a part of the MemoryBackend and directly
    // inserts a block into the backend (bypassing the crypto capabilities of the
    // container).
    let id = backend.insert().unwrap();

    // Create the container.
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();
    let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();

    // Read full block.
    let mut buf = [b'x'; 512];
    assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
    assert_eq!(buf, [0; 512]);

    // Read block into a buffer which is smaller than the block-size.
    // The buffer is filled with the first 400 bytes from the block.
    let mut buf = [b'x'; 400];
    assert_eq!(container.read(&id, &mut buf).unwrap(), 400);
    assert_eq!(buf, [0; 400]);

    // Read block into a buffer which is bigger than the block-size.
    // The first 512 bytes are filled with the content of the block,
    // the remaining 8 bytes are not touched.
    let mut buf = [b'x'; 520];
    assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
    assert_eq!(buf[..512], [0; 512]);
    assert_eq!(buf[512..], [b'x'; 8]);
}

#[test]
fn write() {
    use nuts_container::*;
    use nuts_memory::MemoryBackend;

    // In this example you create a container in a separate block.
    // So, the created container is closed again when leaving the scope.
    let mut backend = MemoryBackend::new();

    // Insert a block into the backend.
    // Note that the insert() method is a part of the MemoryBackend and directly
    // inserts a block into the backend (bypassing the crypto capabilities of the
    // container).
    let id = backend.insert().unwrap();

    // Create the container.
    let options = CreateOptionsBuilder::new(Cipher::None)
        .build::<MemoryBackend>()
        .unwrap();
    let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();

    // Write a full block. The whole block is filled with 'x'.
    assert_eq!(container.write(&id, &[b'x'; 512]).unwrap(), 512);

    let mut buf = [0; 512];
    assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
    assert_eq!(buf, [b'x'; 512]);

    // Write a block from a buffer which is smaller than the block-size.
    // The first bytes of the block are filled with the data from the buffer,
    // the remaining space is padded with '0'.
    assert_eq!(container.write(&id, &[b'x'; 400]).unwrap(), 400);

    let mut buf = [0; 512];
    assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
    assert_eq!(buf[..400], [b'x'; 400]);
    assert_eq!(buf[400..], [0; 112]);

    // Write a block from a buffer which is bigger than the block-size.
    // The block is filled with the first data from the buffer.
    assert_eq!(container.write(&id, &[b'x'; 520]).unwrap(), 512);

    let mut buf = [0; 512];
    assert_eq!(container.read(&id, &mut buf).unwrap(), 512);
    assert_eq!(buf, [b'x'; 512]);
}

# nuts-container: A secure storage library.

## Introduction

The _nuts_ library implements a secure storage library, where data are stored
in a container. The container is divided into encrypted blocks. So a _nuts_
container looks like a block device. All the things (e.g. keys) you need to
open it are stored in the container itself. The library has a rich API, so it
can be easily integrated into you application.

The container does not manage the encrypted data itself. It is transferred to a
backend that is solely responsible for the persistent storage of the blocks. In
this way, the data can easily be stored on different media or in different
formats. The keys remain in the container and do not have to be stored in the
backend.

## Features

The following encryption algorithms are supported:

* AES128-CTR
* None (which basically disables encryption)

The actual key used for encryption of the blocks (and further information) is
encrypted with a PBKDF2 derivated wrapping key, which is based on a password
provided by the user.

You have a self-contained container, which means that all information you need
to open the container are stored in the first block. Some basic information
(like used cipher & wrapping key algorithm) are stored unencrypted, but most of
the information are also encrypted in the header.

## Examples

```rust
// Create a new container

use nuts_container::container::*;
use nuts_container::memory::MemoryBackend;

// Create a container with a memory backend.
// This backend stores the encrypted blocks in memory.
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
```

```rust
// Open an existing container

use nuts_container::container::*;
use nuts_container::memory::MemoryBackend;

let (backend, kdf) = {
    // In this example you create a container in a separate block.
    // So, the created container is closed again when leaving the scope
    // but the memory-backend still has stored its data.
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
```

## License

> You can check out the full license
> [here](https://github.com/drobin/nuts-container/blob/master/LICENSE).

This project is licensed under the terms of the **MIT** license.

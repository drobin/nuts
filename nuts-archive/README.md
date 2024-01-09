# nuts-archive: A `tar` like archive on top of a nuts-container.

## Introduction

The nuts-archive is an application based on the [nuts container]. Inspired by
the `tar` tool you can store files, directories and symlinks in a [nuts container].

* Entries can be appended at the end of the archive.
* They cannot be removed from the archive.
* You can travere the archive from the first to the last entry in the
  archive.

## Examples

```rust
// Append an entry at the end of the archive

use nuts_archive::Archive;
use nuts_container::container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
use tempdir::TempDir;

// This will create an empty archive in a temporary directory.
let tmp_dir = {
    let dir = TempDir::new("nuts-archive").unwrap();

    let backend_options = CreateOptions::for_path(&dir);
    let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<&TempDir>>()
        .unwrap();

    let container =
        Container::<DirectoryBackend<&TempDir>>::create(backend_options, contaner_options)
            .unwrap();
    Archive::create(container, false).unwrap();

    dir
};

// Open the container (with a directory backend) from the temporary directory.
let backend_options = OpenOptions::for_path(tmp_dir);
let container_options = OpenOptionsBuilder::new()
    .with_password_callback(|| Ok(b"123".to_vec()))
    .build::<DirectoryBackend<TempDir>>()
    .unwrap();
let container =
    Container::<DirectoryBackend<TempDir>>::open(backend_options, container_options).unwrap();

// Open the archive
let mut archive = Archive::open(container).unwrap();

// Append a new entry
let mut entry = archive.append("sample").build().unwrap();
entry.write_all("some sample data".as_bytes()).unwrap();
```

```rust
// Scans the archive for entries

use nuts_archive::Archive;
use nuts_container::container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
use tempdir::TempDir;

// This will create an empty archive in a temporary directory.
let tmp_dir = {
    let dir = TempDir::new("nuts-archive").unwrap();

    let backend_options = CreateOptions::for_path(&dir);
    let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<&TempDir>>()
        .unwrap();

    let container =
        Container::<DirectoryBackend<&TempDir>>::create(backend_options, contaner_options)
            .unwrap();
    Archive::create(container, false).unwrap();

    dir
};

// Open the container (with a directory backend) from the temporary directory.
let backend_options = OpenOptions::for_path(tmp_dir);
let container_options = OpenOptionsBuilder::new()
    .with_password_callback(|| Ok(b"123".to_vec()))
    .build::<DirectoryBackend<TempDir>>()
    .unwrap();
let container =
    Container::<DirectoryBackend<TempDir>>::open(backend_options, container_options).unwrap();

// Open the archive and append two entries
let mut archive = Archive::open(container).unwrap();

archive.append("f1").build().unwrap();
archive.append("f2").build().unwrap();

// Go through the archive
let entry = archive.first().unwrap().unwrap();
assert_eq!(entry.name(), "f1");

let entry = entry.next().unwrap().unwrap();
assert_eq!(entry.name(), "f2");

assert!(entry.next().is_none());
```

## License

> You can check out the full license
> [here](https://github.com/drobin/nuts/blob/master/LICENSE).

This project is licensed under the terms of the **MIT** license.

[nuts container]: https://crates.io/crates/nuts-container

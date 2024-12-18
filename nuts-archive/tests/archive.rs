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
    use nuts_archive::ArchiveFactory;
    use nuts_container::{Cipher, Container, CreateOptionsBuilder};
    use nuts_directory::{CreateOptions, DirectoryBackend};
    use tempfile::{Builder, TempDir};

    // Let's create an archive service (with a directory backend) in a temporary directory
    let tmp_dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();
    let backend_options = CreateOptions::for_path(tmp_dir);
    let container_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<TempDir>>()
        .unwrap();
    let container = Container::create(backend_options, container_options).unwrap();
    let archive = Container::create_service::<ArchiveFactory>(container).unwrap();

    // Fetch some information
    let info = archive.info();
    assert_eq!(info.blocks, 0);
    assert_eq!(info.files, 0);
}

#[test]
fn open() {
    use nuts_archive::ArchiveFactory;
    use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
    use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
    use tempfile::{Builder, TempDir};

    let dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();

    {
        // This will create an empty archive in a temporary directory.

        let backend_options = CreateOptions::for_path(dir.path().to_owned());
        let container_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
            .with_password_callback(|| Ok(b"123".to_vec()))
            .build::<DirectoryBackend<&TempDir>>()
            .unwrap();
        let container = Container::create(backend_options, container_options).unwrap();

        Container::create_service::<ArchiveFactory>(container).unwrap();
    }

    // Open the archive service (with a directory backend) from the temporary directory.
    let backend_options = OpenOptions::for_path(dir);
    let container_options = OpenOptionsBuilder::new()
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<TempDir>>()
        .unwrap();
    let container = Container::open(backend_options, container_options).unwrap();

    let archive = Container::open_service::<ArchiveFactory>(container, false).unwrap();

    // Fetch some information
    let info = archive.info();
    assert_eq!(info.blocks, 0);
    assert_eq!(info.files, 0);
}

#[test]
fn append() {
    use nuts_archive::ArchiveFactory;
    use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
    use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
    use tempfile::{Builder, TempDir};

    let dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();

    {
        // This will create an empty archive in a temporary directory.

        let backend_options = CreateOptions::for_path(dir.path().to_owned());
        let container_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
            .with_password_callback(|| Ok(b"123".to_vec()))
            .build::<DirectoryBackend<&TempDir>>()
            .unwrap();
        let container = Container::create(backend_options, container_options).unwrap();

        Container::create_service::<ArchiveFactory>(container).unwrap();
    }

    // Open the archive (with a directory backend) from the temporary directory.
    let backend_options = OpenOptions::for_path(dir);
    let container_options = OpenOptionsBuilder::new()
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<TempDir>>()
        .unwrap();
    let container = Container::open(backend_options, container_options).unwrap();

    let mut archive = Container::open_service::<ArchiveFactory>(container, false).unwrap();

    // Append a new file entry
    let mut entry = archive.append_file("sample file").build().unwrap();
    entry.write_all("some sample data".as_bytes()).unwrap();

    // Append a new directory entry
    archive
        .append_directory("sample directory")
        .build()
        .unwrap();

    // Append a new symlink entry
    archive
        .append_symlink("sample symlink", "target")
        .build()
        .unwrap();
}

#[test]
fn scan() {
    use nuts_archive::ArchiveFactory;
    use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
    use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
    use tempfile::{Builder, TempDir};

    let dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();

    {
        // This will create an empty archive in a temporary directory.

        let backend_options = CreateOptions::for_path(dir.path().to_owned());
        let container_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
            .with_password_callback(|| Ok(b"123".to_vec()))
            .build::<DirectoryBackend<&TempDir>>()
            .unwrap();
        let container = Container::create(backend_options, container_options).unwrap();

        Container::create_service::<ArchiveFactory>(container).unwrap();
    }

    // Open the archive (with a directory backend) from the temporary directory.
    let backend_options = OpenOptions::for_path(dir);
    let container_options = OpenOptionsBuilder::new()
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<TempDir>>()
        .unwrap();
    let container = Container::open(backend_options, container_options).unwrap();

    // Open the archive and append some entries
    let mut archive = Container::open_service::<ArchiveFactory>(container, false).unwrap();

    archive.append_file("f1").build().unwrap();
    archive.append_directory("f2").build().unwrap();
    archive.append_symlink("f3", "target").build().unwrap();

    // Go through the archive
    let entry = archive.first().unwrap().unwrap();
    assert!(entry.is_file());
    assert_eq!(entry.name(), "f1");

    let entry = entry.next().unwrap().unwrap();
    assert!(entry.is_directory());
    assert_eq!(entry.name(), "f2");

    let entry = entry.next().unwrap().unwrap();
    assert!(entry.is_symlink());
    assert_eq!(entry.name(), "f3");

    assert!(entry.next().is_none());
}

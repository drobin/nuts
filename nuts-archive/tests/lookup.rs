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

use nuts_archive::Archive;
use nuts_container::{Cipher, Container, CreateOptionsBuilder, OpenOptionsBuilder};
use nuts_directory::{CreateOptions, DirectoryBackend, OpenOptions};
use tempfile::{Builder, TempDir};

fn setup_archive() -> TempDir {
    let tmp_dir = Builder::new().prefix("nuts-archive").tempdir().unwrap();

    let backend_options = CreateOptions::for_path(&tmp_dir);
    let contaner_options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<&TempDir>>()
        .unwrap();

    let container =
        Container::<DirectoryBackend<&TempDir>>::create(backend_options, contaner_options).unwrap();
    Archive::create(container, false).unwrap();

    tmp_dir
}

fn open_archive(dir: TempDir) -> Archive<DirectoryBackend<TempDir>> {
    let backend_options = OpenOptions::for_path(dir);
    let container_options = OpenOptionsBuilder::new()
        .with_password_callback(|| Ok(b"123".to_vec()))
        .build::<DirectoryBackend<TempDir>>()
        .unwrap();
    let container =
        Container::<DirectoryBackend<TempDir>>::open(backend_options, container_options).unwrap();

    Archive::open(container).unwrap()
}

#[test]
fn empty_archive() {
    let tmp_dir = setup_archive();
    let mut archive = open_archive(tmp_dir);

    assert!(archive.lookup("sample").is_none());
}

#[test]
fn one_entry() {
    let tmp_dir = setup_archive();
    let mut archive = open_archive(tmp_dir);

    {
        let mut entry = archive.append_file("f1.txt").build().unwrap();
        entry.write_all(b"some content").unwrap();
    }

    // Lookup succeeded

    let entry = archive.lookup("f1.txt").unwrap().unwrap();

    assert!(entry.is_file());
    assert_eq!(entry.name(), "f1.txt");
    assert_eq!(entry.size(), 12);
    assert_eq!(
        entry.into_file().unwrap().read_vec().unwrap(),
        b"some content"
    );

    // Failed lookup

    assert!(archive.lookup("no.such.entry").is_none());
}

#[test]
fn two_entries() {
    let tmp_dir = setup_archive();
    let mut archive = open_archive(tmp_dir);

    {
        let mut entry = archive.append_file("f1.txt").build().unwrap();
        entry.write_all(b"some content").unwrap();
    }

    {
        archive.append_directory("d1").build().unwrap();
    }

    // File lookup succeeded

    let entry = archive.lookup("f1.txt").unwrap().unwrap();

    assert!(entry.is_file());
    assert_eq!(entry.name(), "f1.txt");
    assert_eq!(entry.size(), 12);
    assert_eq!(
        entry.into_file().unwrap().read_vec().unwrap(),
        b"some content"
    );

    // Directory lookup succeeded

    let entry = archive.lookup("d1").unwrap().unwrap();

    assert!(entry.is_directory());
    assert_eq!(entry.name(), "d1");
    assert_eq!(entry.size(), 0);

    // Failed lookup

    assert!(archive.lookup("no.such.entry").is_none());
}

#[test]
fn three_entries() {
    let tmp_dir = setup_archive();
    let mut archive = open_archive(tmp_dir);

    {
        let mut entry = archive.append_file("f1.txt").build().unwrap();
        entry.write_all(b"some content").unwrap();
    }

    {
        archive.append_directory("d1").build().unwrap();
    }

    {
        archive.append_symlink("s1.txt", "f1.txt").build().unwrap();
    }

    // File lookup succeeded

    let entry = archive.lookup("f1.txt").unwrap().unwrap();

    assert!(entry.is_file());
    assert_eq!(entry.name(), "f1.txt");
    assert_eq!(entry.size(), 12);
    assert_eq!(
        entry.into_file().unwrap().read_vec().unwrap(),
        b"some content"
    );

    // Directory lookup succeeded

    let entry = archive.lookup("d1").unwrap().unwrap();

    assert!(entry.is_directory());
    assert_eq!(entry.name(), "d1");
    assert_eq!(entry.size(), 0);

    // Symlink lookup succeeded

    let entry = archive.lookup("s1.txt").unwrap().unwrap();

    assert!(entry.is_symlink());
    assert_eq!(entry.name(), "s1.txt");
    assert_eq!(entry.size(), 6);
    assert_eq!(entry.into_symlink().unwrap().target(), "f1.txt");

    // Failed lookup

    assert!(archive.lookup("no.such.entry").is_none());
}

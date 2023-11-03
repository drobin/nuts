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

use nuts_container::memory::MemoryBackend;

use crate::entry::immut::tests::setup_archive;
use crate::entry::immut::{FileEntry, InnerEntry};
use crate::entry::{FULL, HALF};
use crate::error::Error;
use crate::Archive;

fn load_first<'a>(archive: &'a mut Archive<MemoryBackend>) -> FileEntry<'a, MemoryBackend> {
    let inner = InnerEntry::first(&mut archive.pager, &mut archive.tree)
        .unwrap()
        .unwrap();

    FileEntry(inner)
}

#[test]
fn empty() {
    let mut archive = setup_archive(0);
    let mut entry = load_first(&mut archive);
    let mut buf = [];

    entry.read_all(&mut buf).unwrap();
}

#[test]
fn empty_more() {
    let mut archive = setup_archive(0);
    let mut entry = load_first(&mut archive);
    let mut buf = [b'x'; 1];

    let err = entry.read_all(&mut buf).unwrap_err();
    assert!(matches!(err, Error::UnexpectedEof));
}

#[test]
fn half() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);
    let mut buf = [0; HALF as usize];

    entry.read_all(&mut buf).unwrap();
    assert_eq!(buf, (0..HALF).collect::<Vec<u8>>().as_slice());
}

#[test]
fn half_more() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);
    let mut buf = [0; HALF as usize + 1];

    let err = entry.read_all(&mut buf).unwrap_err();
    assert!(matches!(err, Error::UnexpectedEof));
}

#[test]
fn full() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);
    let mut buf = [0; FULL as usize];

    entry.read_all(&mut buf).unwrap();
    assert_eq!(buf, (0..FULL).collect::<Vec<u8>>().as_slice());
}

#[test]
fn full_more() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);
    let mut buf = [0; FULL as usize + 1];

    let err = entry.read_all(&mut buf).unwrap_err();
    assert!(matches!(err, Error::UnexpectedEof));
}

#[test]
fn full_half() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);
    let mut buf = [0; FULL as usize + HALF as usize];

    entry.read_all(&mut buf).unwrap();
    assert_eq!(buf, (0..FULL + HALF).collect::<Vec<u8>>().as_slice());
}

#[test]
fn full_half_more() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);
    let mut buf = [0; FULL as usize + HALF as usize + 1];

    let err = entry.read_all(&mut buf).unwrap_err();
    assert!(matches!(err, Error::UnexpectedEof));
}

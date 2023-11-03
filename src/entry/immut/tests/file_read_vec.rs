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
use crate::Archive;

fn load_first<'a>(archive: &'a mut Archive<MemoryBackend>) -> FileEntry<'a, MemoryBackend> {
    let inner = InnerEntry::first(&mut archive.container, &mut archive.tree)
        .unwrap()
        .unwrap();

    FileEntry(inner)
}

#[test]
fn empty() {
    let mut archive = setup_archive(0);
    let mut entry = load_first(&mut archive);
    assert_eq!(entry.read_vec().unwrap(), []);
}

#[test]
fn half() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);
    assert_eq!(entry.read_vec().unwrap(), (0..HALF).collect::<Vec<u8>>());
}

#[test]
fn full() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);
    assert_eq!(entry.read_vec().unwrap(), (0..FULL).collect::<Vec<u8>>());
}

#[test]
fn full_half() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);
    assert_eq!(
        entry.read_vec().unwrap(),
        (0..FULL + HALF).collect::<Vec<u8>>()
    );
}

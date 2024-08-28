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

use crate::entry::immut::{InnerEntry, SymlinkEntry};
use crate::entry::FULL;
use crate::tests::setup_archive_with_bsize;
use crate::Archive;

const DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn load_first(archive: &mut Archive<MemoryBackend>) -> SymlinkEntry<MemoryBackend> {
    let inner = InnerEntry::first(&mut archive.pager, &mut archive.tree)
        .unwrap()
        .unwrap();

    SymlinkEntry::new(inner).unwrap()
}

#[test]
fn empty_target() {
    let mut archive = setup_archive_with_bsize(FULL as u32);

    archive.append_symlink("f1", "").build().unwrap();

    let entry = load_first(&mut archive);

    assert_eq!(entry.target(), "");
}

#[test]
fn target_half_chunk() {
    let mut archive = setup_archive_with_bsize(FULL as u32);

    let target = (0..32)
        .map(|i| DIGITS[i % 10].to_string())
        .fold(String::new(), |mut acc, s| {
            acc.push_str(&s);
            acc
        });

    assert_eq!(target.bytes().len(), 32);
    archive.append_symlink("f1", &target).build().unwrap();

    let entry = load_first(&mut archive);

    assert_eq!(entry.target(), target);
}

#[test]
fn target_full_chunk() {
    let mut archive = setup_archive_with_bsize(FULL as u32);

    let target = (0..64)
        .map(|i| DIGITS[i % 10].to_string())
        .fold(String::new(), |mut acc, s| {
            acc.push_str(&s);
            acc
        });

    assert_eq!(target.bytes().len(), 64);
    archive.append_symlink("f1", &target).build().unwrap();

    let entry = load_first(&mut archive);

    assert_eq!(entry.target(), target);
}

#[test]
fn target_full_half_chunk() {
    let mut archive = setup_archive_with_bsize(FULL as u32);

    let target =
        (0..64 + 32)
            .map(|i| DIGITS[i % 10].to_string())
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s);
                acc
            });

    assert_eq!(target.bytes().len(), 64 + 32);
    archive.append_symlink("f1", &target).build().unwrap();

    let entry = load_first(&mut archive);

    assert_eq!(entry.target(), target);
}

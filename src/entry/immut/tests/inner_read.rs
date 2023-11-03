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
use crate::entry::immut::InnerEntry;
use crate::entry::{FULL, HALF};
use crate::Archive;

fn load_first(archive: &mut Archive<MemoryBackend>) -> InnerEntry<MemoryBackend> {
    InnerEntry::first(&mut archive.container, &mut archive.tree)
        .unwrap()
        .unwrap()
}

#[test]
fn empty() {
    let mut archive = setup_archive(0);
    let mut entry = load_first(&mut archive);
    let mut buf = [b'x'; 8];

    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 8]);
}

#[test]
fn half_part_aligned() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; 26];
    assert_eq!(entry.read(&mut buf).unwrap(), 26);
    assert_eq!(buf, (0..26).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 27];
    assert_eq!(entry.read(&mut buf).unwrap(), 27);
    assert_eq!(buf, (26..53).collect::<Vec<u8>>().as_slice());
}

#[test]
fn half_part_unaligned() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; 30];
    assert_eq!(entry.read(&mut buf).unwrap(), 30);
    assert_eq!(buf, (0..30).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 30];
    assert_eq!(entry.read(&mut buf).unwrap(), 23);
    assert_eq!(&buf[..23], (30..53).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[23..], [b'x'; 7]);
}

#[test]
fn half_all() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; HALF as usize];
    assert_eq!(entry.read(&mut buf).unwrap(), HALF as usize);
    assert_eq!(buf, (0..HALF).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn half_more() {
    let mut archive = setup_archive(HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; HALF as usize + 1];
    assert_eq!(entry.read(&mut buf).unwrap(), HALF as usize);
    assert_eq!(
        &buf[..HALF as usize],
        (0..HALF).collect::<Vec<u8>>().as_slice()
    );
    assert_eq!(&buf[HALF as usize..], [b'x']);
}

#[test]
fn full_part_aligned() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; 53];
    assert_eq!(entry.read(&mut buf).unwrap(), 53);
    assert_eq!(buf, (0..53).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 53];
    assert_eq!(entry.read(&mut buf).unwrap(), 53);
    assert_eq!(buf, (53..106).collect::<Vec<u8>>().as_slice());
}

#[test]
fn full_part_unaligned() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; 55];
    assert_eq!(entry.read(&mut buf).unwrap(), 55);
    assert_eq!(buf, (0..55).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 55];
    assert_eq!(entry.read(&mut buf).unwrap(), 51);
    assert_eq!(&buf[..51], (55..106).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[51..], [b'x'; 4]);
}

#[test]
fn full_all() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; FULL as usize];
    assert_eq!(entry.read(&mut buf).unwrap(), FULL as usize);
    assert_eq!(buf, (0..FULL).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_more() {
    let mut archive = setup_archive(FULL);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; FULL as usize + 1];
    assert_eq!(entry.read(&mut buf).unwrap(), FULL as usize);
    assert_eq!(
        &buf[..FULL as usize],
        (0..FULL).collect::<Vec<u8>>().as_slice()
    );
    assert_eq!(&buf[FULL as usize..], [b'x']);
}

#[test]
fn full_half_part_aligned() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; 53];
    assert_eq!(entry.read(&mut buf).unwrap(), 53);
    assert_eq!(buf, (0..53).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 53];
    assert_eq!(entry.read(&mut buf).unwrap(), 53);
    assert_eq!(buf, (53..106).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 53];
    assert_eq!(entry.read(&mut buf).unwrap(), 53);
    assert_eq!(buf, (106..159).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_part_unaligned() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; 55];
    assert_eq!(entry.read(&mut buf).unwrap(), 55);
    assert_eq!(buf, (0..55).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 55];
    assert_eq!(entry.read(&mut buf).unwrap(), 51);
    assert_eq!(&buf[..51], (55..106).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[51..], [b'x'; 4]);

    let mut buf = [b'x'; 30];
    assert_eq!(entry.read(&mut buf).unwrap(), 30);
    assert_eq!(buf, (106..136).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 30];
    assert_eq!(entry.read(&mut buf).unwrap(), 23);
    assert_eq!(&buf[..23], (136..159).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[23..], [b'x'; 7]);

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_all() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; FULL as usize];
    assert_eq!(entry.read(&mut buf).unwrap(), FULL as usize);
    assert_eq!(buf, (0..FULL).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; HALF as usize];
    assert_eq!(entry.read(&mut buf).unwrap(), HALF as usize);
    assert_eq!(buf, (FULL..FULL + HALF).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_more() {
    let mut archive = setup_archive(FULL + HALF);
    let mut entry = load_first(&mut archive);

    let mut buf = [b'x'; FULL as usize + 1];
    assert_eq!(entry.read(&mut buf).unwrap(), FULL as usize);
    assert_eq!(
        &buf[..FULL as usize],
        (0..FULL).collect::<Vec<u8>>().as_slice()
    );
    assert_eq!(&buf[FULL as usize..], [b'x']);

    let mut buf = [b'x'; HALF as usize + 1];
    assert_eq!(entry.read(&mut buf).unwrap(), HALF as usize);
    assert_eq!(
        &buf[..HALF as usize],
        (FULL..FULL + HALF).collect::<Vec<u8>>().as_slice()
    );
    assert_eq!(&buf[HALF as usize..], [b'x']);
}

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

use crate::entry::tests::entry::setup_archive;

#[test]
fn empty() {
    let mut archive = setup_archive(0);
    let mut entry = archive.first().unwrap().unwrap();
    let mut buf = [b'x'; 8];

    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 8]);
}

#[test]
fn half_part_aligned() {
    let mut archive = setup_archive(46);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 23];
    assert_eq!(entry.read(&mut buf).unwrap(), 23);
    assert_eq!(buf, (0..23).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 23];
    assert_eq!(entry.read(&mut buf).unwrap(), 23);
    assert_eq!(buf, (23..46).collect::<Vec<u8>>().as_slice());
}

#[test]
fn half_part_unaligned() {
    let mut archive = setup_archive(46);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 25];
    assert_eq!(entry.read(&mut buf).unwrap(), 25);
    assert_eq!(buf, (0..25).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 25];
    assert_eq!(entry.read(&mut buf).unwrap(), 21);
    assert_eq!(&buf[..21], (25..46).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[21..], [b'x'; 4]);
}

#[test]
fn half_all() {
    let mut archive = setup_archive(46);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (0..46).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn half_more() {
    let mut archive = setup_archive(46);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 47];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(&buf[..46], (0..46).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[46..], [b'x']);
}

#[test]
fn full_part_aligned() {
    let mut archive = setup_archive(92);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (0..46).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (46..92).collect::<Vec<u8>>().as_slice());
}

#[test]
fn full_part_unaligned() {
    let mut archive = setup_archive(92);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 50];
    assert_eq!(entry.read(&mut buf).unwrap(), 50);
    assert_eq!(buf, (0..50).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 50];
    assert_eq!(entry.read(&mut buf).unwrap(), 42);
    assert_eq!(&buf[..42], (50..92).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[42..], [b'x'; 8]);
}

#[test]
fn full_all() {
    let mut archive = setup_archive(92);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 92];
    assert_eq!(entry.read(&mut buf).unwrap(), 92);
    assert_eq!(buf, (0..92).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_more() {
    let mut archive = setup_archive(92);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 93];
    assert_eq!(entry.read(&mut buf).unwrap(), 92);
    assert_eq!(&buf[..92], (0..92).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[92..], [b'x']);
}

#[test]
fn full_half_part_aligned() {
    let mut archive = setup_archive(138);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (0..46).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (46..92).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (92..138).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_part_unaligned() {
    let mut archive = setup_archive(138);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 50];
    assert_eq!(entry.read(&mut buf).unwrap(), 50);
    assert_eq!(buf, (0..50).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 50];
    assert_eq!(entry.read(&mut buf).unwrap(), 42);
    assert_eq!(&buf[..42], (50..92).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[42..], [b'x'; 8]);

    let mut buf = [b'x'; 25];
    assert_eq!(entry.read(&mut buf).unwrap(), 25);
    assert_eq!(buf, (92..117).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 25];
    assert_eq!(entry.read(&mut buf).unwrap(), 21);
    assert_eq!(&buf[..21], (117..138).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[21..], [b'x'; 4]);

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_all() {
    let mut archive = setup_archive(138);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 92];
    assert_eq!(entry.read(&mut buf).unwrap(), 92);
    assert_eq!(buf, (0..92).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 46];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(buf, (92..138).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_more() {
    let mut archive = setup_archive(138);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 93];
    assert_eq!(entry.read(&mut buf).unwrap(), 92);
    assert_eq!(&buf[..92], (0..92).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[92..], [b'x']);

    let mut buf = [b'x'; 47];
    assert_eq!(entry.read(&mut buf).unwrap(), 46);
    assert_eq!(&buf[..46], (92..138).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[46..], [b'x']);
}

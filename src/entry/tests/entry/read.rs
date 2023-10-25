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
    let mut archive = setup_archive(38);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 19];
    assert_eq!(entry.read(&mut buf).unwrap(), 19);
    assert_eq!(buf, (0..19).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 19];
    assert_eq!(entry.read(&mut buf).unwrap(), 19);
    assert_eq!(buf, (19..38).collect::<Vec<u8>>().as_slice());
}

#[test]
fn half_part_unaligned() {
    let mut archive = setup_archive(38);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 20];
    assert_eq!(entry.read(&mut buf).unwrap(), 20);
    assert_eq!(buf, (0..20).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 20];
    assert_eq!(entry.read(&mut buf).unwrap(), 18);
    assert_eq!(&buf[..18], (20..38).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[18..], [b'x'; 2]);
}

#[test]
fn half_all() {
    let mut archive = setup_archive(38);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (0..38).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn half_more() {
    let mut archive = setup_archive(38);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 39];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(&buf[..38], (0..38).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[38..], [b'x']);
}

#[test]
fn full_part_aligned() {
    let mut archive = setup_archive(76);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (0..38).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (38..76).collect::<Vec<u8>>().as_slice());
}

#[test]
fn full_part_unaligned() {
    let mut archive = setup_archive(76);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 40];
    assert_eq!(entry.read(&mut buf).unwrap(), 40);
    assert_eq!(buf, (0..40).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 36);
    assert_eq!(&buf[..36], (40..76).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[36..], [b'x'; 2]);
}

#[test]
fn full_all() {
    let mut archive = setup_archive(76);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 76];
    assert_eq!(entry.read(&mut buf).unwrap(), 76);
    assert_eq!(buf, (0..76).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_more() {
    let mut archive = setup_archive(76);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 77];
    assert_eq!(entry.read(&mut buf).unwrap(), 76);
    assert_eq!(&buf[..76], (0..76).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[76..], [b'x']);
}

#[test]
fn full_half_part_aligned() {
    let mut archive = setup_archive(114);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (0..38).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (38..76).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (76..114).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_part_unaligned() {
    let mut archive = setup_archive(114);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 40];
    assert_eq!(entry.read(&mut buf).unwrap(), 40);
    assert_eq!(buf, (0..40).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 36);
    assert_eq!(&buf[..36], (40..76).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[36..], [b'x'; 2]);

    let mut buf = [b'x'; 20];
    assert_eq!(entry.read(&mut buf).unwrap(), 20);
    assert_eq!(buf, (76..96).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 20];
    assert_eq!(entry.read(&mut buf).unwrap(), 18);
    assert_eq!(&buf[..18], (96..114).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[18..], [b'x'; 2]);

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_all() {
    let mut archive = setup_archive(114);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 76];
    assert_eq!(entry.read(&mut buf).unwrap(), 76);
    assert_eq!(buf, (0..76).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 38];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(buf, (76..114).collect::<Vec<u8>>().as_slice());

    let mut buf = [b'x'; 2];
    assert_eq!(entry.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x'; 2]);
}

#[test]
fn full_half_more() {
    let mut archive = setup_archive(114);
    let mut entry = archive.first().unwrap().unwrap();

    let mut buf = [b'x'; 77];
    assert_eq!(entry.read(&mut buf).unwrap(), 76);
    assert_eq!(&buf[..76], (0..76).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[76..], [b'x']);

    let mut buf = [b'x'; 39];
    assert_eq!(entry.read(&mut buf).unwrap(), 38);
    assert_eq!(&buf[..38], (76..114).collect::<Vec<u8>>().as_slice());
    assert_eq!(&buf[38..], [b'x']);
}

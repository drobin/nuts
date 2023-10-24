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

use crate::entry::tests::entry_mut::lookup;
use crate::entry::Inner;
use crate::tests::setup_container_with_bsize;
use crate::Archive;

#[test]
fn no_content() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();

    archive.append("foo").build().unwrap();

    let id = lookup(&mut archive, 0).unwrap().clone();
    assert!(lookup(&mut archive, 1).is_none());

    let mut reader = archive.container.read_buf(&id).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 0);
}

#[test]
fn one_block() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();

    let mut entry = archive.append("foo").build().unwrap();
    assert_eq!(entry.write(&(0..76).collect::<Vec<u8>>()).unwrap(), 76);

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    assert!(lookup(&mut archive, 2).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 76);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());
}

#[test]
fn one_byte_one_block() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for i in 0..76 {
        assert_eq!(entry.write(&[i]).unwrap(), 1);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    assert!(lookup(&mut archive, 2).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 76);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());
}

#[test]
fn one_byte_one_half_blocks() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for i in 0..114 {
        assert_eq!(entry.write(&[i]).unwrap(), 1);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    let id2 = lookup(&mut archive, 2).unwrap().clone();
    assert!(lookup(&mut archive, 3).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 114);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());

    let buf = archive.container.read_buf_raw(&id2).unwrap();
    assert_eq!(buf[..38], (76..114).collect::<Vec<u8>>());
    assert_eq!(buf[38..], [0; 38]);
}

#[test]
fn one_byte_two_blocks() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for i in 0..152 {
        assert_eq!(entry.write(&[i]).unwrap(), 1);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    let id2 = lookup(&mut archive, 2).unwrap().clone();
    assert!(lookup(&mut archive, 3).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 152);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());

    let buf = archive.container.read_buf_raw(&id2).unwrap();
    assert_eq!(buf, (76..152).collect::<Vec<u8>>());
}

#[test]
fn two_bytes_one_block() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for buf in (0..76).collect::<Vec<u8>>().chunks(2) {
        assert_eq!(buf.len(), 2);
        assert_eq!(entry.write(buf).unwrap(), 2);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    assert!(lookup(&mut archive, 2).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 76);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());
}

#[test]
fn two_bytes_one_half_blocks() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for buf in (0..114).collect::<Vec<u8>>().chunks(2) {
        assert_eq!(buf.len(), 2);
        assert_eq!(entry.write(buf).unwrap(), 2);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    let id2 = lookup(&mut archive, 2).unwrap().clone();
    assert!(lookup(&mut archive, 3).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 114);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());

    let buf = archive.container.read_buf_raw(&id2).unwrap();
    assert_eq!(buf[..38], (76..114).collect::<Vec<u8>>());
    assert_eq!(buf[38..], [0; 38]);
}

#[test]
fn two_bytes_two_blocks() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for buf in (0..152).collect::<Vec<u8>>().chunks(2) {
        assert_eq!(buf.len(), 2);
        assert_eq!(entry.write(buf).unwrap(), 2);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    let id2 = lookup(&mut archive, 2).unwrap().clone();
    assert!(lookup(&mut archive, 3).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 152);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());

    let buf = archive.container.read_buf_raw(&id2).unwrap();
    assert_eq!(buf, (76..152).collect::<Vec<u8>>());
}

#[test]
fn three_bytes_one_block() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for buf in (0..75).collect::<Vec<u8>>().chunks(3) {
        assert_eq!(buf.len(), 3);
        assert_eq!(entry.write(buf).unwrap(), 3);
    }

    assert_eq!(entry.write(&[75, 76, 77]).unwrap(), 1);

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    assert!(lookup(&mut archive, 2).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 76);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());
}

#[test]
fn three_bytes_one_half_blocks() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for buf in (0..75).collect::<Vec<u8>>().chunks(3) {
        assert_eq!(buf.len(), 3);
        assert_eq!(entry.write(buf).unwrap(), 3);
    }

    assert_eq!(entry.write(&[75, 76, 77]).unwrap(), 1);

    for buf in (76..112).collect::<Vec<u8>>().chunks(3) {
        assert_eq!(buf.len(), 3);
        assert_eq!(entry.write(buf).unwrap(), 3);
    }

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    let id2 = lookup(&mut archive, 2).unwrap().clone();
    assert!(lookup(&mut archive, 3).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 112);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());

    let buf = archive.container.read_buf_raw(&id2).unwrap();
    assert_eq!(buf[..36], (76..112).collect::<Vec<u8>>());
    assert_eq!(buf[36..], [0; 40]);
}

#[test]
fn three_bytes_two_blocks() {
    let container = setup_container_with_bsize(76);
    let mut archive = Archive::create(container, false).unwrap();
    let mut entry = archive.append("foo").build().unwrap();

    for buf in (0..75).collect::<Vec<u8>>().chunks(3) {
        assert_eq!(buf.len(), 3);
        assert_eq!(entry.write(buf).unwrap(), 3);
    }

    assert_eq!(entry.write(&[75, 76, 77]).unwrap(), 1);

    for buf in (76..151).collect::<Vec<u8>>().chunks(3) {
        assert_eq!(buf.len(), 3);
        assert_eq!(entry.write(buf).unwrap(), 3);
    }

    assert_eq!(entry.write(&[151, 152, 153]).unwrap(), 1);

    let id0 = lookup(&mut archive, 0).unwrap().clone();
    let id1 = lookup(&mut archive, 1).unwrap().clone();
    let id2 = lookup(&mut archive, 2).unwrap().clone();
    assert!(lookup(&mut archive, 3).is_none());

    let mut reader = archive.container.read_buf(&id0).unwrap();
    let entry = reader.deserialize::<Inner>().unwrap();

    assert_eq!(entry.name, "foo");
    assert_eq!(entry.size, 152);

    let buf = archive.container.read_buf_raw(&id1).unwrap();
    assert_eq!(buf, (0..76).collect::<Vec<u8>>());

    let buf = archive.container.read_buf_raw(&id2).unwrap();
    assert_eq!(buf, (76..152).collect::<Vec<u8>>());
}

// MIT License
//
// Copyright (c) 2024 Robin Doer
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

use nuts_bytes::{Reader, Writer};
use nuts_memory::{Id, MemoryBackend};

use crate::error::Error;
use crate::pager::Pager;
use crate::tests::{into_error, setup_container_with_bsize};
use crate::tree::node::Node;

#[test]
fn new() {
    let node = Node::<MemoryBackend>::new();

    assert!(node.buf.is_empty());
    assert!(node.vec.is_empty());
}

#[test]
fn load() {
    let mut pager = Pager::new(setup_container_with_bsize(20));
    let id = pager.aquire().unwrap();

    let mut writer = Writer::new(vec![]);

    writer.write(b"node").unwrap();
    writer.write(&3u32).unwrap();
    writer.write(&4711u32).unwrap();
    writer.write(&4712u32).unwrap();
    writer.write(&4713u32).unwrap();

    pager.write(&id, &writer.into_target()).unwrap();

    let mut node = Node::new();
    node.load(&id, &mut pager).unwrap();

    assert_eq!(node.vec.len(), 3);
    assert_eq!(node.vec[0], "4711".parse().unwrap());
    assert_eq!(node.vec[1], "4712".parse().unwrap());
    assert_eq!(node.vec[2], "4713".parse().unwrap());
}

#[test]
fn load_inval_node() {
    let mut pager = Pager::new(setup_container_with_bsize(20));
    let id = pager.aquire().unwrap();

    let mut writer = Writer::new(vec![]);

    writer.write(b"xode").unwrap();
    writer.write(&3u32).unwrap();
    writer.write(&4711u32).unwrap();
    writer.write(&4712u32).unwrap();
    writer.write(&4713u32).unwrap();

    pager.write(&id, &writer.into_target()).unwrap();

    let err = Node::new().load(&id, &mut pager).unwrap_err();
    let err_id = into_error!(err, Error::InvalidNode);
    assert_eq!(err_id, *id.as_ref());
}

#[test]
fn flush() {
    let mut pager = Pager::new(setup_container_with_bsize(20));
    let id = pager.aquire().unwrap();

    let mut node = Node::new();

    node.vec.push("4711".parse().unwrap());
    node.vec.push("4712".parse().unwrap());
    node.vec.push("4713".parse().unwrap());

    node.flush(&id, &mut pager).unwrap();

    let mut buf = [0; 20];

    pager.read(&id, &mut buf).unwrap();

    let mut reader = Reader::new(buf.as_slice());

    assert_eq!(reader.read::<[u8; 4]>().unwrap(), *b"node");
    assert_eq!(reader.read::<u32>().unwrap(), 3);
    assert_eq!(reader.read::<Id>().unwrap(), "4711".parse().unwrap());
    assert_eq!(reader.read::<Id>().unwrap(), "4712".parse().unwrap());
    assert_eq!(reader.read::<Id>().unwrap(), "4713".parse().unwrap());
}

#[test]
fn flush_nospace() {
    let mut pager = Pager::new(setup_container_with_bsize(19));
    let id = pager.aquire().unwrap();

    let mut node = Node::new();

    node.vec.push("4711".parse().unwrap());
    node.vec.push("4712".parse().unwrap());
    node.vec.push("4713".parse().unwrap());

    let err = node.flush(&id, &mut pager).unwrap_err();
    assert!(matches!(err, Error::InvalidBlockSize));
}

#[test]
fn aquire() {
    let mut pager = Pager::new(setup_container_with_bsize(16));
    let mut node = Node::<MemoryBackend>::new();

    node.aquire(&mut pager).unwrap();

    assert_eq!(node.vec.len(), 1);

    node.load(&node.vec[0].clone(), &mut pager).unwrap();

    assert!(node.vec.is_empty());
}

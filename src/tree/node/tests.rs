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

use nuts_bytes::{Reader, Writer};
use nuts_container::backend::BlockId;
use nuts_container::memory::{Id, MemoryBackend};

use crate::pager::Pager;
use crate::tests::setup_container_with_bsize;
use crate::tree::node::Node;

#[test]
fn new() {
    let pager = Pager::new(setup_container_with_bsize(12));
    let node = Node::<MemoryBackend>::new(&pager);

    assert_eq!(node.len(), 3);
    assert!(node[0].is_null());
    assert!(node[1].is_null());
    assert!(node[2].is_null());
}

#[test]
fn aquire() {
    let mut pager = Pager::new(setup_container_with_bsize(16));
    let id = Node::aquire(&mut pager).unwrap();

    let mut buf = vec![0; 32];
    assert_eq!(pager.into_container().read(&id, &mut buf).unwrap(), 16);

    let mut reader = Reader::new(&buf[..16]);

    for _ in 0..4 {
        assert!(reader.deserialize::<Id>().unwrap().is_null());
    }

    assert!(reader.as_ref().is_empty());
}

#[test]
fn fill() {
    let mut pager = Pager::new(setup_container_with_bsize(16));
    let mut writer = Writer::new(vec![]);

    writer.serialize(&1u32).unwrap();
    writer.serialize(&2u32).unwrap();
    writer.serialize(&3u32).unwrap();
    writer.serialize(&4u32).unwrap();

    let id = pager.aquire().unwrap();
    assert_eq!(pager.write(&id, &writer.into_target()).unwrap(), 16);

    let mut node = Node::new(&pager);

    node.fill(&mut pager, &id).unwrap();
    assert_eq!(
        node,
        [
            "1".parse().unwrap(),
            "2".parse().unwrap(),
            "3".parse().unwrap(),
            "4".parse().unwrap()
        ]
    );
}

#[test]
fn flush() {
    let mut pager = Pager::new(setup_container_with_bsize(16));
    let id = pager.aquire().unwrap();

    let mut node = Node::<MemoryBackend>::new(&pager);

    node[0] = "1".parse().unwrap();
    node[1] = "2".parse().unwrap();
    node[2] = "3".parse().unwrap();
    node[3] = "4".parse().unwrap();
    node.flush(&mut pager, &id).unwrap();

    let mut buf = [0; 16];
    assert_eq!(pager.into_container().read(&id, &mut buf).unwrap(), 16);
    assert_eq!(buf, [0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4]);
}

#[test]
#[should_panic(
    expected = "node underflow detected, about to write 4 ids (4 bytes each) into 20 bytes"
)]
fn flush_underflow() {
    let vec = (1..=4)
        .map(|i| i.to_string().parse().unwrap())
        .collect::<Vec<Id>>();

    let mut pager = Pager::new(setup_container_with_bsize(20));
    let id = pager.aquire().unwrap();

    let _ = Node(vec).flush(&mut pager, &id);
}

#[test]
#[should_panic(
    expected = "node overflow detected, about to write 4 ids (4 bytes each) into 15 bytes"
)]
fn flush_overflow() {
    let vec = (1..=4)
        .map(|i| i.to_string().parse().unwrap())
        .collect::<Vec<Id>>();

    let mut pager = Pager::new(setup_container_with_bsize(15));
    let id = pager.aquire().unwrap();

    let _ = Node(vec).flush(&mut pager, &id);
}

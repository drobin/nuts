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

use nuts_backend::BlockId;
use nuts_bytes::{Reader, Writer};
use nuts_container::container::Container;
use nuts_memory::{Id, MemoryBackend};

use crate::error::Error;
use crate::pager::Pager;
use crate::tests::setup_container_with_bsize;
use crate::tree::Tree;

const BSIZE: u32 = 8;

macro_rules! assert_direct {
    ($tree:expr, $nblocks:expr, $expected:expr) => {
        assert_eq!($tree.nblocks, $nblocks);

        for (l, r) in $tree.direct.iter().zip($expected.iter()) {
            assert_eq!(l, r);
        }
    };
}

fn read_node(container: &mut Container<MemoryBackend>, id: &Id) -> Vec<Id> {
    let mut buf = [0; BSIZE as usize];

    assert_eq!(container.read(id, &mut buf).unwrap(), 8);

    let mut reader = Reader::new(&buf[..]);

    let mut vec = vec![];

    for _ in 0..2 {
        vec.push(reader.read().unwrap());
    }

    vec
}

#[test]
fn ser() {
    let tree = Tree::<MemoryBackend> {
        direct: [
            "1".parse().unwrap(),
            "2".parse().unwrap(),
            "3".parse().unwrap(),
            "4".parse().unwrap(),
            "5".parse().unwrap(),
            "6".parse().unwrap(),
            "7".parse().unwrap(),
            "8".parse().unwrap(),
            "9".parse().unwrap(),
            "10".parse().unwrap(),
            "11".parse().unwrap(),
            "12".parse().unwrap(),
        ],
        indirect: "13".parse().unwrap(),
        d_indirect: "14".parse().unwrap(),
        t_indirect: "15".parse().unwrap(),
        nblocks: 16,
        cache: vec![],
    };
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&tree).unwrap(), 68);
    assert_eq!(
        writer.into_target(),
        [
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0,
            0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14,
            0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 16
        ]
    );
}

#[test]
fn de() {
    let mut reader = Reader::new(
        [
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0, 7, 0,
            0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12, 0, 0, 0, 13, 0, 0, 0, 14,
            0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 16,
        ]
        .as_slice(),
    );
    let tree = reader.read::<Tree<MemoryBackend>>().unwrap();

    assert_eq!(tree.direct[0], "1".parse().unwrap());
    assert_eq!(tree.direct[1], "2".parse().unwrap());
    assert_eq!(tree.direct[2], "3".parse().unwrap());
    assert_eq!(tree.direct[3], "4".parse().unwrap());
    assert_eq!(tree.direct[4], "5".parse().unwrap());
    assert_eq!(tree.direct[5], "6".parse().unwrap());
    assert_eq!(tree.direct[6], "7".parse().unwrap());
    assert_eq!(tree.direct[7], "8".parse().unwrap());
    assert_eq!(tree.direct[8], "9".parse().unwrap());
    assert_eq!(tree.direct[9], "10".parse().unwrap());
    assert_eq!(tree.direct[10], "11".parse().unwrap());
    assert_eq!(tree.direct[11], "12".parse().unwrap());
    assert_eq!(tree.indirect, "13".parse().unwrap());
    assert_eq!(tree.d_indirect, "14".parse().unwrap());
    assert_eq!(tree.t_indirect, "15".parse().unwrap());
    assert_eq!(tree.nblocks, 16);
}

#[test]
fn aquire() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = Tree::<MemoryBackend>::new();

    // direct

    let mut direct = [Id::null(); 12];

    for i in 0..12 {
        direct[i] = tree.aquire(&mut pager).unwrap().clone();
        assert_direct!(tree, i as u64 + 1, direct);
        assert!(tree.indirect.is_null());
        assert!(tree.d_indirect.is_null());
        assert!(tree.t_indirect.is_null());
    }

    // indirect

    let mut indirect = [Id::null(); 2];

    for i in 0..2 {
        indirect[i] = tree.aquire(&mut pager).unwrap().clone();

        assert_direct!(tree, 12 + i as u64 + 1, direct);
        assert_eq!(&indirect[..], read_node(&mut pager, &tree.indirect));
        assert!(tree.d_indirect.is_null());
        assert!(tree.t_indirect.is_null());
    }

    // d_indirect

    let mut d_indirect = [Id::null(); 4];

    for i in 0..4 {
        d_indirect[i] = tree.aquire(&mut pager).unwrap().clone();

        assert_direct!(tree, 12 + 2 + i as u64 + 1, direct);
        assert_eq!(&indirect[..], read_node(&mut pager, &tree.indirect));
        assert!(tree.t_indirect.is_null());

        let d_node = read_node(&mut pager, &tree.d_indirect);

        assert_eq!(d_indirect[..2], read_node(&mut pager, &d_node[0]));

        if d_node[1].is_null() {
            assert_eq!(d_indirect[2..], [Id::null(); 2]);
        } else {
            assert_eq!(d_indirect[2..], read_node(&mut pager, &d_node[1]));
        }
    }

    // t_indirect

    let mut t_indirect = [Id::null(); 8];

    for i in 0..8 {
        t_indirect[i] = tree.aquire(&mut pager).unwrap().clone();

        assert_direct!(tree, 12 + 2 + 4 + i as u64 + 1, direct);
        assert_eq!(&indirect[..], read_node(&mut pager, &tree.indirect));

        let d_node = read_node(&mut pager, &tree.d_indirect);

        let d_leafs: Vec<Id> = d_node
            .iter()
            .map(|id| {
                if id.is_null() {
                    vec![Id::null(); 2]
                } else {
                    read_node(&mut pager, id)
                }
            })
            .flatten()
            .collect();

        assert_eq!(d_leafs, d_indirect);

        let t_node = read_node(&mut pager, &tree.t_indirect);
        let t_leafs: Vec<Id> = t_node
            .iter()
            .map(|id| {
                if id.is_null() {
                    vec![Id::null(); 2]
                } else {
                    read_node(&mut pager, id)
                }
            })
            .flatten()
            .collect::<Vec<Id>>()
            .iter()
            .map(|id| {
                if id.is_null() {
                    vec![Id::null(); 2]
                } else {
                    read_node(&mut pager, id)
                }
            })
            .flatten()
            .collect();

        assert_eq!(t_leafs, t_indirect);
    }

    let err = tree.aquire(&mut pager).unwrap_err();
    assert!(matches!(err, Error::Full));
}

#[test]
fn lookup() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = Tree::<MemoryBackend>::new();
    let mut id_vec = vec![];

    for _ in 0..26 {
        let id = tree.aquire(&mut pager).unwrap().clone();
        id_vec.push(id);
    }

    for i in 0..26 {
        let id = tree.lookup(&mut pager, i).unwrap().unwrap();
        assert_eq!(&id_vec[i], id);
    }

    assert!(tree.lookup(&mut pager, 26).is_none());
}

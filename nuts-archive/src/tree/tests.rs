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

mod acquire;
mod lookup;

use nuts_bytes::{Reader, Writer};
use nuts_memory::MemoryBackend;

use crate::tree::cache::Cache;
use crate::tree::Tree;

const BSIZE: u32 = 16;

macro_rules! _id {
    ($id:expr) => {
        $id.parse::<crate::id::Id<nuts_memory::MemoryBackend>>()
            .unwrap()
    };
}

macro_rules! assert_direct {
    ($tree:expr, $pager:expr) => {
        assert_eq!($tree.lookup(&mut $pager, 0).unwrap().unwrap(), &_id!("1"));
        assert_eq!($tree.lookup(&mut $pager, 1).unwrap().unwrap(), &_id!("2"));
        assert_eq!($tree.lookup(&mut $pager, 2).unwrap().unwrap(), &_id!("3"));
        assert_eq!($tree.lookup(&mut $pager, 3).unwrap().unwrap(), &_id!("4"));
        assert_eq!($tree.lookup(&mut $pager, 4).unwrap().unwrap(), &_id!("5"));
        assert_eq!($tree.lookup(&mut $pager, 5).unwrap().unwrap(), &_id!("6"));
        assert_eq!($tree.lookup(&mut $pager, 6).unwrap().unwrap(), &_id!("7"));
        assert_eq!($tree.lookup(&mut $pager, 7).unwrap().unwrap(), &_id!("8"));
        assert_eq!($tree.lookup(&mut $pager, 8).unwrap().unwrap(), &_id!("9"));
        assert_eq!($tree.lookup(&mut $pager, 9).unwrap().unwrap(), &_id!("10"));
        assert_eq!($tree.lookup(&mut $pager, 10).unwrap().unwrap(), &_id!("11"));
        assert_eq!($tree.lookup(&mut $pager, 11).unwrap().unwrap(), &_id!("12"));
    };
}

pub(crate) use {_id, assert_direct};

fn make_tree() -> Tree<MemoryBackend> {
    Tree::<MemoryBackend> {
        ids: vec![],
        nblocks: 0,
        cache: Cache::new(),
    }
}

#[test]
fn ser() {
    let tree = Tree::<MemoryBackend> {
        ids: vec![
            _id!("1"),
            _id!("2"),
            _id!("3"),
            _id!("4"),
            _id!("5"),
            _id!("6"),
            _id!("7"),
            _id!("8"),
            _id!("9"),
            _id!("10"),
            _id!("11"),
            _id!("12"),
            _id!("13"),
            _id!("14"),
            _id!("15"),
        ],
        nblocks: 16,
        cache: Cache::new(),
    };
    let mut writer = Writer::new(vec![]);

    assert_eq!(writer.write(&tree).unwrap(), 76);
    assert_eq!(
        writer.into_target(),
        [
            0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0,
            0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12, 0,
            0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 16
        ]
    );
}

#[test]
fn de() {
    let mut reader = Reader::new(
        [
            0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 5, 0,
            0, 0, 6, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 9, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 12, 0,
            0, 0, 13, 0, 0, 0, 14, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 16,
        ]
        .as_slice(),
    );
    let tree = reader.read::<Tree<MemoryBackend>>().unwrap();

    assert_eq!(
        tree.ids,
        [
            _id!("1"),
            _id!("2"),
            _id!("3"),
            _id!("4"),
            _id!("5"),
            _id!("6"),
            _id!("7"),
            _id!("8"),
            _id!("9"),
            _id!("10"),
            _id!("11"),
            _id!("12"),
            _id!("13"),
            _id!("14"),
            _id!("15"),
        ]
    );
    assert_eq!(tree.nblocks, 16);
}

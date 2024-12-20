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

use nuts_bytes::Writer;
use nuts_memory::MemoryBackend;

use crate::pager::Pager;
use crate::tests::setup_container_with_bsize;
use crate::tree::tests::{_id, assert_direct, make_tree, BSIZE};
use crate::tree::Tree;

macro_rules! make_node {
    ($pager:expr, $parent:expr, $num:literal => $($id:expr),+) => {{
        let mut writer = Writer::new(vec![]);

        writer.write(b"node").unwrap();
        writer.write(&($num as u32)).unwrap();

        $(
            writer.write(&$id).unwrap();
        )*

        $pager.write(&$parent, &writer.into_target()).unwrap();
    }};
}

fn make_indirect_tree(num: u64, pager: &mut Pager<MemoryBackend>) -> Tree<MemoryBackend> {
    let mut tree = make_direct_tree(12, pager);
    let indirect = pager.acquire().unwrap();

    make_node!(pager, indirect, 2 => pager.acquire().unwrap(), pager.acquire().unwrap());

    tree.ids.push(indirect);
    tree.nblocks = 12 + num;

    tree
}

fn make_d_indirect_tree(num: u64, pager: &mut Pager<MemoryBackend>) -> Tree<MemoryBackend> {
    let mut tree = make_indirect_tree(2, pager);

    //               id
    //                |
    //           ( 1_0 1_1 )
    //            /       \
    // ( 1_0_1 1_0_1 )  ( 1_1_0_ 1_1_1)

    let id = pager.acquire().unwrap();

    let id_1_0 = pager.acquire().unwrap();
    let id_1_1 = pager.acquire().unwrap();

    let id_1_0_0 = pager.acquire().unwrap();
    let id_1_0_1 = pager.acquire().unwrap();

    let id_1_1_0 = pager.acquire().unwrap();
    let id_1_1_1 = pager.acquire().unwrap();

    make_node!(pager, id, 2 => id_1_0, id_1_1);
    make_node!(pager, id_1_0, 2 => id_1_0_0, id_1_0_1);
    make_node!(pager, id_1_1, 2 => id_1_1_0, id_1_1_1);

    tree.ids.push(id);
    tree.nblocks = 12 + 2 + num;

    tree
}

fn make_t_indirect_tree(num: u64, pager: &mut Pager<MemoryBackend>) -> Tree<MemoryBackend> {
    let mut tree = make_d_indirect_tree(4, pager);

    //               id
    //                |
    //           ( 1_0 1_1 )
    //            /       \
    // ( 1_0_1 1_0_1 )  ( 1_1_0_ 1_1_1)

    let id = pager.acquire().unwrap();

    let id_1_0 = pager.acquire().unwrap();
    let id_1_1 = pager.acquire().unwrap();

    let id_1_0_0 = pager.acquire().unwrap();
    let id_1_0_1 = pager.acquire().unwrap();

    let id_1_1_0 = pager.acquire().unwrap();
    let id_1_1_1 = pager.acquire().unwrap();

    let id_1_0_0_0 = pager.acquire().unwrap();
    let id_1_0_0_1 = pager.acquire().unwrap();

    let id_1_0_1_0 = pager.acquire().unwrap();
    let id_1_0_1_1 = pager.acquire().unwrap();

    let id_1_1_0_0 = pager.acquire().unwrap();
    let id_1_1_0_1 = pager.acquire().unwrap();

    let id_1_1_1_0 = pager.acquire().unwrap();
    let id_1_1_1_1 = pager.acquire().unwrap();

    make_node!(pager, id, 2 => id_1_0, id_1_1);
    make_node!(pager, id_1_0, 2 => id_1_0_0, id_1_0_1);
    make_node!(pager, id_1_1, 2 => id_1_1_0, id_1_1_1);
    make_node!(pager, id_1_0_0, 2 => id_1_0_0_0, id_1_0_0_1);
    make_node!(pager, id_1_0_1, 2 => id_1_0_1_0, id_1_0_1_1);
    make_node!(pager, id_1_1_0, 2 => id_1_1_0_0, id_1_1_0_1);
    make_node!(pager, id_1_1_1, 2 => id_1_1_1_0, id_1_1_1_1);

    tree.ids.push(id);
    tree.nblocks = 12 + 2 + 4 + num;

    tree
}

macro_rules! assert_indirect {
    ($tree:expr, $pager:expr) => {
        assert_eq!($tree.lookup(&mut $pager, 12).unwrap().unwrap(), &_id!("14"));
        assert_eq!($tree.lookup(&mut $pager, 13).unwrap().unwrap(), &_id!("15"));
    };
}

macro_rules! assert_d_indirect {
    ($tree:expr, $pager:expr) => {
        assert_eq!($tree.lookup(&mut $pager, 14).unwrap().unwrap(), &_id!("19"));
        assert_eq!($tree.lookup(&mut $pager, 15).unwrap().unwrap(), &_id!("20"));
        assert_eq!($tree.lookup(&mut $pager, 16).unwrap().unwrap(), &_id!("21"));
        assert_eq!($tree.lookup(&mut $pager, 17).unwrap().unwrap(), &_id!("22"));
    };
}

fn make_direct_tree(num: u64, pager: &mut Pager<MemoryBackend>) -> Tree<MemoryBackend> {
    let mut tree = make_tree();

    tree.nblocks = num;

    for _ in 0..num {
        tree.ids.push(pager.acquire().unwrap());
    }

    tree
}

#[test]
fn direct_0() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(0, &mut pager);

    assert!(tree.lookup(&mut pager, 0).is_none());
}

#[test]
fn direct_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(1, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert!(tree.lookup(&mut pager, 1).is_none());
}

#[test]
fn direct_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(2, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert!(tree.lookup(&mut pager, 2).is_none());
}

#[test]
fn direct_3() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(3, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert!(tree.lookup(&mut pager, 3).is_none());
}

#[test]
fn direct_4() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(4, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert!(tree.lookup(&mut pager, 4).is_none());
}

#[test]
fn direct_5() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(5, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert!(tree.lookup(&mut pager, 5).is_none());
}

#[test]
fn direct_6() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(6, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert_eq!(tree.lookup(&mut pager, 5).unwrap().unwrap(), &_id!("6"));
    assert!(tree.lookup(&mut pager, 6).is_none());
}

#[test]
fn direct_7() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(7, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert_eq!(tree.lookup(&mut pager, 5).unwrap().unwrap(), &_id!("6"));
    assert_eq!(tree.lookup(&mut pager, 6).unwrap().unwrap(), &_id!("7"));
    assert!(tree.lookup(&mut pager, 7).is_none());
}

#[test]
fn direct_8() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(8, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert_eq!(tree.lookup(&mut pager, 5).unwrap().unwrap(), &_id!("6"));
    assert_eq!(tree.lookup(&mut pager, 6).unwrap().unwrap(), &_id!("7"));
    assert_eq!(tree.lookup(&mut pager, 7).unwrap().unwrap(), &_id!("8"));
    assert!(tree.lookup(&mut pager, 8).is_none());
}

#[test]
fn direct_9() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(9, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert_eq!(tree.lookup(&mut pager, 5).unwrap().unwrap(), &_id!("6"));
    assert_eq!(tree.lookup(&mut pager, 6).unwrap().unwrap(), &_id!("7"));
    assert_eq!(tree.lookup(&mut pager, 7).unwrap().unwrap(), &_id!("8"));
    assert_eq!(tree.lookup(&mut pager, 8).unwrap().unwrap(), &_id!("9"));
    assert!(tree.lookup(&mut pager, 9).is_none());
}

#[test]
fn direct_10() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(10, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert_eq!(tree.lookup(&mut pager, 5).unwrap().unwrap(), &_id!("6"));
    assert_eq!(tree.lookup(&mut pager, 6).unwrap().unwrap(), &_id!("7"));
    assert_eq!(tree.lookup(&mut pager, 7).unwrap().unwrap(), &_id!("8"));
    assert_eq!(tree.lookup(&mut pager, 8).unwrap().unwrap(), &_id!("9"));
    assert_eq!(tree.lookup(&mut pager, 9).unwrap().unwrap(), &_id!("10"));
    assert!(tree.lookup(&mut pager, 10).is_none());
}

#[test]
fn direct_11() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(11, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert_eq!(tree.lookup(&mut pager, 4).unwrap().unwrap(), &_id!("5"));
    assert_eq!(tree.lookup(&mut pager, 5).unwrap().unwrap(), &_id!("6"));
    assert_eq!(tree.lookup(&mut pager, 6).unwrap().unwrap(), &_id!("7"));
    assert_eq!(tree.lookup(&mut pager, 7).unwrap().unwrap(), &_id!("8"));
    assert_eq!(tree.lookup(&mut pager, 8).unwrap().unwrap(), &_id!("9"));
    assert_eq!(tree.lookup(&mut pager, 9).unwrap().unwrap(), &_id!("10"));
    assert_eq!(tree.lookup(&mut pager, 10).unwrap().unwrap(), &_id!("11"));
    assert!(tree.lookup(&mut pager, 11).is_none());
}

#[test]
fn direct_12() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_direct_tree(12, &mut pager);

    assert_direct!(tree, pager);
    assert!(tree.lookup(&mut pager, 12).is_none());
}

#[test]
fn indirect_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_indirect_tree(1, &mut pager);

    assert_direct!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 12).unwrap().unwrap(), &_id!("14"));
    assert!(tree.lookup(&mut pager, 13).is_none());
}

#[test]
fn indirect_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_indirect_tree(2, &mut pager);

    assert_direct!(tree, pager);

    assert_indirect!(tree, pager);
    assert!(tree.lookup(&mut pager, 14).is_none());
}

#[test]
fn d_indirect_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_d_indirect_tree(1, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager); // ..=15

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("19"));
    assert!(tree.lookup(&mut pager, 15).is_none());
}

#[test]
fn d_indirect_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_d_indirect_tree(2, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager); // ..=15

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("19"));
    assert_eq!(tree.lookup(&mut pager, 15).unwrap().unwrap(), &_id!("20"));
    assert!(tree.lookup(&mut pager, 16).is_none());
}

#[test]
fn d_indirect_3() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_d_indirect_tree(3, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager); // ..=15

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("19"));
    assert_eq!(tree.lookup(&mut pager, 15).unwrap().unwrap(), &_id!("20"));
    assert_eq!(tree.lookup(&mut pager, 16).unwrap().unwrap(), &_id!("21"));
    assert!(tree.lookup(&mut pager, 17).is_none());
}

#[test]
fn d_indirect_4() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_d_indirect_tree(4, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager); // ..=15

    assert_d_indirect!(tree, pager);
    assert!(tree.lookup(&mut pager, 18).is_none());
}

#[test]
fn t_indirect_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(1, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert!(tree.lookup(&mut pager, 19).is_none());
}

#[test]
fn t_indirect_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(2, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert!(tree.lookup(&mut pager, 20).is_none());
}

#[test]
fn t_indirect_3() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(3, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("32"));
    assert!(tree.lookup(&mut pager, 21).is_none());
}

#[test]
fn t_indirect_4() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(4, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("32"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("33"));
    assert!(tree.lookup(&mut pager, 22).is_none());
}

#[test]
fn t_indirect_5() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(5, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("32"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("33"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("34"));
    assert!(tree.lookup(&mut pager, 23).is_none());
}

#[test]
fn t_indirect_6() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(6, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("32"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("33"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("34"));
    assert_eq!(tree.lookup(&mut pager, 23).unwrap().unwrap(), &_id!("35"));
    assert!(tree.lookup(&mut pager, 24).is_none());
}

#[test]
fn t_indirect_7() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(7, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("32"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("33"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("34"));
    assert_eq!(tree.lookup(&mut pager, 23).unwrap().unwrap(), &_id!("35"));
    assert_eq!(tree.lookup(&mut pager, 24).unwrap().unwrap(), &_id!("36"));
    assert!(tree.lookup(&mut pager, 25).is_none());
}

#[test]
fn t_indirect_8() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_t_indirect_tree(8, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager); // ..=22

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("31"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("32"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("33"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("34"));
    assert_eq!(tree.lookup(&mut pager, 23).unwrap().unwrap(), &_id!("35"));
    assert_eq!(tree.lookup(&mut pager, 24).unwrap().unwrap(), &_id!("36"));
    assert_eq!(tree.lookup(&mut pager, 25).unwrap().unwrap(), &_id!("37"));
    assert!(tree.lookup(&mut pager, 26).is_none());
}

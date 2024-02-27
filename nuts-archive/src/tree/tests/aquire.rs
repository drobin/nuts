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

use nuts_memory::MemoryBackend;

use crate::pager::Pager;
use crate::tests::setup_container_with_bsize;
use crate::tree::tests::{_id, assert_direct, make_tree};
use crate::tree::Tree;
use crate::Error;

const BSIZE: u32 = 12;

macro_rules! assert_indirect {
    ($tree:expr, $pager:expr) => {
        assert_eq!($tree.lookup(&mut $pager, 12).unwrap().unwrap(), &_id!("14"));
        assert_eq!($tree.lookup(&mut $pager, 13).unwrap().unwrap(), &_id!("15"));
    };
}

macro_rules! assert_d_indirect {
    ($tree:expr, $pager:expr) => {
        assert_eq!($tree.lookup(&mut $pager, 14).unwrap().unwrap(), &_id!("18"));
        assert_eq!($tree.lookup(&mut $pager, 15).unwrap().unwrap(), &_id!("19"));
        assert_eq!($tree.lookup(&mut $pager, 16).unwrap().unwrap(), &_id!("21"));
        assert_eq!($tree.lookup(&mut $pager, 17).unwrap().unwrap(), &_id!("22"));
    };
}

macro_rules! assert_t_indirect {
    ($tree:expr, $pager:expr) => {
        assert_eq!($tree.lookup(&mut $pager, 18).unwrap().unwrap(), &_id!("26"));
        assert_eq!($tree.lookup(&mut $pager, 19).unwrap().unwrap(), &_id!("27"));
        assert_eq!($tree.lookup(&mut $pager, 20).unwrap().unwrap(), &_id!("29"));
        assert_eq!($tree.lookup(&mut $pager, 21).unwrap().unwrap(), &_id!("30"));
        assert_eq!($tree.lookup(&mut $pager, 22).unwrap().unwrap(), &_id!("33"));
        assert_eq!($tree.lookup(&mut $pager, 23).unwrap().unwrap(), &_id!("34"));
        assert_eq!($tree.lookup(&mut $pager, 24).unwrap().unwrap(), &_id!("36"));
        assert_eq!($tree.lookup(&mut $pager, 25).unwrap().unwrap(), &_id!("37"));
    };
}

fn aquire_direct(num: usize, tree: &mut Tree<MemoryBackend>, pager: &mut Pager<MemoryBackend>) {
    for i in 0..num {
        assert_eq!(tree.aquire(pager).unwrap(), &_id!((i + 1).to_string()));
    }
}

fn aquire_indirect(num: usize, tree: &mut Tree<MemoryBackend>, pager: &mut Pager<MemoryBackend>) {
    for i in 0..num {
        assert_eq!(tree.aquire(pager).unwrap(), &_id!((i + 14).to_string()));
    }
}

fn aquire_d_indirect(num: usize, tree: &mut Tree<MemoryBackend>, pager: &mut Pager<MemoryBackend>) {
    let results = [_id!("18"), _id!("19"), _id!("21"), _id!("22")];

    for i in 0..num {
        assert_eq!(tree.aquire(pager).unwrap(), &results[i]);
    }
}

fn aquire_t_indirect(num: usize, tree: &mut Tree<MemoryBackend>, pager: &mut Pager<MemoryBackend>) {
    let results = [
        _id!("26"),
        _id!("27"),
        _id!("29"),
        _id!("30"),
        _id!("33"),
        _id!("34"),
        _id!("36"),
        _id!("37"),
    ];

    for i in 0..num {
        assert_eq!(tree.aquire(pager).unwrap(), &results[i]);
    }
}

#[test]
fn direct_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(1, &mut tree, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert!(tree.lookup(&mut pager, 1).is_none());
}

#[test]
fn direct_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(2, &mut tree, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert!(tree.lookup(&mut pager, 2).is_none());
}

#[test]
fn direct_3() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(3, &mut tree, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert!(tree.lookup(&mut pager, 3).is_none());
}

#[test]
fn direct_4() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(4, &mut tree, &mut pager);

    assert_eq!(tree.lookup(&mut pager, 0).unwrap().unwrap(), &_id!("1"));
    assert_eq!(tree.lookup(&mut pager, 1).unwrap().unwrap(), &_id!("2"));
    assert_eq!(tree.lookup(&mut pager, 2).unwrap().unwrap(), &_id!("3"));
    assert_eq!(tree.lookup(&mut pager, 3).unwrap().unwrap(), &_id!("4"));
    assert!(tree.lookup(&mut pager, 4).is_none());
}

#[test]
fn direct_5() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(5, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(6, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(7, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(8, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(9, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(10, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(11, &mut tree, &mut pager);

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
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert!(tree.lookup(&mut pager, 12).is_none());
}

#[test]
fn indirect_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(1, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_eq!(tree.lookup(&mut pager, 12).unwrap().unwrap(), &_id!("14"));
    assert!(tree.lookup(&mut pager, 13).is_none());
}

#[test]
fn indirect_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_eq!(tree.lookup(&mut pager, 12).unwrap().unwrap(), &_id!("14"));
    assert_eq!(tree.lookup(&mut pager, 13).unwrap().unwrap(), &_id!("15"));
    assert!(tree.lookup(&mut pager, 14).is_none());
}

#[test]
fn d_indirect_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(1, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("18"));
    assert!(tree.lookup(&mut pager, 15).is_none());
}

#[test]
fn d_indirect_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(2, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("18"));
    assert_eq!(tree.lookup(&mut pager, 15).unwrap().unwrap(), &_id!("19"));
    assert!(tree.lookup(&mut pager, 16).is_none());
}

#[test]
fn d_indirect_3() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(3, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("18"));
    assert_eq!(tree.lookup(&mut pager, 15).unwrap().unwrap(), &_id!("19"));
    assert_eq!(tree.lookup(&mut pager, 16).unwrap().unwrap(), &_id!("21"));
    assert!(tree.lookup(&mut pager, 17).is_none());
}

#[test]
fn d_indirect_4() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 14).unwrap().unwrap(), &_id!("18"));
    assert_eq!(tree.lookup(&mut pager, 15).unwrap().unwrap(), &_id!("19"));
    assert_eq!(tree.lookup(&mut pager, 16).unwrap().unwrap(), &_id!("21"));
    assert_eq!(tree.lookup(&mut pager, 17).unwrap().unwrap(), &_id!("22"));
    assert!(tree.lookup(&mut pager, 18).is_none());
}

#[test]
fn t_indirect_1() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(1, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert!(tree.lookup(&mut pager, 19).is_none());
}

#[test]
fn t_indirect_2() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(2, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("27"));
    assert!(tree.lookup(&mut pager, 20).is_none());
}

#[test]
fn t_indirect_3() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(3, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("27"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("29"));
    assert!(tree.lookup(&mut pager, 21).is_none());
}

#[test]
fn t_indirect_4() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(4, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("27"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("29"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("30"));
    assert!(tree.lookup(&mut pager, 22).is_none());
}

#[test]
fn t_indirect_5() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(5, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("27"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("29"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("33"));
    assert!(tree.lookup(&mut pager, 23).is_none());
}

#[test]
fn t_indirect_6() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(6, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("27"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("29"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("33"));
    assert_eq!(tree.lookup(&mut pager, 23).unwrap().unwrap(), &_id!("34"));
    assert!(tree.lookup(&mut pager, 24).is_none());
}

#[test]
fn t_indirect_7() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(7, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);

    assert_eq!(tree.lookup(&mut pager, 18).unwrap().unwrap(), &_id!("26"));
    assert_eq!(tree.lookup(&mut pager, 19).unwrap().unwrap(), &_id!("27"));
    assert_eq!(tree.lookup(&mut pager, 20).unwrap().unwrap(), &_id!("29"));
    assert_eq!(tree.lookup(&mut pager, 21).unwrap().unwrap(), &_id!("30"));
    assert_eq!(tree.lookup(&mut pager, 22).unwrap().unwrap(), &_id!("33"));
    assert_eq!(tree.lookup(&mut pager, 23).unwrap().unwrap(), &_id!("34"));
    assert_eq!(tree.lookup(&mut pager, 24).unwrap().unwrap(), &_id!("36"));
    assert!(tree.lookup(&mut pager, 25).is_none());
}

#[test]
fn t_indirect_8() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(8, &mut tree, &mut pager);

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);
    assert_t_indirect!(tree, pager);
    assert!(tree.lookup(&mut pager, 26).is_none());
}

#[test]
fn full() {
    let mut pager = Pager::new(setup_container_with_bsize(BSIZE));
    let mut tree = make_tree();

    aquire_direct(12, &mut tree, &mut pager);
    aquire_indirect(2, &mut tree, &mut pager);
    aquire_d_indirect(4, &mut tree, &mut pager);
    aquire_t_indirect(8, &mut tree, &mut pager);

    assert!(matches!(tree.aquire(&mut pager).unwrap_err(), Error::Full));

    assert_direct!(tree, pager);
    assert_indirect!(tree, pager);
    assert_d_indirect!(tree, pager);
    assert_t_indirect!(tree, pager);
    assert!(tree.lookup(&mut pager, 26).is_none());
}

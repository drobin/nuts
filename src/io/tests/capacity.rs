// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use std::io::{Cursor, Seek, SeekFrom};

use crate::io::IO;
use crate::rand::RND;
use crate::types::DiskType;

fn mk_fake_file(vec: Vec<u8>) -> Cursor<Vec<u8>> {
    let mut c = Cursor::new(vec);
    c.seek(SeekFrom::Start(0)).unwrap();
    c
}

fn setup(vec: Vec<u8>, dtype: DiskType) -> (Cursor<Vec<u8>>, IO) {
    let mut f = mk_fake_file(vec);
    let io = IO::new(4, 3, dtype, &mut f).unwrap();

    (f, io)
}

#[test]
fn thin_zero_empty_0_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), []);
}

#[test]
fn thin_zero_empty_1_block() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [0; 4]);
}

#[test]
fn thin_zero_empty_2_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 2);
    assert_eq!(f.into_inner(), [0; 8]);
}

#[test]
fn thin_zero_empty_3_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn thin_zero_empty_overflow() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn thin_zero_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), [9, 9]);
}

#[test]
fn thin_zero_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [0; 4]);
}

#[test]
fn thin_zero_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 2);
    assert_eq!(f.into_inner(), [0; 8]);
}

#[test]
fn thin_zero_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn thin_zero_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn thin_zero_one_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [9; 6]);
}

#[test]
fn thin_zero_one_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [9; 6]);
}

#[test]
fn thin_zero_one_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 2);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0]);
}

#[test]
fn thin_zero_one_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn thin_zero_one_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinZero);

    io.ensure_capacity(&mut f, 4).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn thin_random_empty_0_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), []);
}

#[test]
fn thin_random_empty_1_block() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), &RND[0..4]);
}

#[test]
fn thin_random_empty_2_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 2);
    assert_eq!(
        f.into_inner(),
        [RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn thin_random_empty_3_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn thin_random_empty_overflow() {
    let (mut f, mut io) = setup(vec![], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn thin_random_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), [9, 9]);
}

#[test]
fn thin_random_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), &RND[0..4]);
}

#[test]
fn thin_random_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 2);
    assert_eq!(
        f.into_inner(),
        [RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn thin_random_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn thin_random_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn thin_random_one_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [9; 6]);
}

#[test]
fn thin_random_one_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [9; 6]);
}

#[test]
fn thin_random_one_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 2);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3]]);
}

#[test]
fn thin_random_one_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 4).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn thin_random_one_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::ThinRandom);

    io.ensure_capacity(&mut f, 4).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn fat_zero_empty_0_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::FatZero);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), []);
}

#[test]
fn fat_zero_empty_1_block() {
    let (mut f, mut io) = setup(vec![], DiskType::FatZero);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_empty_2_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::FatZero);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_empty_3_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_empty_overflow() {
    let (mut f, mut io) = setup(vec![], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatZero);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), [9; 2]);
}

#[test]
fn fat_zero_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatZero);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatZero);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [0; 12]);
}

#[test]
fn fat_zero_one_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatZero);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [9; 6]);
}

#[test]
fn fat_zero_one_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatZero);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn fat_zero_one_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatZero);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn fat_zero_one_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn fat_zero_one_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9, 9, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn fat_zero_too_big() {
    let (mut f, mut io) = setup(vec![9; 14], DiskType::FatZero);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9; 14]);
}

#[test]
fn fat_random_empty_0_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), []);
}

#[test]
fn fat_random_empty_1_block() {
    let (mut f, mut io) = setup(vec![], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_empty_2_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_empty_3_blocks() {
    let (mut f, mut io) = setup(vec![], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_empty_overflow() {
    let (mut f, mut io) = setup(vec![], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 0);
    assert_eq!(f.into_inner(), [9; 2]);
}

#[test]
fn fat_random_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 2], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [
            RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2],
            RND[3]
        ]
    );
}

#[test]
fn fat_random_one_half_0_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 0).unwrap();
    assert_eq!(io.ablocks, 1);
    assert_eq!(f.into_inner(), [9; 6]);
}

#[test]
fn fat_random_one_half_1_block() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 1).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn fat_random_one_half_2_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 2).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn fat_random_one_half_3_blocks() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn fat_random_one_half_overflow() {
    let (mut f, mut io) = setup(vec![9; 6], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(
        f.into_inner(),
        [9, 9, 9, 9, RND[0], RND[1], RND[2], RND[3], RND[0], RND[1], RND[2], RND[3]]
    );
}

#[test]
fn fat_random_too_big() {
    let (mut f, mut io) = setup(vec![9; 14], DiskType::FatRandom);

    io.ensure_capacity(&mut f, 3).unwrap();
    assert_eq!(io.ablocks, 3);
    assert_eq!(f.into_inner(), [9; 14]);
}

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

use std::io::{Cursor, ErrorKind};

use crate::error::Error;
use crate::io::IO;
use crate::openssl::RND;
use crate::types::DiskType;

fn prepare(
    dtype: DiskType,
    bsize: u32,
    blocks: u64,
    data: Vec<u8>,
) -> (IO, [u8; 6], Cursor<Vec<u8>>) {
    let mut target = Cursor::new(data);
    let io = IO::new(bsize, blocks, dtype, &mut target).unwrap();
    let source = [1, 2, 3, 4, 5, 6];

    (io, source, target)
}

#[test]
fn thin_zero_allocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn thin_zero_allocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 0, 9, 9, 9]);
}

#[test]
fn thin_zero_allocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn thin_zero_allocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn thin_zero_allocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 0]);
}

#[test]
fn thin_zero_allocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn thin_zero_allocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_zero_unallocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3]);
}

#[test]
fn thin_zero_unallocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 0]);
}

#[test]
fn thin_zero_unallocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3]);
}

#[test]
fn thin_zero_unallocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 3]);
}

#[test]
fn thin_zero_unallocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 0]);
}

#[test]
fn thin_zero_unallocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 3]);
}

#[test]
fn thin_zero_unallocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::ThinZero, 3, 2, vec![]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_allocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn fat_zero_allocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 0, 9, 9, 9]);
}

#[test]
fn fat_zero_allocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn fat_zero_allocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn fat_zero_allocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 0]);
}

#[test]
fn fat_zero_allocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn fat_zero_allocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_unallocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 0, 0, 0]);
}

#[test]
fn fat_zero_unallocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 0, 0, 0, 0]);
}

#[test]
fn fat_zero_unallocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 0, 0, 0]);
}

#[test]
fn fat_zero_unallocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 3]);
}

#[test]
fn fat_zero_unallocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 0]);
}

#[test]
fn fat_zero_unallocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [0, 0, 0, 1, 2, 3]);
}

#[test]
fn fat_zero_unallocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::FatZero, 3, 2, vec![]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_allocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn thin_random_allocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, RND[0], 9, 9, 9]);
}

#[test]
fn thin_random_allocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn thin_random_allocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn thin_random_allocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, RND[0]]);
}

#[test]
fn thin_random_allocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn thin_random_allocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_unallocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3]);
}

#[test]
fn thin_random_unallocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, RND[0]]);
}

#[test]
fn thin_random_unallocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3]);
}

#[test]
fn thin_random_unallocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [RND[0], RND[1], RND[2], 1, 2, 3]);
}

#[test]
fn thin_random_unallocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [RND[0], RND[1], RND[2], 1, 2, RND[0]]);
}

#[test]
fn thin_random_unallocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [RND[0], RND[1], RND[2], 1, 2, 3]);
}

#[test]
fn thin_random_unallocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::ThinRandom, 3, 2, vec![]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_allocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn fat_random_allocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, RND[0], 9, 9, 9]);
}

#[test]
fn fat_random_allocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, 9, 9, 9]);
}

#[test]
fn fat_random_allocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn fat_random_allocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, RND[0]]);
}

#[test]
fn fat_random_allocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [9, 9, 9, 1, 2, 3]);
}

#[test]
fn fat_random_allocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![9; 6]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_unallocated_0_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, RND[0], RND[1], RND[2]]);
}

#[test]
fn fat_random_unallocated_0_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, RND[0], RND[0], RND[1], RND[2]]);
}

#[test]
fn fat_random_unallocated_0_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 0).unwrap(), 3);
    assert_eq!(target.into_inner(), [1, 2, 3, RND[0], RND[1], RND[2]]);
}

#[test]
fn fat_random_unallocated_1_full() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2, 3];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [RND[0], RND[1], RND[2], 1, 2, 3]);
}

#[test]
fn fat_random_unallocated_1_part() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [RND[0], RND[1], RND[2], 1, 2, RND[0]]);
}

#[test]
fn fat_random_unallocated_1_more() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2, 3, 4];

    assert_eq!(io.write(&source, &mut target, 1).unwrap(), 3);
    assert_eq!(target.into_inner(), [RND[0], RND[1], RND[2], 1, 2, 3]);
}

#[test]
fn fat_random_unallocated_overflow() {
    let (mut io, _, mut target) = prepare(DiskType::FatRandom, 3, 2, vec![]);
    let source = [1, 2, 3];

    if let Error::IoError(err) = io.write(&source, &mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
    } else {
        panic!("invalid error");
    }
}

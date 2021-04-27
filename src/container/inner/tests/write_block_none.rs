// MIT License
//
// Copyright (c) 2020, 2021 Robin Doer
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

use std::io::{Cursor, ErrorKind, Seek, SeekFrom, Write};

use crate::container::inner::tests::PLAINTEXT;
use crate::container::inner::Inner;
use crate::error::Error;
use crate::password::PasswordStore;
use crate::rand::RND;
use crate::types::{Cipher, DiskType, OptionsBuilder};

fn setup(dtype: DiskType, bsize: u32, blocks: u64, ablocks: u64) -> Inner<Cursor<Vec<u8>>> {
    let mut store = PasswordStore::new();

    let data = {
        let options = OptionsBuilder::new(Cipher::None)
            .with_dtype(dtype)
            .with_bsize(bsize)
            .with_blocks(blocks)
            .build()
            .unwrap();

        let cursor = Cursor::new(vec![]);
        let mut inner = Inner::create(cursor, options, &mut store).unwrap();
        let nbytes = (bsize as u64 * (ablocks - 1)) as usize;

        inner.fh.seek(SeekFrom::Start(bsize as u64)).unwrap();
        inner.fh.write_all(&PLAINTEXT[..nbytes]).unwrap();
        inner.fh.flush().unwrap();

        inner.as_ref().get_ref().to_vec()
    };

    Inner::open(Cursor::new(data), &mut store).unwrap()
}

macro_rules! assert_header {
    ($buf:expr, $rnd:expr) => {
        let mut store = PasswordStore::new();
        let (_, nbytes) = crate::header::Header::read($buf, &mut store).unwrap();
        let nbytes = nbytes as usize;

        if ($rnd) {
            assert_eq!($buf[nbytes..512], RND[..512 - nbytes]);
        } else {
            assert_eq!(&$buf[nbytes..512], &vec![0u8; 512 - nbytes][..]);
        };
    };
}

macro_rules! assert_block {
    ($block:expr, $buf:expr, $exp:expr, $rnd:expr) => {
        let mut block2 = vec![0; 512];

        block2[..$exp.len()].copy_from_slice($exp);

        if $rnd {
            block2[$exp.len()..].copy_from_slice(&RND[..512 - $exp.len()]);
        }

        let offs = $block * 512 as usize;
        assert_eq!(&$buf[offs..offs + 512], &block2[..]);
    };
}

#[test]
fn thin_zero_header() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_zero_allocated_full() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512][..], false);
}

#[test]
fn thin_zero_allocated_part() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &[9, 9, 9], false);
}

#[test]
fn thin_zero_allocated_more() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512][..], false);
}

#[test]
fn thin_zero_unallocated_full() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 2).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &PLAINTEXT[..512], false);
    assert_block!(2, &buf, &vec![9; 512][..], false);
}

#[test]
fn thin_zero_unallocated_part() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 2).unwrap(), 3);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &PLAINTEXT[..512], false);
    assert_block!(2, &buf, &[9, 9, 9], false);
}

#[test]
fn thin_zero_unallocated_more() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 2).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &PLAINTEXT[..512], false);
    assert_block!(2, &buf, &vec![9; 512], false);
}

#[test]
fn thin_zero_no_such_block() {
    let mut inner = setup(DiskType::ThinZero, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_header() {
    let mut inner = setup(DiskType::FatZero, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_allocated_full() {
    let mut inner = setup(DiskType::FatZero, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512], false);
}

#[test]
fn fat_zero_allocated_part() {
    let mut inner = setup(DiskType::FatZero, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &[9, 9, 9], false);
}

#[test]
fn fat_zero_allocated_more() {
    let mut inner = setup(DiskType::FatZero, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512], false);
}

#[test]
fn fat_zero_no_such_block() {
    let mut inner = setup(DiskType::FatZero, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 2");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_header() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_allocated_full() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_allocated_part() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &[9, 9, 9], true);
}

#[test]
fn thin_random_allocated_more() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_unallocated_full() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 2).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &PLAINTEXT[..512], true);
    assert_block!(2, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_unallocated_part() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 2).unwrap(), 3);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &PLAINTEXT[..512], true);
    assert_block!(2, &buf, &[9, 9, 9], true);
}

#[test]
fn thin_random_unallocated_more() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 2).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &PLAINTEXT[..512], true);
    assert_block!(2, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_no_such_block() {
    let mut inner = setup(DiskType::ThinRandom, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_header() {
    let mut inner = setup(DiskType::FatRandom, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_allocated_full() {
    let mut inner = setup(DiskType::FatRandom, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn fat_random_allocated_part() {
    let mut inner = setup(DiskType::FatRandom, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &[9, 9, 9], true);
}

#[test]
fn fat_random_allocated_more() {
    let mut inner = setup(DiskType::FatRandom, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = inner.as_ref().get_ref();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn fat_random_no_such_block() {
    let mut inner = setup(DiskType::FatRandom, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 2");
    } else {
        panic!("invalid error");
    }
}

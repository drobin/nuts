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

use std::io::{ErrorKind, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::container::inner::tests::PLAINTEXT;
use crate::container::inner::Inner;
use crate::error::Error;
use crate::header::Header;
use crate::password::PasswordStore;
use crate::rand::RND;
use crate::types::{Cipher, DiskType, OptionsBuilder};

fn setup_container(dtype: DiskType, bsize: u32, blocks: u64, ablocks: u64) -> Inner {
    let tmp_dir = TempDir::new().unwrap();
    let path: PathBuf = [tmp_dir.path(), Path::new("container")].iter().collect();
    let mut store = PasswordStore::new();

    {
        let options = OptionsBuilder::new(Cipher::None)
            .with_dtype(dtype)
            .with_bsize(bsize)
            .with_blocks(blocks)
            .build()
            .unwrap();

        let mut inner = Inner::create(&path, options, &mut store).unwrap();
        let nbytes = (bsize as u64 * (ablocks - 1)) as usize;

        inner.fh.seek(SeekFrom::Start(bsize as u64)).unwrap();
        inner.fh.write_all(&PLAINTEXT[..nbytes]).unwrap();
        inner.fh.flush().unwrap();
    };

    Inner::open(&path, &mut store).unwrap()
}

#[test]
fn thin_zero_header_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(&target[nbytes..], &vec![0u8; 512 - nbytes][..]);
}

#[test]
fn thin_zero_header_part() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 3);
    assert_eq!(target, [b'n', b'u', b't']);
}

#[test]
fn thin_zero_header_bigger() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 513];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(&target[nbytes..512], &vec![0u8; 512 - nbytes][..]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_zero_allocated_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &PLAINTEXT[..512]);
}

#[test]
fn thin_zero_allocated_part() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &PLAINTEXT[..3]);
}

#[test]
fn thin_zero_allocated_bigger() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], PLAINTEXT[..512]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_zero_unallocated_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 512);
    assert_eq!(target, &vec![0u8; 512][..]);
}

#[test]
fn thin_zero_unallocated_part() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 3);
    assert_eq!(target, [0, 0, 0]);
}

#[test]
fn thin_zero_unallocated_bigger() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 512);
    assert_eq!(target[..512], vec![0u8; 512][..]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_zero_no_such_block() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    if let Error::IoError(err) = inner.read_block(&mut target, 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_header_full() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 512];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(&target[nbytes..], &vec![0u8; 512 - nbytes][..]);
}

#[test]
fn fat_zero_header_part() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 3);
    assert_eq!(target, [b'n', b'u', b't']);
}

#[test]
fn fat_zero_header_bigger() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 513];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(&target[nbytes..512], &vec![0u8; 512 - nbytes][..]);
    assert_eq!(target[512], b'x');
}

#[test]
fn fat_zero_allocated_full() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &PLAINTEXT[..512]);
}

#[test]
fn fat_zero_allocated_part() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &PLAINTEXT[..3]);
}

#[test]
fn fat_zero_allocated_bigger() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], PLAINTEXT[..512]);
    assert_eq!(target[512], b'x');
}

#[test]
fn fat_zero_no_such_block() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 512];

    if let Error::IoError(err) = inner.read_block(&mut target, 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 2");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_header_full() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 512];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(target[nbytes..], RND[..512 - nbytes]);
}

#[test]
fn thin_random_header_part() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 3);
    assert_eq!(target, [b'n', b'u', b't']);
}

#[test]
fn thin_random_header_bigger() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 513];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(target[nbytes..512], RND[..512 - nbytes]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_random_allocated_full() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &PLAINTEXT[..512]);
}

#[test]
fn thin_random_allocated_part() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &PLAINTEXT[..3]);
}

#[test]
fn thin_random_allocated_bigger() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], PLAINTEXT[..512]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_random_unallocated_full() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 512);
    assert_eq!(target, &RND[..512]);
}

#[test]
fn thin_random_unallocated_part() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 3);
    assert_eq!(target, &RND[..3]);
}

#[test]
fn thin_random_unallocated_bigger() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 512);
    assert_eq!(target[..512], RND[..512]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_random_no_such_block() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    if let Error::IoError(err) = inner.read_block(&mut target, 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_header_full() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 512];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(target[nbytes..], RND[..512 - nbytes]);
}

#[test]
fn fat_random_header_part() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 3);
    assert_eq!(target, [b'n', b'u', b't']);
}

#[test]
fn fat_random_header_bigger() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 513];
    let mut store = PasswordStore::new();

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, &mut store).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(target[nbytes..512], RND[..512 - nbytes]);
    assert_eq!(target[512], b'x');
}

#[test]
fn fat_random_allocated_full() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &PLAINTEXT[..512]);
}

#[test]
fn fat_random_allocated_part() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &PLAINTEXT[..3]);
}

#[test]
fn fat_random_allocated_bigger() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], PLAINTEXT[..512]);
    assert_eq!(target[512], b'x');
}

#[test]
fn fat_random_no_such_block() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 512];

    if let Error::IoError(err) = inner.read_block(&mut target, 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

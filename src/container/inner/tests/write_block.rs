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

use std::fs::read;
use std::io::{ErrorKind, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::container::inner::Inner;
use crate::error::Error;
use crate::password::PasswordStore;
use crate::rand::RND;
use crate::types::{Cipher, DiskType, OptionsBuilder};

const SOURCE: [u8; 1024] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42,
    43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
    67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
    91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
    131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
    150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168,
    169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187,
    188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206,
    207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225,
    226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244,
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
    36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
    60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
    84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105,
    106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
    125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
    144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162,
    163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181,
    182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
    201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
    220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
    239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 0, 1, 2,
    3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118,
    119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137,
    138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
    157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
    176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194,
    195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213,
    214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232,
    233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251,
    252, 253, 254, 255,
];

fn setup(dtype: DiskType, bsize: u32, blocks: u64, ablocks: u64) -> (TempDir, PathBuf, Inner) {
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

        let mut inner = Inner::create(&path, &options, &mut store).unwrap();
        let nbytes = (bsize as u64 * (ablocks - 1)) as usize;
        let mut buf = vec![0u8; nbytes];

        for (pos, elem) in buf.iter_mut().enumerate() {
            *elem = SOURCE[pos];
        }

        inner.fh.seek(SeekFrom::Start(bsize as u64)).unwrap();
        inner.fh.write_all(&buf).unwrap();
        inner.fh.flush().unwrap();
    };

    let inner = Inner::open(&path, &mut store).unwrap();

    (tmp_dir, path, inner)
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
    let (_tmp_dir, _path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_zero_allocated_full() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512][..], false);
}

#[test]
fn thin_zero_allocated_part() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &[9, 9, 9], false);
}

#[test]
fn thin_zero_allocated_more() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512][..], false);
}

#[test]
fn thin_zero_unallocated_full() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 2).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &SOURCE[..512], false);
    assert_block!(2, &buf, &vec![9; 512][..], false);
}

#[test]
fn thin_zero_unallocated_part() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 2).unwrap(), 3);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &SOURCE[..512], false);
    assert_block!(2, &buf, &[9, 9, 9], false);
}

#[test]
fn thin_zero_unallocated_more() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 2).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &SOURCE[..512], false);
    assert_block!(2, &buf, &vec![9; 512], false);
}

#[test]
fn thin_zero_no_such_block() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::ThinZero, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_header() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::FatZero, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_zero_allocated_full() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::FatZero, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512], false);
}

#[test]
fn fat_zero_allocated_part() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::FatZero, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &[9, 9, 9], false);
}

#[test]
fn fat_zero_allocated_more() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::FatZero, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, false);
    assert_block!(1, &buf, &vec![9; 512], false);
}

#[test]
fn fat_zero_no_such_block() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::FatZero, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 2");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_header() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn thin_random_allocated_full() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_allocated_part() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &[9, 9, 9], true);
}

#[test]
fn thin_random_allocated_more() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_unallocated_full() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 2).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &SOURCE[..512], true);
    assert_block!(2, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_unallocated_part() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 2).unwrap(), 3);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &SOURCE[..512], true);
    assert_block!(2, &buf, &[9, 9, 9], true);
}

#[test]
fn thin_random_unallocated_more() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 2).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1536);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &SOURCE[..512], true);
    assert_block!(2, &buf, &vec![9; 512][..], true);
}

#[test]
fn thin_random_no_such_block() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::ThinRandom, 512, 3, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 3).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 3");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_header() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::FatRandom, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 0).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "cannot overwrite header");
    } else {
        panic!("invalid error");
    }
}

#[test]
fn fat_random_allocated_full() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::FatRandom, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 512], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn fat_random_allocated_part() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::FatRandom, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 3], 1).unwrap(), 3);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &[9, 9, 9], true);
}

#[test]
fn fat_random_allocated_more() {
    let (_tmp_dir, path, mut inner) = setup(DiskType::FatRandom, 512, 2, 2);
    assert_eq!(inner.write_block(&vec![9; 513], 1).unwrap(), 512);

    let buf = read(path).unwrap();
    assert_eq!(buf.len(), 1024);
    assert_header!(&buf, true);
    assert_block!(1, &buf, &vec![9; 512][..], true);
}

#[test]
fn fat_random_no_such_block() {
    let (_tmp_dir, _path, mut inner) = setup(DiskType::FatRandom, 512, 2, 2);

    if let Error::IoError(err) = inner.write_block(&vec![9; 512], 2).unwrap_err() {
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(format!("{}", err), "unable to locate block 2");
    } else {
        panic!("invalid error");
    }
}

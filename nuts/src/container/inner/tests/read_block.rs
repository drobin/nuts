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

use std::io::{ErrorKind, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;

use crate::container::inner::Inner;
use crate::error::Error;
use crate::header::Header;
use crate::rand::RND;
use crate::result::Result;
use crate::types::{Cipher, DiskType, Options};

const NONE: Option<&fn() -> Result<Vec<u8>>> = None;

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

fn setup_container(dtype: DiskType, bsize: u32, blocks: u64, ablocks: u64) -> Inner {
    let tmp_dir = TempDir::new().unwrap();
    let path: PathBuf = [tmp_dir.path(), Path::new("container")].iter().collect();

    {
        let mut options = Options::default_with_cipher(Cipher::None).unwrap();

        options.set_dtype(dtype);
        options.set_bsize(bsize).unwrap();
        options.set_blocks(blocks).unwrap();

        let mut inner = Inner::create(&path, &options, NONE).unwrap();
        let nbytes = (bsize as u64 * (ablocks - 1)) as usize;
        let mut buf = vec![0u8; nbytes];

        for (pos, elem) in buf.iter_mut().enumerate() {
            *elem = SOURCE[pos];
        }

        inner.fh.seek(SeekFrom::Start(bsize as u64)).unwrap();
        inner.fh.write_all(&buf).unwrap();
        inner.fh.flush().unwrap();
    };

    Inner::open(&path, NONE).unwrap()
}

#[test]
fn thin_zero_header_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(&target[nbytes..512], &vec![0u8; 512 - nbytes][..]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_zero_allocated_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &SOURCE[..512]);
}

#[test]
fn thin_zero_allocated_part() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &SOURCE[..3]);
}

#[test]
fn thin_zero_allocated_bigger() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], SOURCE[..512]);
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(&target[nbytes..512], &vec![0u8; 512 - nbytes][..]);
    assert_eq!(target[512], b'x');
}

#[test]
fn fat_zero_allocated_full() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &SOURCE[..512]);
}

#[test]
fn fat_zero_allocated_part() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &SOURCE[..3]);
}

#[test]
fn fat_zero_allocated_bigger() {
    let mut inner = setup_container(DiskType::FatZero, 512, 2, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], SOURCE[..512]);
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(target[nbytes..512], RND[..512 - nbytes]);
    assert_eq!(target[512], b'x');
}

#[test]
fn thin_random_allocated_full() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &SOURCE[..512]);
}

#[test]
fn thin_random_allocated_part() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &SOURCE[..3]);
}

#[test]
fn thin_random_allocated_bigger() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], SOURCE[..512]);
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
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

    assert_eq!(inner.read_block(&mut target, 0).unwrap(), 512);
    let (_, nbytes) = Header::read(&target, NONE).unwrap();
    let nbytes = nbytes as usize;
    assert_eq!(target[nbytes..512], RND[..512 - nbytes]);
    assert_eq!(target[512], b'x');
}

#[test]
fn fat_random_allocated_full() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 512];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target, &SOURCE[..512]);
}

#[test]
fn fat_random_allocated_part() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 3];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 3);
    assert_eq!(target, &SOURCE[..3]);
}

#[test]
fn fat_random_allocated_bigger() {
    let mut inner = setup_container(DiskType::FatRandom, 512, 2, 2);
    let mut target = vec![b'x'; 513];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 512);
    assert_eq!(target[..512], SOURCE[..512]);
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

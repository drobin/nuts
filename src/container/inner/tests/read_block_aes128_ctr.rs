// MIT License
//
// Copyright (c) 2021 Robin Doer
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

use crate::container::inner::tests::{setup_store, PLAINTEXT};
use crate::container::inner::Inner;
use crate::error::Error;
use crate::header::Header;
use crate::rand::RND;
use crate::types::{Cipher, DiskType, OptionsBuilder};

const CIPHERTEXT: [u8; 1024] = [
    249, 66, 241, 84, 199, 162, 65, 162, 114, 230, 225, 112, 41, 84, 190, 52, 239, 127, 73, 233,
    187, 154, 17, 122, 147, 206, 58, 87, 56, 220, 135, 116, 230, 145, 26, 216, 113, 197, 233, 187,
    128, 16, 1, 231, 196, 198, 37, 254, 136, 36, 73, 51, 52, 16, 148, 54, 118, 57, 109, 97, 232,
    130, 103, 68, 249, 53, 188, 85, 106, 78, 210, 202, 110, 143, 34, 159, 202, 35, 33, 208, 25, 55,
    149, 97, 109, 219, 44, 124, 239, 84, 83, 211, 49, 76, 142, 88, 148, 96, 80, 106, 171, 82, 171,
    10, 181, 111, 138, 41, 195, 245, 45, 105, 243, 177, 28, 8, 94, 205, 51, 160, 26, 251, 116, 253,
    191, 142, 151, 210, 132, 93, 143, 74, 99, 94, 243, 102, 182, 88, 203, 130, 226, 231, 47, 49,
    227, 2, 155, 188, 31, 58, 168, 183, 197, 37, 192, 152, 223, 77, 118, 58, 239, 246, 242, 111,
    75, 15, 214, 124, 242, 159, 189, 164, 126, 198, 248, 108, 25, 219, 127, 100, 212, 100, 139,
    165, 189, 189, 69, 232, 166, 57, 181, 111, 131, 84, 38, 111, 30, 181, 227, 14, 101, 31, 193,
    227, 138, 194, 244, 157, 98, 182, 225, 95, 117, 227, 148, 197, 254, 43, 147, 169, 201, 152, 26,
    35, 177, 194, 226, 237, 227, 78, 157, 54, 107, 131, 29, 194, 220, 224, 167, 81, 54, 73, 119,
    29, 38, 89, 246, 58, 224, 25, 212, 107, 42, 108, 108, 82, 78, 62, 177, 32, 129, 118, 178, 99,
    211, 9, 117, 149, 43, 204, 224, 2, 43, 74, 231, 17, 116, 199, 29, 125, 70, 73, 56, 141, 91, 1,
    181, 203, 234, 104, 163, 157, 178, 84, 205, 17, 72, 26, 149, 80, 26, 234, 224, 22, 189, 134,
    18, 16, 116, 202, 57, 103, 172, 148, 45, 57, 181, 38, 45, 217, 86, 64, 17, 78, 76, 221, 4, 246,
    213, 253, 25, 254, 204, 107, 166, 134, 211, 240, 208, 207, 158, 215, 90, 24, 226, 57, 219, 215,
    23, 142, 96, 56, 173, 41, 230, 2, 8, 11, 209, 233, 16, 32, 225, 198, 1, 225, 42, 141, 189, 53,
    212, 190, 157, 195, 132, 72, 15, 231, 89, 19, 183, 161, 245, 237, 25, 226, 166, 199, 133, 86,
    7, 142, 76, 179, 238, 99, 51, 177, 14, 94, 94, 185, 37, 165, 230, 160, 38, 29, 6, 11, 62, 190,
    69, 104, 76, 247, 29, 16, 30, 48, 165, 39, 77, 135, 133, 156, 239, 130, 151, 28, 70, 157, 193,
    128, 148, 49, 46, 108, 23, 61, 205, 119, 100, 84, 130, 237, 230, 125, 63, 40, 253, 120, 229,
    125, 144, 204, 44, 173, 65, 153, 140, 199, 131, 177, 193, 228, 50, 229, 77, 132, 213, 223, 71,
    175, 184, 87, 152, 39, 196, 198, 51, 190, 200, 77, 251, 228, 63, 165, 48, 149, 32, 158, 240,
    219, 209, 10, 159, 14, 223, 252, 179, 77, 82, 118, 14, 188, 129, 28, 76, 167, 230, 193, 235,
    112, 208, 153, 15, 125, 226, 147, 76, 124, 159, 148, 144, 37, 9, 173, 254, 251, 26, 226, 76,
    158, 251, 97, 145, 162, 18, 91, 64, 117, 228, 114, 3, 16, 197, 223, 165, 8, 210, 121, 245, 165,
    27, 233, 142, 207, 168, 242, 38, 201, 224, 150, 1, 88, 183, 2, 222, 36, 106, 25, 132, 154, 222,
    109, 137, 173, 66, 228, 156, 121, 77, 151, 12, 103, 13, 29, 224, 241, 148, 14, 187, 145, 169,
    8, 86, 115, 99, 171, 178, 156, 121, 120, 86, 60, 99, 21, 46, 187, 114, 43, 33, 185, 68, 45, 14,
    3, 70, 188, 127, 217, 207, 217, 128, 211, 91, 79, 60, 96, 198, 247, 189, 20, 151, 105, 34, 115,
    16, 89, 101, 25, 125, 183, 28, 56, 218, 62, 145, 131, 211, 212, 138, 94, 178, 95, 54, 152, 178,
    2, 138, 247, 246, 7, 79, 62, 236, 77, 25, 118, 63, 137, 210, 5, 114, 132, 214, 7, 196, 9, 79,
    27, 255, 161, 72, 92, 141, 121, 137, 66, 200, 41, 4, 225, 9, 29, 248, 195, 223, 189, 139, 175,
    110, 50, 253, 224, 199, 142, 81, 118, 23, 89, 230, 240, 76, 182, 95, 122, 26, 255, 161, 43,
    112, 41, 224, 20, 52, 92, 166, 17, 214, 53, 67, 229, 219, 205, 174, 99, 231, 212, 159, 168, 9,
    118, 14, 190, 133, 105, 159, 127, 85, 204, 197, 212, 3, 180, 94, 148, 176, 47, 229, 156, 168,
    0, 18, 28, 175, 125, 124, 187, 211, 111, 247, 16, 62, 187, 80, 143, 51, 54, 225, 247, 110, 228,
    117, 222, 167, 243, 36, 108, 198, 170, 121, 187, 130, 125, 240, 23, 22, 252, 236, 202, 39, 190,
    89, 40, 249, 54, 7, 100, 236, 58, 167, 81, 122, 247, 100, 137, 205, 135, 162, 135, 46, 210, 99,
    120, 64, 170, 253, 55, 167, 62, 13, 32, 211, 18, 44, 94, 0, 66, 46, 14, 92, 154, 57, 250, 245,
    133, 38, 239, 124, 115, 101, 145, 146, 82, 129, 106, 181, 24, 214, 100, 56, 20, 177, 111, 161,
    163, 49, 254, 77, 210, 253, 34, 181, 167, 144, 240, 227, 118, 200, 199, 105, 251, 230, 245,
    181, 67, 213, 75, 29, 159, 88, 82, 216, 192, 22, 107, 219, 78, 87, 77, 188, 32, 222, 15, 55,
    83, 172, 177, 219, 93, 53, 75, 72, 93, 239, 65, 155, 19, 201, 182, 67, 34, 89, 150, 14, 46,
    177, 32, 172, 66, 61, 143, 3, 13, 229, 31, 121, 22, 176, 103, 149, 241, 15, 78, 193, 189, 168,
    193, 60, 47, 229, 213, 115, 29, 179, 225, 240, 72, 50, 64, 144, 229, 131, 19, 241, 47, 236,
    206, 223, 106, 186, 143, 165, 88, 2, 21, 119, 235, 219, 245, 141, 181, 107, 199, 77, 180, 239,
    132, 42, 227, 229, 3, 107, 33, 245, 226, 59, 163, 158, 35, 214, 58, 76, 19, 9, 98, 58, 99, 196,
    5, 242, 172, 0, 228, 171, 200, 16, 31, 139, 13, 18, 231,
];

fn setup_container(dtype: DiskType, bsize: u32, blocks: u64, ablocks: u64) -> Inner {
    let tmp_dir = TempDir::new().unwrap();
    let path: PathBuf = [tmp_dir.path(), Path::new("container")].iter().collect();
    let mut store = setup_store();

    {
        let options = OptionsBuilder::new(Cipher::Aes128Ctr)
            .with_dtype(dtype)
            .with_bsize(bsize)
            .with_blocks(blocks)
            .build()
            .unwrap();

        let mut inner = Inner::create(&path, options, &mut store).unwrap();

        inner.header.master_key = secure_vec![b'a'; 16];
        inner.header.master_iv = secure_vec![b'b'; 16];

        inner.flush_header(&mut store).unwrap();

        let nbytes = (bsize as u64 * (ablocks - 1)) as usize;

        inner.fh.seek(SeekFrom::Start(bsize as u64)).unwrap();
        inner.fh.write_all(&CIPHERTEXT[..nbytes]).unwrap();
        inner.fh.flush().unwrap();
    };

    Inner::open(&path, &mut store).unwrap()
}

// fn setup_key_iv(block_id: u64) -> (Vec<u8>, Vec<u8>) {
//     let mut master_key = vec![b'a'; 16];
//     let mut master_iv = vec![b'b'; 16];
//     let block_id = block_id.to_be_bytes();

//     for (idx, by) in master_key.iter_mut().enumerate() {
//         *by = *by ^ block_id[idx % block_id.len()];
//     }

//     for (idx, by) in master_iv.iter_mut().enumerate() {
//         *by = *by ^ block_id[idx % block_id.len()];
//     }

//     (master_key, master_iv)
// }

// #[test]
// fn make_ciphertext() {
//     let (key, iv) = setup_key_iv(1);

//     let ciphertext = Cipher::Aes128Ctr
//         .encrypt(&PLAINTEXT[..512], &key, &iv)
//         .unwrap();
//     println!("{:?}", ciphertext);

//     let (key, iv) = setup_key_iv(2);
//     let ciphertext = Cipher::Aes128Ctr
//         .encrypt(&PLAINTEXT[512..], &key, &iv)
//         .unwrap();
//     println!("{:?}", ciphertext);
// }

#[test]
fn thin_zero_header_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 512];
    let mut store = setup_store();

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
    let mut store = setup_store();

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
    let mut store = setup_store();

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
    let mut store = setup_store();

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
    let mut store = setup_store();

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
    let mut store = setup_store();

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
    let mut store = setup_store();

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
    let mut store = setup_store();

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

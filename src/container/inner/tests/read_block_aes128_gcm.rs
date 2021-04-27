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

use std::io::{Cursor, ErrorKind, Seek, SeekFrom, Write};

use crate::container::inner::tests::{setup_store, PLAINTEXT};
use crate::container::inner::Inner;
use crate::error::Error;
use crate::header::Header;
use crate::rand::RND;
use crate::types::{Cipher, DiskType, OptionsBuilder};

const CIPHERTEXT: [u8; 1024] = [
    103, 183, 43, 78, 24, 66, 53, 64, 231, 182, 170, 61, 157, 119, 81, 130, 185, 170, 35, 29, 121,
    221, 161, 244, 216, 145, 120, 13, 24, 110, 25, 104, 139, 175, 11, 241, 82, 134, 136, 31, 153,
    76, 180, 115, 116, 188, 239, 105, 108, 21, 242, 34, 23, 106, 115, 135, 253, 179, 44, 243, 119,
    70, 105, 236, 192, 193, 106, 142, 11, 192, 104, 12, 143, 153, 34, 34, 252, 178, 54, 132, 208,
    32, 57, 130, 16, 248, 205, 160, 108, 55, 179, 153, 213, 32, 254, 210, 1, 201, 103, 132, 193,
    131, 137, 36, 45, 114, 17, 235, 92, 210, 150, 111, 241, 9, 56, 34, 235, 105, 63, 216, 84, 194,
    85, 52, 155, 122, 167, 16, 186, 137, 214, 225, 200, 162, 59, 3, 71, 110, 87, 47, 128, 191, 217,
    52, 70, 133, 111, 3, 4, 223, 40, 45, 247, 148, 254, 46, 52, 98, 105, 22, 196, 185, 111, 220,
    94, 250, 85, 192, 132, 225, 141, 56, 32, 56, 195, 183, 195, 115, 27, 174, 198, 182, 209, 216,
    30, 186, 206, 23, 184, 46, 231, 5, 11, 130, 75, 187, 164, 139, 111, 138, 86, 210, 248, 107,
    139, 68, 69, 95, 10, 7, 224, 222, 14, 255, 54, 151, 181, 154, 56, 199, 113, 132, 78, 87, 11,
    178, 122, 90, 151, 241, 212, 167, 39, 139, 93, 175, 79, 200, 201, 101, 64, 78, 155, 109, 136,
    149, 163, 194, 39, 116, 223, 99, 21, 2, 53, 167, 67, 42, 131, 94, 27, 221, 212, 223, 34, 177,
    41, 141, 25, 104, 76, 145, 31, 228, 222, 200, 253, 13, 105, 211, 221, 206, 253, 89, 47, 12,
    136, 239, 11, 225, 207, 110, 228, 196, 189, 245, 208, 220, 24, 84, 170, 70, 100, 247, 8, 145,
    8, 159, 123, 231, 164, 107, 88, 117, 237, 235, 30, 166, 81, 24, 178, 118, 148, 195, 179, 153,
    235, 8, 67, 86, 213, 179, 190, 155, 127, 66, 96, 53, 195, 226, 246, 182, 69, 194, 210, 80, 148,
    194, 202, 69, 236, 247, 204, 3, 10, 192, 55, 48, 61, 80, 41, 159, 233, 105, 102, 195, 173, 135,
    149, 120, 5, 156, 76, 4, 3, 122, 121, 59, 146, 87, 90, 216, 241, 32, 197, 140, 229, 98, 165,
    215, 90, 19, 154, 136, 227, 171, 145, 69, 207, 133, 14, 133, 105, 238, 14, 211, 53, 102, 24,
    92, 202, 47, 121, 100, 66, 141, 43, 134, 245, 200, 248, 99, 54, 247, 217, 190, 68, 244, 236,
    139, 153, 140, 82, 164, 21, 7, 6, 174, 147, 88, 56, 30, 57, 157, 41, 137, 203, 180, 126, 99,
    198, 96, 203, 183, 97, 142, 86, 232, 74, 103, 125, 233, 224, 182, 240, 240, 159, 206, 75, 236,
    73, 60, 236, 108, 237, 188, 240, 11, 123, 218, 182, 17, 209, 109, 82, 20, 221, 60, 1, 155, 240,
    104, 82, 44, 86, 166, 157, 78, 209, 190, 218, 177, 192, 156, 70, 74, 20, 77, 239, 46, 231, 184,
    93, 98, 97, 220, 47, 39, 214, 126, 64, 215, 10, 69, 175, 237, 59, 70, 122, 222, 190, 110, 215,
    20, 194, 101, 8, 250, 172, 22, 141, 178, 56, 172, 254, 234, 231, 211, 10, 114, 48, 76, 7, 131,
    87, 166, 239, 163, 227, 238, 95, 9, 253, 62, 233, 224, 92, 73, 37, 175, 33, 21, 81, 2, 155,
    218, 42, 205, 216, 9, 187, 55, 101, 22, 154, 66, 150, 60, 193, 152, 20, 43, 233, 51, 215, 12,
    52, 151, 117, 194, 194, 139, 189, 121, 37, 18, 42, 24, 63, 138, 140, 81, 18, 32, 8, 29, 235,
    221, 85, 72, 215, 170, 7, 235, 82, 50, 210, 178, 165, 155, 242, 69, 180, 58, 97, 235, 235, 253,
    247, 199, 64, 226, 20, 103, 19, 13, 36, 111, 77, 94, 236, 135, 166, 22, 215, 120, 220, 125, 83,
    249, 220, 84, 227, 19, 28, 192, 53, 162, 40, 79, 74, 30, 80, 219, 19, 91, 164, 66, 193, 23, 85,
    48, 209, 11, 225, 235, 102, 68, 22, 101, 68, 43, 134, 5, 59, 195, 175, 66, 142, 79, 164, 120,
    19, 37, 237, 94, 79, 20, 241, 217, 72, 251, 9, 213, 117, 101, 242, 132, 8, 15, 211, 5, 123, 84,
    133, 118, 215, 178, 115, 172, 248, 187, 138, 81, 57, 216, 70, 176, 37, 9, 0, 201, 9, 85, 170,
    36, 106, 116, 111, 93, 56, 239, 168, 190, 41, 182, 32, 184, 166, 30, 234, 230, 114, 7, 234,
    150, 173, 200, 250, 23, 233, 17, 244, 173, 143, 119, 92, 202, 63, 133, 26, 164, 0, 22, 52, 29,
    6, 201, 189, 238, 0, 161, 247, 211, 133, 133, 205, 177, 128, 161, 14, 105, 47, 185, 17, 114,
    84, 232, 154, 153, 87, 160, 188, 241, 44, 102, 132, 166, 25, 153, 96, 49, 134, 157, 57, 141,
    201, 29, 233, 204, 15, 144, 63, 201, 239, 237, 21, 142, 105, 200, 151, 41, 19, 209, 70, 102,
    94, 52, 23, 29, 5, 253, 33, 3, 168, 120, 42, 137, 105, 173, 29, 82, 172, 81, 179, 45, 140, 38,
    90, 77, 244, 251, 151, 45, 19, 162, 66, 35, 30, 190, 111, 175, 204, 183, 251, 30, 213, 143, 15,
    50, 219, 164, 85, 87, 60, 169, 43, 82, 193, 179, 65, 78, 193, 68, 87, 111, 63, 48, 25, 113,
    101, 116, 246, 76, 163, 92, 165, 170, 89, 177, 235, 100, 7, 253, 5, 77, 0, 134, 10, 98, 127,
    150, 178, 98, 219, 165, 237, 79, 190, 10, 19, 49, 172, 35, 212, 246, 16, 210, 84, 140, 225,
    194, 24, 149, 208, 243, 163, 5, 66, 119, 123, 185, 57, 174, 146, 226, 76, 251, 114, 141, 94,
    232, 41, 148, 127, 113, 246, 243, 162, 179, 233, 100, 127, 238, 32, 66, 16, 251, 108, 159, 170,
    231, 118, 181, 174, 166, 24, 36, 198, 175, 148, 178, 252, 217, 77, 42, 25, 226, 154, 117, 189,
    223, 144, 194, 243, 12, 149, 138, 113, 186, 131, 31, 127, 170,
];

fn setup_container(
    dtype: DiskType,
    bsize: u32,
    blocks: u64,
    ablocks: u64,
) -> Inner<Cursor<Vec<u8>>> {
    let mut store = setup_store();

    let data = {
        let options = OptionsBuilder::new(Cipher::Aes128Gcm)
            .with_dtype(dtype)
            .with_bsize(bsize)
            .with_blocks(blocks)
            .build()
            .unwrap();

        let cursor = Cursor::new(vec![]);
        let mut inner = Inner::create(cursor, options, &mut store).unwrap();

        inner.header.master_key = secure_vec![b'a'; 16];
        inner.header.master_iv = secure_vec![b'b'; 12];

        inner.flush_header(&mut store).unwrap();

        let nbytes = (bsize as u64 * (ablocks - 1)) as usize;

        inner.fh.seek(SeekFrom::Start(bsize as u64)).unwrap();
        inner.fh.write_all(&CIPHERTEXT[..nbytes]).unwrap();
        inner.fh.flush().unwrap();

        inner.as_ref().get_ref().to_vec()
    };

    Inner::open(Cursor::new(data), &mut store).unwrap()
}

// fn setup_key_iv(block_id: u64) -> (Vec<u8>, Vec<u8>) {
//     let mut master_key = vec![b'a'; 16];
//     let mut master_iv = vec![b'b'; 12];
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

//     let ciphertext = Cipher::Aes128Gcm
//         .encrypt(&PLAINTEXT[..496], &key, &iv)
//         .unwrap();
//     println!("{:?}", ciphertext);

//     let (key, iv) = setup_key_iv(2);
//     let ciphertext = Cipher::Aes128Gcm
//         .encrypt(&PLAINTEXT[512..1008], &key, &iv)
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
    let mut target = vec![b'x'; 496];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target, &PLAINTEXT[..496]);
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
    let mut target = vec![b'x'; 497];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target[..496], PLAINTEXT[..496]);
    assert_eq!(target[496], b'x');
}

#[test]
fn thin_zero_unallocated_full() {
    let mut inner = setup_container(DiskType::ThinZero, 512, 3, 2);
    let mut target = vec![b'x'; 496];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 496);
    assert_eq!(target, &vec![0u8; 496][..]);
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
    let mut target = vec![b'x'; 497];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 496);
    assert_eq!(target[..496], vec![0u8; 496][..]);
    assert_eq!(target[496], b'x');
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
    let mut target = vec![b'x'; 496];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target, &PLAINTEXT[..496]);
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
    let mut target = vec![b'x'; 497];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target[..496], PLAINTEXT[..496]);
    assert_eq!(target[496], b'x');
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
    let mut target = vec![b'x'; 496];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target, &PLAINTEXT[..496]);
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
    let mut target = vec![b'x'; 497];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target[..496], PLAINTEXT[..496]);
    assert_eq!(target[496], b'x');
}

#[test]
fn thin_random_unallocated_full() {
    let mut inner = setup_container(DiskType::ThinRandom, 512, 3, 2);
    let mut target = vec![b'x'; 496];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 496);
    assert_eq!(target, &RND[..496]);
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
    let mut target = vec![b'x'; 497];

    assert_eq!(inner.read_block(&mut target, 2).unwrap(), 496);
    assert_eq!(target[..496], RND[..496]);
    assert_eq!(target[496], b'x');
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
    let mut target = vec![b'x'; 496];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target, &PLAINTEXT[..496]);
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
    let mut target = vec![b'x'; 497];

    assert_eq!(inner.read_block(&mut target, 1).unwrap(), 496);
    assert_eq!(target[..496], PLAINTEXT[..496]);
    assert_eq!(target[496], b'x');
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

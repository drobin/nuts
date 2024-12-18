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

use crate::{buffer::Buffer, BufferError};

#[test]
fn get_chunk_empty_none() {
    assert!([].as_slice().get_chunk(0).unwrap().is_empty());
}

#[test]
fn get_chunk_empty_some() {
    let err = [].as_slice().get_chunk(1).unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

#[test]
fn get_chunk_none() {
    let mut slice = [1, 2, 3].as_slice();

    assert!(slice.get_chunk(0).unwrap().is_empty());
    assert_eq!(slice, [1, 2, 3]);
}

#[test]
fn get_chunk_some() {
    let mut slice = [1, 2, 3].as_slice();

    assert_eq!(slice.get_chunk(1).unwrap(), [1]);
    assert_eq!(slice, [2, 3]);
}

#[test]
fn get_chunk_all() {
    let mut slice = [1, 2, 3].as_slice();

    assert_eq!(slice.get_chunk(3).unwrap(), [1, 2, 3]);
    assert!(slice.is_empty());
}

#[test]
fn get_chunk_more() {
    let mut slice = [1, 2, 3].as_slice();
    let err = slice.get_chunk(4).unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

#[test]
fn get_array_empty_none() {
    let arr: [u8; 0] = [].as_slice().get_array().unwrap();

    assert!(arr.is_empty());
}

#[test]
fn get_array_empty_some() {
    let err = [].as_slice().get_array::<1>().unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

#[test]
fn get_array_none() {
    let mut slice = [1, 2, 3].as_slice();
    let arr: [u8; 0] = slice.get_array().unwrap();

    assert!(arr.is_empty());
    assert_eq!(slice, [1, 2, 3]);
}

#[test]
fn get_array_some() {
    let mut slice = [1, 2, 3].as_slice();
    let arr: [u8; 1] = slice.get_array().unwrap();

    assert_eq!(arr, [1]);
    assert_eq!(slice, [2, 3]);
}

#[test]
fn get_array_all() {
    let mut slice = [1, 2, 3].as_slice();
    let arr: [u8; 3] = slice.get_array().unwrap();

    assert_eq!(arr, [1, 2, 3]);
    assert!(slice.is_empty());
}

#[test]
fn get_array_more() {
    let mut slice = [1, 2, 3].as_slice();
    let err = slice.get_array::<4>().unwrap_err();

    assert!(matches!(err, BufferError::UnexpectedEof));
}

// #[test]
// fn get_u8() {
//     let mut slice = [1, 2, 3].as_slice();
//     let n = slice.get_u8().unwrap();

//     assert_eq!(n, 1);
//     assert_eq!(slice, [2, 3]);
// }

// #[test]
// fn get_u8_eof() {
//     let err = [].as_slice().get_u8().unwrap_err();

//     assert!(matches!(err, BufferError::UnexpectedEof));
// }

// #[test]
// fn get_u16() {
//     let mut slice = [1, 2, 3].as_slice();
//     let n = slice.get_u16().unwrap();

//     assert_eq!(n, 258);
//     assert_eq!(slice, [3]);
// }

// #[test]
// fn get_u16_eof() {
//     let mut slice = [1].as_slice();
//     let err = slice.get_u16().unwrap_err();

//     assert_eq!(slice, [1]);
//     assert!(matches!(err, BufferError::UnexpectedEof));
// }

#[test]
fn get_u32() {
    let mut slice = [1, 2, 3, 4, 5].as_slice();
    let n = slice.get_u32().unwrap();

    assert_eq!(n, 16_909_060);
    assert_eq!(slice, [5]);
}

#[test]
fn get_u32_eof() {
    let mut slice = [1, 2, 3].as_slice();
    let err = slice.get_u32().unwrap_err();

    assert_eq!(slice, [1, 2, 3]);
    assert!(matches!(err, BufferError::UnexpectedEof));
}

// #[test]
// fn get_u64() {
//     let mut slice = [1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice();
//     let n = slice.get_u64().unwrap();

//     assert_eq!(n, 72_623_859_790_382_856);
//     assert_eq!(slice, [9]);
// }

// #[test]
// fn get_u64_eof() {
//     let mut slice = [1, 2, 3, 4, 5, 6, 7].as_slice();
//     let err = slice.get_u64().unwrap_err();

//     assert_eq!(slice, [1, 2, 3, 4, 5, 6, 7]);
//     assert!(matches!(err, BufferError::UnexpectedEof));
// }

macro_rules! vec_tests {
    ($mod:ident,$len:literal) => {
        mod $mod {

            use crate::buffer::{Buffer, BufferError};

            #[test]
            fn get_vec_eof_len() {
                let mut slice = [0; $len - 1].as_slice();
                let err = slice.get_vec::<$len>().unwrap_err();

                assert!(matches!(err, BufferError::UnexpectedEof));
            }

            #[test]
            fn get_vec_eof_data() {
                let mut slice = [0; $len];

                slice[$len - 1] = 1;

                let err = slice.as_slice().get_vec::<$len>().unwrap_err();

                assert!(matches!(err, BufferError::UnexpectedEof));
            }

            #[test]
            fn get_vec_empty() {
                let mut buf = [0; $len + 1];

                buf[$len] = b'x';

                let mut slice = buf.as_slice();
                let vec = slice.get_vec::<$len>().unwrap();

                assert!(vec.is_empty());
                assert_eq!(slice, b"x");
            }

            #[test]
            fn get_vec() {
                let mut buf = [0; $len + 4];

                buf[$len - 1..].copy_from_slice(&[3, 1, 2, 3, b'x']);

                let mut slice = buf.as_slice();
                let vec = slice.get_vec::<$len>().unwrap();

                assert_eq!(vec, [1, 2, 3]);
                assert_eq!(slice, b"x");
            }
        }
    };
}

vec_tests!(vec_1, 1);
vec_tests!(vec_2, 2);
vec_tests!(vec_3, 3);
vec_tests!(vec_4, 4);
vec_tests!(vec_5, 5);
vec_tests!(vec_6, 6);
vec_tests!(vec_7, 7);
vec_tests!(vec_8, 8);
vec_tests!(vec_9, 9);

#[test]
fn get_vec_too_large() {
    if cfg!(target_pointer_width = "64") {
        let mut slice = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff].as_slice();
        let err = slice.get_vec::<9>().unwrap_err();

        assert!(matches!(err, BufferError::VecTooLarge));
    } else {
        unimplemented!("test only implemented for x64")
    }
}

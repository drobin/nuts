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

use crate::buffer::BufferMut;

#[test]
fn put_chunk_none() {
    let mut vec = vec![];

    vec.put_chunk(&[]).unwrap();

    assert!(vec.is_empty());
}

#[test]
fn put_chunk_some() {
    let mut vec = vec![];

    vec.put_chunk(&[1, 2, 3]).unwrap();

    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn put_chunk_append() {
    let mut vec = vec![1, 2, 3];

    vec.put_chunk(&[4, 5]).unwrap();

    assert_eq!(vec, [1, 2, 3, 4, 5]);
}

#[test]
fn put_u8() {
    let mut vec = vec![];

    vec.put_u8(1).unwrap();

    assert_eq!(vec, [1]);
}

#[test]
fn put_u16() {
    let mut vec = vec![];

    vec.put_u16(258).unwrap();

    assert_eq!(vec, [1, 2]);
}

#[test]
fn put_u32() {
    let mut vec = vec![];

    vec.put_u32(16_909_060).unwrap();

    assert_eq!(vec, [1, 2, 3, 4]);
}

#[test]
fn put_u64() {
    let mut vec = vec![];

    vec.put_u64(72_623_859_790_382_856).unwrap();

    assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn put_usize() {
    let mut vec = vec![];

    vec.put_usize(72_623_859_790_382_856).unwrap();

    assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn put_vec_empty() {
    let mut vec = vec![];

    vec.put_vec(&[]).unwrap();

    assert_eq!(vec, [0; 8]);
}

#[test]
fn put_vec() {
    let mut vec = vec![];

    vec.put_vec(&[1, 2, 3]).unwrap();

    assert_eq!(vec, [0, 0, 0, 0, 0, 0, 0, 3, 1, 2, 3]);
}

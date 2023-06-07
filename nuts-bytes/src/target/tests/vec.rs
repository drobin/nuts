// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use crate::target::PutBytes;

#[test]
fn empty_put_0() {
    let mut target = vec![];

    target.put_bytes(&[]).unwrap();
    assert_eq!(target, []);
}

#[test]
fn empty_put_1() {
    let mut target = vec![];

    target.put_bytes(&[1]).unwrap();
    assert_eq!(target, [1]);
}

#[test]
fn empty_put_2() {
    let mut target = vec![];

    target.put_bytes(&[1, 2]).unwrap();
    assert_eq!(target, [1, 2]);
}

#[test]
fn empty_put_3() {
    let mut target = vec![];

    target.put_bytes(&[1, 2, 3]).unwrap();
    assert_eq!(target, [1, 2, 3]);
}

#[test]
fn put_0() {
    let mut target = vec![b'x'];

    target.put_bytes(&[]).unwrap();
    assert_eq!(target, [b'x']);
}

#[test]
fn put_1() {
    let mut target = vec![b'x'];

    target.put_bytes(&[1]).unwrap();
    assert_eq!(target, [b'x', 1]);
}

#[test]
fn put_2() {
    let mut target = vec![b'x'];

    target.put_bytes(&[1, 2]).unwrap();
    assert_eq!(target, [b'x', 1, 2]);
}

#[test]
fn put_3() {
    let mut target = vec![b'x'];

    target.put_bytes(&[1, 2, 3]).unwrap();
    assert_eq!(target, [b'x', 1, 2, 3]);
}

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

use crate::utils::SecureVec;

#[test]
fn no_args() {
    let v: SecureVec<u8> = secure_vec!();
    assert!(v.is_empty());
}

#[test]
fn empty() {
    let v: SecureVec<u8> = secure_vec![];
    assert!(v.is_empty());
}

#[test]
fn from_empty_array() {
    let v = secure_vec![9; 0];
    assert!(v.is_empty());
}

#[test]
fn from_array() {
    let v = secure_vec![9; 3];
    assert_eq!(v, vec![9, 9, 9]);
}

#[test]
fn from_elems() {
    let v = secure_vec![9, 9, 9];
    assert_eq!(v, vec![9, 9, 9]);
}

#[test]
fn from_empty_vec() {
    let v: SecureVec<u8> = SecureVec::new(vec![]);
    assert!(v.is_empty());
}

#[test]
fn from_vec() {
    let v: SecureVec<u8> = SecureVec::new(vec![9; 3]);
    assert_eq!(v, vec![9, 9, 9]);
}

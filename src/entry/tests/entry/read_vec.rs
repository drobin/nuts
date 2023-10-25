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

use crate::entry::tests::entry::setup_archive;

#[test]
fn empty() {
    let mut archive = setup_archive(0);
    let mut entry = archive.first().unwrap().unwrap();
    assert_eq!(entry.read_vec().unwrap(), []);
}

#[test]
fn half() {
    let mut archive = setup_archive(46);
    let mut entry = archive.first().unwrap().unwrap();
    assert_eq!(entry.read_vec().unwrap(), (0..46).collect::<Vec<u8>>());
}

#[test]
fn full() {
    let mut archive = setup_archive(92);
    let mut entry = archive.first().unwrap().unwrap();
    assert_eq!(entry.read_vec().unwrap(), (0..92).collect::<Vec<u8>>());
}

#[test]
fn full_half() {
    let mut archive = setup_archive(138);
    let mut entry = archive.first().unwrap().unwrap();
    assert_eq!(entry.read_vec().unwrap(), (0..138).collect::<Vec<u8>>());
}

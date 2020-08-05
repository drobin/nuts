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

use std::io::Cursor;

use crate::io::WriteBasics;

#[test]
fn u8() {
    let mut c = Cursor::new(vec![]);

    c.write_u8(6).unwrap();
    assert_eq!(c.get_ref(), &[6]);
}

#[test]
fn u32() {
    let mut c = Cursor::new(vec![]);

    c.write_u32(4711).unwrap();
    assert_eq!(c.get_ref(), &[0x00, 0x00, 0x12, 0x67]);
}

#[test]
fn u64() {
    let mut c = Cursor::new(vec![]);

    c.write_u64(1_326_049_953_023_858_032).unwrap();
    assert_eq!(
        c.get_ref(),
        &[0x12, 0x67, 0x13, 0x68, 0x14, 0x69, 0x15, 0x70]
    );
}

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

use std::io::Read;

use crate::header::ser::HeaderReader;

#[test]
fn no_data_empty() {
    let data = [];
    let mut reader = HeaderReader::new(&data);
    let mut buf = [];

    assert_eq!(reader.read(&mut buf).unwrap(), 0);
}

#[test]
fn no_data_none_empty() {
    let data = [];
    let mut reader = HeaderReader::new(&data);
    let mut buf = [b'x'; 8];

    assert_eq!(reader.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [b'x', b'x', b'x', b'x', b'x', b'x', b'x', b'x'])
}

#[test]
fn complete() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = HeaderReader::new(&data);
    let mut buf = [b'x'; 8];

    assert_eq!(reader.read(&mut buf).unwrap(), 8);
    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8])
}

#[test]
fn complete_large_buf() {
    let data = [1, 2, 3, 4, 5, 6, 7];
    let mut reader = HeaderReader::new(&data);
    let mut buf = [b'x'; 8];

    assert_eq!(reader.read(&mut buf).unwrap(), 7);
    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, b'x'])
}

#[test]
fn incomplete() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = HeaderReader::new(&data);
    let mut buf = [b'x'; 5];

    assert_eq!(reader.read(&mut buf).unwrap(), 5);
    assert_eq!(buf, [1, 2, 3, 4, 5])
}

#[test]
fn byte_by_byte() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = HeaderReader::new(&data);

    for i in 0..8 {
        let mut buf = [b'x'; 1];
        assert_eq!(reader.read(&mut buf).unwrap(), 1);
        assert_eq!(buf, [i + 1]);
    }
}

#[test]
fn two_bytes_by_two_bytes() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = HeaderReader::new(&data);

    for i in 0..4 {
        let mut buf = [b'x'; 2];
        assert_eq!(reader.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [2 * i + 1, 2 * i + 2]);
    }
}

#[test]
fn three_bytes_by_three_bytes() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = HeaderReader::new(&data);
    let mut buf = [b'x'; 3];

    assert_eq!(reader.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(reader.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(reader.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [7, 8, 6]);
}

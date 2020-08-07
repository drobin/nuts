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

use std::io::Write;

use crate::header::ser::HeaderWriter;

#[test]
fn no_data_empty() {
    let mut data = [];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[]).unwrap(), 0);
    assert_eq!(data, []);
}

#[test]
fn no_data_none_empty() {
    let mut data = [];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 0);
    assert_eq!(data, []);
}

#[test]
fn complete() {
    let mut data = [b'x'; 3];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(data, [1, 2, 3]);
}

#[test]
fn complete_large_data() {
    let mut data = [b'x'; 4];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(data, [1, 2, 3, b'x']);
}

#[test]
fn incomplete() {
    let mut data = [b'x'; 3];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1, 2, 3, 4]).unwrap(), 3);
    assert_eq!(data, [1, 2, 3]);
}

#[test]
fn byte_by_byte() {
    let mut data = [b'x'; 8];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1]).unwrap(), 1);
    assert_eq!(writer.write(&[2]).unwrap(), 1);
    assert_eq!(writer.write(&[3]).unwrap(), 1);
    assert_eq!(writer.write(&[4]).unwrap(), 1);
    assert_eq!(writer.write(&[5]).unwrap(), 1);
    assert_eq!(writer.write(&[6]).unwrap(), 1);
    assert_eq!(writer.write(&[7]).unwrap(), 1);
    assert_eq!(writer.write(&[8]).unwrap(), 1);
    assert_eq!(writer.write(&[9]).unwrap(), 0);

    assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn two_bytes_by_two_bytes() {
    let mut data = [b'x'; 8];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1, 2]).unwrap(), 2);
    assert_eq!(writer.write(&[3, 4]).unwrap(), 2);
    assert_eq!(writer.write(&[5, 6]).unwrap(), 2);
    assert_eq!(writer.write(&[7, 8]).unwrap(), 2);
    assert_eq!(writer.write(&[9, 0]).unwrap(), 0);

    assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn three_bytes_by_three_bytes() {
    let mut data = [b'x'; 8];
    let mut writer = HeaderWriter::new(&mut data);

    assert_eq!(writer.write(&[1, 2, 3]).unwrap(), 3);
    assert_eq!(writer.write(&[4, 5, 6]).unwrap(), 3);
    assert_eq!(writer.write(&[7, 8, 9]).unwrap(), 2);
    assert_eq!(writer.write(&[0, 1]).unwrap(), 0);

    assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8]);
}

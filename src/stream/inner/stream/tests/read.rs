// MIT License
//
// Copyright (c) 2022,2023 Robin Doer
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

use crate::container::Container;
use crate::memory::{Id, MemoryBackend};
use crate::stream::{Error, OpenOptions};

use crate::stream::testutils::{
    setup_one, setup_one_with, setup_three, setup_three_with, setup_two, setup_two_with,
};

fn one(with: Option<&[u8]>) -> (Container<MemoryBackend>, Id) {
    match with {
        Some(payload) => setup_one_with(payload),
        None => setup_one(),
    }
}

fn two(with: Option<(&[u8], &[u8])>) -> (Container<MemoryBackend>, Id) {
    let (container, (id1, _)) = match with {
        Some((payload1, payload2)) => setup_two_with(payload1, payload2),
        None => setup_two(),
    };

    (container, id1)
}

fn three(with: Option<(&[u8], &[u8], &[u8])>) -> (Container<MemoryBackend>, Id) {
    let (container, (id1, ..)) = match with {
        Some((payload1, payload2, payload3)) => setup_three_with(payload1, payload2, payload3),
        None => setup_three(),
    };
    (container, id1)
}

macro_rules! open {
    ($setup:ident) => {{
        let (container, id) = $setup(None);
        OpenOptions::new().read(true).open(container, id).unwrap()
    }};

    ($setup:ident, $($with:expr),+) => {{
        let (container, id) = $setup(Some(($($with),*)));
        OpenOptions::new().read(true).open(container, id).unwrap()
    }};
}

#[test]
fn no_read() {
    for option in [OpenOptions::new(), *OpenOptions::new().read(false)] {
        let (container, id) = setup_one();
        let mut stream = option.open(container, id).unwrap();
        let mut buf = [0; 3];

        let err = stream.read(&mut buf).unwrap_err();
        assert_error!(err, Error::NotReadable);
    }
}

#[test]
fn one_full() {
    let mut stream = open!(one);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn one_more() {
    let mut stream = open!(one);
    let mut buf = [0; 4];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn one_part() {
    let mut stream = open!(one);
    let mut buf = [0; 2];

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [3, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn two_full() {
    let mut stream = open!(two);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn two_more() {
    let mut stream = open!(two);
    let mut buf = [0; 4];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn two_part() {
    let mut stream = open!(two);
    let mut buf = [0; 2];

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [3, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [4, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [6, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn three_full() {
    let mut stream = open!(three);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [7, 8, 9]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn three_more() {
    let mut stream = open!(three);
    let mut buf = [0; 4];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [7, 8, 9, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn three_part() {
    let mut stream = open!(three);
    let mut buf = [0; 2];

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [3, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [4, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [6, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [7, 8]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [9, 8]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn one_with_empty() {
    let mut stream = open!(one, &[]);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn two_with_empty_front() {
    let mut stream = open!(two, &[], &[4, 5, 6]);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn two_with_empty_back() {
    let mut stream = open!(two, &[1, 2, 3], &[]);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn three_with_empty_front() {
    let mut stream = open!(three, &[], &[4, 5, 6], &[7, 8, 9]);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [7, 8, 9]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn three_with_empty_mid() {
    let mut stream = open!(three, &[1, 2, 3], &[], &[7, 8, 9]);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [7, 8, 9]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[test]
fn three_with_empty_back() {
    let mut stream = open!(three, &[1, 2, 3], &[4, 5, 6], &[]);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

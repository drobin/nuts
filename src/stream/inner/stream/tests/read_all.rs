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

macro_rules! mk_test {
    ($name:ident ($setup:ident), $num:literal => $expected:expr) => {
        #[test]
        fn $name() {
            let mut stream = open!($setup);
            let mut buf = [0; $num];

            stream.read_all(&mut buf).unwrap();
            assert_eq!(buf, $expected);
        }
    };
}

#[test]
fn no_read() {
    for option in [OpenOptions::new(), *OpenOptions::new().read(false)] {
        let (container, id) = setup_one();
        let mut stream = option.open(container, id).unwrap();
        let mut buf = [0; 3];

        let err = stream.read_all(&mut buf).unwrap_err();
        assert_error!(err, Error::NotReadable);
    }
}

mk_test!(one_0(one), 0 => []);
mk_test!(one_1(one), 1 => [1]);
mk_test!(one_2(one), 2 => [1, 2]);
mk_test!(one_3(one), 3 => [1, 2, 3]);

#[test]
fn one_4() {
    let mut stream = open!(one);
    let mut buf = [0; 4];

    let err = stream.read_all(&mut buf).unwrap_err();
    assert_error!(err, Error::ReadAll);
}

mk_test!(two_0(two), 0 => []);
mk_test!(two_1(two), 1 => [1]);
mk_test!(two_2(two), 2 => [1, 2]);
mk_test!(two_3(two), 3 => [1, 2, 3]);
mk_test!(two_4(two), 4 => [1, 2, 3, 4]);
mk_test!(two_5(two), 5 => [1, 2, 3, 4, 5]);
mk_test!(two_6(two), 6 => [1, 2, 3, 4, 5, 6]);

#[test]
fn two_7() {
    let mut stream = open!(two);
    let mut buf = [0; 7];

    let err = stream.read_all(&mut buf).unwrap_err();
    assert_error!(err, Error::ReadAll);
}

mk_test!(three_0(three), 0 => []);
mk_test!(three_1(three), 1 => [1]);
mk_test!(three_2(three), 2 => [1, 2]);
mk_test!(three_3(three), 3 => [1, 2, 3]);
mk_test!(three_4(three), 4 => [1, 2, 3, 4]);
mk_test!(three_5(three), 5 => [1, 2, 3, 4, 5]);
mk_test!(three_6(three), 6 => [1, 2, 3, 4, 5, 6]);
mk_test!(three_7(three), 7 => [1, 2, 3, 4, 5, 6, 7]);
mk_test!(three_8(three), 8 => [1, 2, 3, 4, 5, 6, 7, 8]);
mk_test!(three_9(three), 9 => [1, 2, 3, 4, 5, 6, 7, 8, 9]);

#[test]
fn three_10() {
    let mut stream = open!(three);
    let mut buf = [0; 10];

    let err = stream.read_all(&mut buf).unwrap_err();
    assert_error!(err, Error::ReadAll);
}

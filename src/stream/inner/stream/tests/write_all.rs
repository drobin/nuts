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
use crate::openssl::rand::RND;
use crate::stream::inner::Inner;
use crate::stream::{Error, OpenOptions};

use crate::stream::testutils::{setup_one, setup_three, setup_two};

fn one() -> (Container<MemoryBackend>, Id) {
    setup_one()
}

fn two() -> (Container<MemoryBackend>, Id) {
    let (container, (id1, ..)) = setup_two();
    (container, id1)
}

fn three() -> (Container<MemoryBackend>, Id) {
    let (container, (id1, ..)) = setup_three();
    (container, id1)
}

macro_rules! mk_test {
    ($name:ident ($setup:ident), $in:expr => $($out:expr),+) => {
        #[test]
        fn $name() {
            let mut inner = {
                let (container, id) = $setup();
                let mut stream = OpenOptions::new().write(true).open(container, id.clone()).unwrap();

                stream.write_all($in).unwrap();

                Inner::open(stream.inner.into_container(), id).unwrap()
            };

            $(
                inner.goto_next().unwrap().unwrap();
                assert_eq!(inner.payload().unwrap(), $out);
            )*

            assert!(inner.goto_next().is_none());
        }
    };
}

#[test]
fn no_write() {
    for options in [OpenOptions::new(), *OpenOptions::new().write(false)] {
        let (container, id) = setup_one();
        let mut stream = options.open(container, id).unwrap();

        let err = stream.write_all(b"abc").unwrap_err();
        assert_error!(err, Error::NotWritable);
    }
}

mk_test!(one_0(one), &[] => &[1, 2, 3]);
mk_test!(one_1(one), b"a" => &[b'a', 2, 3]);
mk_test!(one_2(one), b"ab" => &[b'a', b'b', 3]);
mk_test!(one_3(one), b"abc" => b"abc");
mk_test!(one_4(one), b"abcd" => b"abcd");
mk_test!(one_5(one), b"abcde" => b"abcde");
mk_test!(one_6(one), &RND[..499] => &RND[..499]);
mk_test!(one_7(one), &RND[..500] => &RND[..500]);
mk_test!(one_8(one), &RND[..516] => &RND[..500], &RND[500..516]);

mk_test!(two_0(two), &[] => &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_1(two), b"a" => &[b'a', 2, 3], &[4, 5, 6]);
mk_test!(two_2(two), b"ab" => &[b'a', b'b', 3], &[4, 5, 6]);
mk_test!(two_3(two), b"abc" => b"abc", &[4, 5, 6]);
mk_test!(two_4(two), b"abcd" => b"abc", &[b'd', 5, 6]);
mk_test!(two_5(two), b"abcde" => b"abc", &[b'd', b'e', 6]);
mk_test!(two_6(two), b"abcdef" => b"abc", b"def");
mk_test!(two_7(two), b"abcdefg" => b"abc", b"defg");
mk_test!(two_8(two), b"abcdefgh" => b"abc", b"defgh");
mk_test!(two_9(two), &RND[..502] => &RND[..3], &RND[3..502]);
mk_test!(two_10(two), &RND[..503] => &RND[..3], &RND[3..503]);
mk_test!(two_11(two), &RND[..519] => &RND[..3], &RND[3..503], &RND[503..519]);

mk_test!(three_0(three), &[] => &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_1(three), b"a" => &[b'a', 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_2(three), b"ab" => &[b'a', b'b', 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_3(three), b"abc" => b"abc", &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_4(three), b"abcd" => b"abc", &[b'd', 5, 6], &[7, 8, 9]);
mk_test!(three_5(three), b"abcde" => b"abc", &[b'd', b'e', 6], &[7, 8, 9]);
mk_test!(three_6(three), b"abcdef" => b"abc", b"def", &[7, 8, 9]);
mk_test!(three_7(three), b"abcdefg" => b"abc", b"def", &[b'g', 8, 9]);
mk_test!(three_8(three), b"abcdefgh" => b"abc", b"def", &[b'g', b'h', 9]);
mk_test!(three_9(three), b"abcdefghi" => b"abc", b"def", b"ghi");
mk_test!(three_10(three), b"abcdefghij" => b"abc", b"def", b"ghij");
mk_test!(three_11(three), b"abcdefghijk" => b"abc", b"def", b"ghijk");
mk_test!(three_12(three), &RND[..505] => &RND[..3], &RND[3..6], &RND[6..505]);
mk_test!(three_13(three), &RND[..506] => &RND[..3], &RND[3..6], &RND[6..506]);
mk_test!(three_14(three), &RND[..522] => &RND[..3], &RND[3..6], &RND[6..506], &RND[506..522]);

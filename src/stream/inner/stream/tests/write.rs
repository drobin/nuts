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
use crate::stream::inner::Inner;
use crate::stream::{Error, OpenOptions, Position};
use crate::tests::RND;

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
    ($name:ident ($setup:ident), $pos:literal, $($in:expr, $inlen:literal),+ -> $($out:expr),+) => {
        #[test]
        fn $name() {
            let mut inner = {
                let (container, id) = $setup();
                let mut stream = OpenOptions::new().write(true).open(container, id.clone()).unwrap();

                stream.seek(Position::Start($pos)).unwrap();

                $(
                    assert_eq!(stream.write($in).unwrap(), $inlen);
                )*
                stream.flush().unwrap();

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

        let err = stream.write(b"abc").unwrap_err();
        assert_error!(err, Error::NotWritable);
    }
}

mk_test!(one_0_0(one), 0, &[], 0 -> &[1, 2, 3]);
mk_test!(one_0_1(one), 0, b"a", 1 -> &[b'a', 2, 3]);
mk_test!(one_0_2(one), 0, b"ab", 2 -> &[b'a', b'b', 3]);
mk_test!(one_0_3(one), 0, b"abc", 3 -> b"abc");
mk_test!(one_0_4(one), 0, b"abcd", 3, b"d", 1 -> b"abcd");
mk_test!(one_0_5(one), 0, b"abcd", 3, b"de", 2 -> b"abcde");
mk_test!(one_0_6(one), 0, &RND, 3, &RND[3..499], 496 -> &RND[..499]);
mk_test!(one_0_7(one), 0, &RND, 3, &RND[3..], 497 -> &RND[..500]);
mk_test!(one_0_8(one), 0, &RND, 3, &RND[3..], 497, &RND[500..516], 16 -> &RND[..500], &RND[500..516]);

mk_test!(one_1_0(one), 1, &[], 0 -> &[1, 2, 3]);
mk_test!(one_1_1(one), 1, b"a", 1 -> &[1, b'a', 3]);
mk_test!(one_1_2(one), 1, b"ab", 2 -> &[1, b'a', b'b']);
mk_test!(one_1_3(one), 1, b"abc", 2, b"c", 1 -> &[1, b'a', b'b', b'c']);
mk_test!(one_1_4(one), 1, b"abc", 2, b"cd", 2 -> &[1, b'a', b'b', b'c', b'd']);
mk_test!(one_1_5(one), 1, &RND, 2, &RND[2..498], 496 -> [&[1], &RND[..498]].concat());
mk_test!(one_1_6(one), 1, &RND, 2, &RND[2..], 497 -> [&[1], &RND[..499]].concat());
mk_test!(one_1_7(one), 1, &RND, 2, &RND[2..], 497, &RND[499..515], 16 -> [&[1], &RND[..499]].concat(), &RND[499..515]);

mk_test!(one_2_0(one), 2, &[], 0 -> &[1, 2, 3]);
mk_test!(one_2_1(one), 2, b"a", 1 -> &[1, 2, b'a']);
mk_test!(one_2_2(one), 2, b"ab", 1, b"b", 1 -> &[1, 2, b'a', b'b']);
mk_test!(one_2_3(one), 2, b"ab", 1, b"bc", 2 -> &[1, 2, b'a', b'b', b'c']);
mk_test!(one_2_4(one), 2, &RND, 1, &RND[1..497], 496 -> [&[1, 2], &RND[..497]].concat());
mk_test!(one_2_5(one), 2, &RND, 1, &RND[1..], 497 -> [&[1, 2], &RND[..498]].concat());
mk_test!(one_2_6(one), 2, &RND, 1, &RND[1..], 497, &RND[498..514], 16 -> [&[1, 2], &RND[..498]].concat(), &RND[498..514]);

mk_test!(one_3_0(one), 3, &[], 0 -> &[1, 2, 3]);
mk_test!(one_3_1(one), 3, b"a", 1 -> &[1, 2, 3, b'a']);
mk_test!(one_3_2(one), 3, b"ab", 2 -> &[1, 2, 3, b'a', b'b']);
mk_test!(one_3_3(one), 3, &RND[..496], 496 -> [&[1, 2, 3], &RND[..496]].concat());
mk_test!(one_3_4(one), 3, &RND, 497 -> [&[1, 2, 3], &RND[..497]].concat());
mk_test!(one_3_5(one), 3, &RND, 497, &RND[497..513], 16 -> [&[1, 2, 3], &RND[..497]].concat(), &RND[497..513]);

mk_test!(two_0_0(two), 0, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_0_1(two), 0, b"a", 1 -> &[b'a', 2, 3], &[4, 5, 6]);
mk_test!(two_0_2(two), 0, b"ab", 2 -> &[b'a', b'b', 3], &[4, 5, 6]);
mk_test!(two_0_3(two), 0, b"abc", 3 -> b"abc", &[4, 5, 6]);
mk_test!(two_0_4(two), 0, b"abcd", 3, b"d", 1 -> b"abc", &[b'd', 5, 6]);
mk_test!(two_0_5(two), 0, b"abcd", 3, b"de", 2 -> b"abc", &[b'd', b'e', 6]);
mk_test!(two_0_6(two), 0, b"abcd", 3, b"def", 3 -> b"abc", b"def");
mk_test!(two_0_7(two), 0, b"abcd", 3, b"defg", 3, b"g", 1 -> b"abc", b"defg");
mk_test!(two_0_8(two), 0, b"abcd", 3, b"defg", 3, b"gh", 2 -> b"abc", b"defgh");
mk_test!(two_0_9(two), 0, &RND, 3, &RND[3..], 3, &RND[6..502], 496 -> &RND[..3], &RND[3..502]);
mk_test!(two_0_10(two), 0, &RND, 3, &RND[3..], 3, &RND[6..], 497 -> &RND[..3], &RND[3..503]);
mk_test!(two_0_11(two), 0, &RND, 3, &RND[3..], 3, &RND[6..], 497, &RND[503..519], 16 -> &RND[..3], &RND[3..503], &RND[503..519]);

mk_test!(two_1_0(two), 1, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_1_1(two), 1, b"a", 1 -> &[1, b'a', 3], &[4, 5, 6]);
mk_test!(two_1_2(two), 1, b"ab", 2 -> &[1, b'a', b'b'], &[4, 5, 6]);
mk_test!(two_1_3(two), 1, b"abc", 2, b"c", 1 -> &[1, b'a', b'b'], &[b'c', 5, 6]);
mk_test!(two_1_4(two), 1, b"abc", 2, b"cd", 2 -> &[1, b'a', b'b'], &[b'c', b'd', 6]);
mk_test!(two_1_5(two), 1, b"abc", 2, b"cde", 3 -> &[1, b'a', b'b'], b"cde");
mk_test!(two_1_6(two), 1, b"abc", 2, b"cdef", 3, b"f", 1 -> &[1, b'a', b'b'], b"cdef");
mk_test!(two_1_7(two), 1, b"abc", 2, b"cdef", 3, b"fg", 2 -> &[1, b'a', b'b'], b"cdefg");
mk_test!(two_1_9(two), 1, &RND, 2, &RND[2..], 3, &RND[5..], 497 -> [&[1], &RND[..2]].concat(), &RND[2..502]);
mk_test!(two_1_10(two), 1, &RND, 2, &RND[2..], 3, &RND[5..], 497, &RND[502..518], 16 -> [&[1], &RND[..2]].concat(), &RND[2..502], &RND[502..518]);

mk_test!(two_2_0(two), 2, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_2_1(two), 2, b"a", 1 -> &[1, 2, b'a'], &[4, 5, 6]);
mk_test!(two_2_2(two), 2, b"ab", 1, b"b", 1 -> &[1, 2, b'a'], &[b'b', 5, 6]);
mk_test!(two_2_3(two), 2, b"ab", 1, b"bc", 2 -> &[1, 2, b'a'], &[b'b', b'c', 6]);
mk_test!(two_2_4(two), 2, b"ab", 1, b"bcd", 3 -> &[1, 2, b'a'], b"bcd");
mk_test!(two_2_5(two), 2, b"ab", 1, b"bcde", 3, b"e", 1 -> &[1, 2, b'a'], b"bcde");
mk_test!(two_2_6(two), 2, b"ab", 1, b"bcde", 3, b"ef", 2 -> &[1, 2, b'a'], b"bcdef");
mk_test!(two_2_7(two), 2, &RND, 1, &RND[1..], 3, &RND[4..500], 496 -> &[1, 2, RND[0]], &RND[1..500]);
mk_test!(two_2_8(two), 2, &RND, 1, &RND[1..], 3, &RND[4..], 497 -> &[1, 2, RND[0]], &RND[1..501]);
mk_test!(two_2_9(two), 2, &RND, 1, &RND[1..], 3, &RND[4..], 497, &RND[501..517], 16 -> &[1, 2, RND[0]], &RND[1..501], &RND[501..517]);

mk_test!(two_3_0(two), 3, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_3_1(two), 3, b"a", 1 -> &[1, 2, 3], &[b'a', 5, 6]);
mk_test!(two_3_2(two), 3, b"ab", 2 -> &[1, 2, 3], &[b'a', b'b', 6]);
mk_test!(two_3_3(two), 3, b"abc", 3 -> &[1, 2, 3], b"abc");
mk_test!(two_3_4(two), 3, b"abcd", 3, b"d", 1 -> &[1, 2, 3], b"abcd");
mk_test!(two_3_5(two), 3, b"abcd", 3, b"de", 2 -> &[1, 2, 3], b"abcde");
mk_test!(two_3_6(two), 3, &RND, 3, &RND[3..499], 496 -> &[1, 2, 3], &RND[..499]);
mk_test!(two_3_7(two), 3, &RND, 3, &RND[3..], 497 -> &[1, 2, 3], &RND[..500]);
mk_test!(two_3_8(two), 3, &RND, 3, &RND[3..], 497, &RND[500..516], 16 -> &[1, 2, 3], &RND[..500], &RND[500..516]);

mk_test!(two_4_0(two), 4, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_4_1(two), 4, b"a", 1 -> &[1, 2, 3], &[4, b'a', 6]);
mk_test!(two_4_2(two), 4, b"ab", 2 -> &[1, 2, 3], &[4, b'a', b'b']);
mk_test!(two_4_3(two), 4, b"abc", 2, b"c", 1 -> &[1, 2, 3], &[4, b'a', b'b', b'c']);
mk_test!(two_4_4(two), 4, b"abc", 2, b"cd", 2 -> &[1, 2, 3], &[4, b'a', b'b', b'c', b'd']);
mk_test!(two_4_5(two), 4, &RND, 2, &RND[2..498], 496 -> &[1, 2, 3], [&[4], &RND[..498]].concat());
mk_test!(two_4_6(two), 4, &RND, 2, &RND[2..], 497 -> &[1, 2, 3], [&[4], &RND[..499]].concat());

mk_test!(two_5_0(two), 5, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_5_1(two), 5, b"a", 1 -> &[1, 2, 3], &[4, 5, b'a']);
mk_test!(two_5_2(two), 5, b"ab", 1, b"b", 1 -> &[1, 2, 3], &[4, 5, b'a', b'b']);
mk_test!(two_5_3(two), 5, b"ab", 1, b"bc", 2 -> &[1, 2, 3], &[4, 5, b'a', b'b', b'c']);
mk_test!(two_5_4(two), 5, &RND, 1, &RND[1..497], 496 -> &[1, 2, 3], [&[4, 5], &RND[..497]].concat());
mk_test!(two_5_5(two), 5, &RND, 1, &RND[1..], 497 -> &[1, 2, 3], [&[4, 5], &RND[..498]].concat());
mk_test!(two_5_6(two), 5, &RND, 1, &RND[1..], 497, &RND[498..514], 16 -> &[1, 2, 3], [&[4, 5], &RND[..498]].concat(), &RND[498..514]);

mk_test!(two_6_0(two), 6, &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_6_1(two), 6, b"a", 1 -> &[1, 2, 3], &[4, 5, 6, b'a']);
mk_test!(two_6_2(two), 6, b"ab", 2 -> &[1, 2, 3], &[4, 5, 6, b'a', b'b']);
mk_test!(two_6_3(two), 6, &RND[..496], 496 -> &[1, 2, 3], [&[4, 5, 6], &RND[..496]].concat());
mk_test!(two_6_4(two), 6, &RND, 497 -> &[1, 2, 3], [&[4, 5, 6], &RND[..497]].concat());
mk_test!(two_6_5(two), 6, &RND, 497, &RND[497..513], 16 -> &[1, 2, 3], [&[4, 5, 6], &RND[..497]].concat(), &RND[497..513]);

mk_test!(three_0_0(three), 0, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_0_1(three), 0, b"a", 1 -> &[b'a', 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_0_2(three), 0, b"ab", 2 -> &[b'a', b'b', 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_0_3(three), 0, b"abc", 3 -> b"abc", &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_0_4(three), 0, b"abcd", 3, b"d", 1 -> b"abc", &[b'd', 5, 6], &[7, 8, 9]);
mk_test!(three_0_5(three), 0, b"abcd", 3, b"de", 2 -> b"abc", &[b'd', b'e', 6], &[7, 8, 9]);
mk_test!(three_0_6(three), 0, b"abcd", 3, b"def", 3 -> b"abc", b"def", &[7, 8, 9]);
mk_test!(three_0_7(three), 0, b"abcd", 3, b"defg", 3, b"g", 1 -> b"abc", b"def", &[b'g', 8, 9]);
mk_test!(three_0_8(three), 0, b"abcd", 3, b"defg", 3, b"gh", 2 -> b"abc", b"def", &[b'g', b'h', 9]);
mk_test!(three_0_9(three), 0, b"abcd", 3, b"defg", 3, b"ghi", 3 -> b"abc", b"def", b"ghi");
mk_test!(three_0_10(three), 0, b"abcd", 3, b"defg", 3, b"ghij", 3, b"j", 1 -> b"abc", b"def", b"ghij");
mk_test!(three_0_11(three), 0, b"abcd", 3, b"defg", 3, b"ghij", 3, b"jk", 2 -> b"abc", b"def", b"ghijk");
mk_test!(three_0_12(three), 0, &RND, 3, &RND[3..], 3, &RND[6..], 3, &RND[9..505], 496 -> &RND[..3], &RND[3..6], &RND[6..505]);
mk_test!(three_0_13(three), 0, &RND, 3, &RND[3..], 3, &RND[6..], 3, &RND[9..], 497 -> &RND[..3], &RND[3..6], &RND[6..506]);
mk_test!(three_0_14(three), 0, &RND, 3, &RND[3..], 3, &RND[6..], 3, &RND[9..], 497, &RND[506..522], 16 -> &RND[..3], &RND[3..6], &RND[6..506], &RND[506..522]);

mk_test!(three_1_0(three), 1, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_1_1(three), 1, b"a", 1 -> &[1, b'a', 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_1_2(three), 1, b"ab", 2 -> &[1, b'a', b'b'], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_1_3(three), 1, b"abc", 2, b"c", 1 -> &[1, b'a', b'b'], &[b'c', 5, 6], &[7, 8, 9]);
mk_test!(three_1_4(three), 1, b"abc", 2, b"cd", 2 -> &[1, b'a', b'b'], &[b'c', b'd', 6], &[7, 8, 9]);
mk_test!(three_1_5(three), 1, b"abc", 2, b"cde", 3 -> &[1, b'a', b'b'], b"cde", &[7, 8, 9]);
mk_test!(three_1_6(three), 1, b"abc", 2, b"cdef", 3, b"f", 1 -> &[1, b'a', b'b'], b"cde", &[b'f', 8, 9]);
mk_test!(three_1_7(three), 1, b"abc", 2, b"cdef", 3, b"fg", 2 -> &[1, b'a', b'b'], b"cde", &[b'f', b'g', 9]);
mk_test!(three_1_8(three), 1, b"abc", 2, b"cdef", 3, b"fgh", 3 -> &[1, b'a', b'b'], b"cde", b"fgh");
mk_test!(three_1_9(three), 1, b"abc", 2, b"cdef", 3, b"fghi", 3, b"i", 1 -> &[1, b'a', b'b'], b"cde", b"fghi");
mk_test!(three_1_10(three), 1, b"abc", 2, b"cdef", 3, b"fghi", 3, b"ij", 2 -> &[1, b'a', b'b'], b"cde", b"fghij");
mk_test!(three_1_11(three), 1, &RND, 2, &RND[2..], 3, &RND[5..], 3, &RND[8..504], 496 -> [&[1], &RND[..2]].concat(), &RND[2..5], &RND[5..504]);
mk_test!(three_1_12(three), 1, &RND, 2, &RND[2..], 3, &RND[5..], 3, &RND[8..], 497 -> [&[1], &RND[..2]].concat(), &RND[2..5], &RND[5..505]);
mk_test!(three_1_13(three), 1, &RND, 2, &RND[2..], 3, &RND[5..], 3, &RND[8..], 497, &RND[505..521], 16 -> [&[1], &RND[..2]].concat(), &RND[2..5], &RND[5..505], &RND[505..521]);

mk_test!(three_2_0(three), 2, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_2_1(three), 2, b"a", 1 -> &[1, 2, b'a'], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_2_2(three), 2, b"ab", 1, b"b", 1 -> &[1, 2, b'a'], &[b'b', 5, 6], &[7, 8, 9]);
mk_test!(three_2_3(three), 2, b"ab", 1, b"bc", 2 -> &[1, 2, b'a'], &[b'b', b'c', 6], &[7, 8, 9]);
mk_test!(three_2_4(three), 2, b"ab", 1, b"bcd", 3 -> &[1, 2, b'a'], b"bcd", &[7, 8, 9]);
mk_test!(three_2_5(three), 2, b"ab", 1, b"bcde", 3, b"e", 1 -> &[1, 2, b'a'], b"bcd", &[b'e', 8, 9]);
mk_test!(three_2_6(three), 2, b"ab", 1, b"bcde", 3, b"ef", 2 -> &[1, 2, b'a'], b"bcd", &[b'e', b'f', 9]);
mk_test!(three_2_7(three), 2, b"ab", 1, b"bcde", 3, b"efg", 3 -> &[1, 2, b'a'], b"bcd", b"efg");
mk_test!(three_2_8(three), 2, b"ab", 1, b"bcde", 3, b"efgh", 3, b"h", 1 -> &[1, 2, b'a'], b"bcd", b"efgh");
mk_test!(three_2_9(three), 2, b"ab", 1, b"bcde", 3, b"efgh", 3, b"hi", 2 -> &[1, 2, b'a'], b"bcd", b"efghi");
mk_test!(three_2_10(three), 2, &RND, 1, &RND[1..], 3, &RND[4..], 3, &RND[7..503], 496 -> &[1, 2, RND[0]], &RND[1..4], &RND[4..503]);
mk_test!(three_2_11(three), 2, &RND, 1, &RND[1..], 3, &RND[4..], 3, &RND[7..], 497 -> &[1, 2, RND[0]], &RND[1..4], &RND[4..504]);
mk_test!(three_2_12(three), 2, &RND, 1, &RND[1..], 3, &RND[4..], 3, &RND[7..], 497, &RND[504..520], 16 -> &[1, 2, RND[0]], &RND[1..4], &RND[4..504], &RND[504..520]);

mk_test!(three_3_0(three), 3, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_3_1(three), 3, b"a", 1 -> &[1, 2, 3], &[b'a', 5, 6], &[7, 8, 9]);
mk_test!(three_3_2(three), 3, b"ab", 2 -> &[1, 2, 3], &[b'a', b'b', 6], &[7, 8, 9]);
mk_test!(three_3_3(three), 3, b"abc", 3 -> &[1, 2, 3], b"abc", &[7, 8, 9]);
mk_test!(three_3_4(three), 3, b"abcd", 3, b"d", 1 -> &[1, 2, 3], b"abc", &[b'd', 8, 9]);
mk_test!(three_3_5(three), 3, b"abcd", 3, b"de", 2 -> &[1, 2, 3], b"abc", &[b'd', b'e', 9]);
mk_test!(three_3_6(three), 3, b"abcd", 3, b"def", 3 -> &[1, 2, 3], b"abc", b"def");
mk_test!(three_3_7(three), 3, b"abcd", 3, b"defg", 3, b"g", 1 -> &[1, 2, 3], b"abc", b"defg");
mk_test!(three_3_8(three), 3, b"abcd", 3, b"defg", 3, b"gh", 2 -> &[1, 2, 3], b"abc", b"defgh");
mk_test!(three_3_9(three), 3, &RND, 3, &RND[3..], 3, &RND[6..502], 496 -> &[1, 2, 3], &RND[..3], &RND[3..502]);
mk_test!(three_3_10(three), 3, &RND, 3, &RND[3..], 3, &RND[6..], 497 -> &[1, 2, 3], &RND[..3], &RND[3..503]);
mk_test!(three_3_11(three), 3, &RND, 3, &RND[3..], 3, &RND[6..], 497, &RND[503..519], 16 -> &[1, 2, 3], &RND[..3], &RND[3..503], &RND[503..519]);

mk_test!(three_4_0(three), 4, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_4_1(three), 4, b"a", 1 -> &[1, 2, 3], &[4, b'a', 6], &[7, 8, 9]);
mk_test!(three_4_2(three), 4, b"ab", 2 -> &[1, 2, 3], &[4, b'a', b'b'], &[7, 8, 9]);
mk_test!(three_4_3(three), 4, b"abc", 2, b"c", 1 -> &[1, 2, 3], &[4, b'a', b'b'], &[b'c', 8, 9]);
mk_test!(three_4_4(three), 4, b"abc", 2, b"cd", 2 -> &[1, 2, 3], &[4, b'a', b'b'], &[b'c', b'd', 9]);
mk_test!(three_4_5(three), 4, b"abc", 2, b"cde", 3 -> &[1, 2, 3], &[4, b'a', b'b'], b"cde");
mk_test!(three_4_6(three), 4, b"abc", 2, b"cdef", 3, b"f", 1 -> &[1, 2, 3], &[4, b'a', b'b'], b"cdef");
mk_test!(three_4_7(three), 4, b"abc", 2, b"cdef", 3, b"fg", 2 -> &[1, 2, 3], &[4, b'a', b'b'], b"cdefg");
mk_test!(three_4_8(three), 4, &RND, 2, &RND[2..], 3, &RND[5..501], 496 -> &[1, 2, 3], [&[4], &RND[..2]].concat(), &RND[2..501]);
mk_test!(three_4_9(three), 4, &RND, 2, &RND[2..], 3, &RND[5..], 497 -> &[1, 2, 3], [&[4], &RND[..2]].concat(), &RND[2..502]);
mk_test!(three_4_10(three), 4, &RND, 2, &RND[2..], 3, &RND[5..], 497, &RND[502..518], 16 -> &[1, 2, 3], [&[4], &RND[..2]].concat(), &RND[2..502], &RND[502..518]);

mk_test!(three_5_0(three), 5, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_5_1(three), 5, b"a", 1 -> &[1, 2, 3], &[4, 5, b'a'], &[7, 8, 9]);
mk_test!(three_5_2(three), 5, b"ab", 1, b"b", 1 -> &[1, 2, 3], &[4, 5, b'a'], &[b'b', 8, 9]);
mk_test!(three_5_3(three), 5, b"ab", 1, b"bc", 2 -> &[1, 2, 3], &[4, 5, b'a'], &[b'b', b'c', 9]);
mk_test!(three_5_4(three), 5, b"ab", 1, b"bcd", 3 -> &[1, 2, 3], &[4, 5, b'a'], b"bcd");
mk_test!(three_5_5(three), 5, b"ab", 1, b"bcde", 3, b"e", 1 -> &[1, 2, 3], &[4, 5, b'a'], b"bcde");
mk_test!(three_5_6(three), 5, b"ab", 1, b"bcde", 3, b"ef", 2 -> &[1, 2, 3], &[4, 5, b'a'], b"bcdef");
mk_test!(three_5_7(three), 5, &RND, 1, &RND[1..], 3, &RND[4..500], 496 -> &[1, 2, 3], &[4, 5, RND[0]], &RND[1..500]);
mk_test!(three_5_8(three), 5, &RND, 1, &RND[1..], 3, &RND[4..], 497 -> &[1, 2, 3], &[4, 5, RND[0]], &RND[1..501]);
mk_test!(three_5_9(three), 5, &RND, 1, &RND[1..], 3, &RND[4..], 497, &RND[501..517], 16 -> &[1, 2, 3], &[4, 5, RND[0]], &RND[1..501], &RND[501..517]);

mk_test!(three_6_0(three), 6, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_6_1(three), 6, b"a", 1 -> &[1, 2, 3], &[4, 5, 6], &[b'a', 8, 9]);
mk_test!(three_6_2(three), 6, b"ab", 2 -> &[1, 2, 3], &[4, 5, 6], &[b'a', b'b', 9]);
mk_test!(three_6_3(three), 6, b"abc", 3 -> &[1, 2, 3], &[4, 5, 6], b"abc");
mk_test!(three_6_4(three), 6, b"abcd", 3, b"d", 1 -> &[1, 2, 3], &[4, 5, 6], b"abcd");
mk_test!(three_6_5(three), 6, b"abcd", 3, b"de", 2 -> &[1, 2, 3], &[4, 5, 6], b"abcde");
mk_test!(three_6_6(three), 6, &RND, 3, &RND[3..499], 496 -> &[1, 2, 3], &[4, 5, 6], &RND[..499]);
mk_test!(three_6_7(three), 6, &RND, 3, &RND[3..], 497 -> &[1, 2, 3], &[4, 5, 6], &RND[..500]);
mk_test!(three_6_8(three), 6, &RND, 3, &RND[3..], 497, &RND[500..516], 16 -> &[1, 2, 3], &[4, 5, 6], &RND[..500], &RND[500..516]);

mk_test!(three_7_0(three), 7, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_7_1(three), 7, b"a", 1 -> &[1, 2, 3], &[4, 5, 6], &[7, b'a', 9]);
mk_test!(three_7_2(three), 7, b"ab", 2 -> &[1, 2, 3], &[4, 5, 6], &[7, b'a', b'b']);
mk_test!(three_7_3(three), 7, b"abc", 2, b"c", 1 -> &[1, 2, 3], &[4, 5, 6], &[7, b'a', b'b', b'c']);
mk_test!(three_7_4(three), 7, b"abc", 2, b"cd", 2 -> &[1, 2, 3], &[4, 5, 6], &[7, b'a', b'b', b'c', b'd']);
mk_test!(three_7_5(three), 7, &RND, 2, &RND[2..498], 496 -> &[1, 2, 3], &[4, 5, 6], [&[7], &RND[..498]].concat());
mk_test!(three_7_6(three), 7, &RND, 2, &RND[2..], 497 -> &[1, 2, 3], &[4, 5, 6], [&[7], &RND[..499]].concat());
mk_test!(three_7_7(three), 7, &RND, 2, &RND[2..], 497, &RND[499..515], 16 -> &[1, 2, 3], &[4, 5, 6], [&[7], &RND[..499]].concat(), &RND[499..515]);

mk_test!(three_8_0(three), 8, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_8_1(three), 8, b"a", 1 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, b'a']);
mk_test!(three_8_2(three), 8, b"ab", 1, b"b", 1 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, b'a', b'b']);
mk_test!(three_8_3(three), 8, b"ab", 1, b"bc", 2 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, b'a', b'b', b'c']);
mk_test!(three_8_4(three), 8, &RND, 1, &RND[1..497], 496 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8], &RND[..497]].concat());
mk_test!(three_8_5(three), 8, &RND, 1, &RND[1..], 497 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8], &RND[..498]].concat());
mk_test!(three_8_6(three), 8, &RND, 1, &RND[1..], 497, &RND[498..514], 16 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8], &RND[..498]].concat(), &RND[498..514]);

mk_test!(three_9_0(three), 9, &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_9_1(three), 9, b"a", 1 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9, b'a']);
mk_test!(three_9_2(three), 9, b"ab", 2 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9, b'a', b'b']);
mk_test!(three_9_3(three), 9, &RND[..496], 496 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8, 9], &RND[..496]].concat());
mk_test!(three_9_4(three), 9, &RND, 497 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8, 9], &RND[..497]].concat());
mk_test!(three_9_5(three), 9, &RND, 497, &RND[497..513], 16 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8, 9], &RND[..497]].concat(), &RND[497..513]);

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
use crate::stream::OpenOptions;
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
    ($name:ident ($setup:ident), $($in:expr, $inlen:literal),+ -> $($out:expr),+) => {
        #[test]
        fn $name() {
            let mut inner = {
                let (container, id) = $setup();
                let mut stream = OpenOptions::new().append(true).open(container, id.clone()).unwrap();

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

mk_test!(one_0(one), &[], 0 -> &[1, 2, 3]);
mk_test!(one_1(one), b"a", 1 -> &[1, 2, 3, b'a']);
mk_test!(one_2(one), b"ab", 2 -> &[1, 2, 3, b'a', b'b']);
mk_test!(one_3(one), &RND[..496], 496 -> [&[1, 2, 3], &RND[..496]].concat());
mk_test!(one_4(one), &RND, 497 -> [&[1, 2, 3], &RND[..497]].concat());
mk_test!(one_5(one), &RND, 497, &RND[497..513], 16 -> [&[1, 2, 3], &RND[..497]].concat(), &RND[497..513]);

mk_test!(two_0(two), &[], 0 -> &[1, 2, 3], &[4, 5, 6]);
mk_test!(two_1(two), b"a", 1 -> &[1, 2, 3], &[4, 5, 6, b'a']);
mk_test!(two_2(two), b"ab", 2 -> &[1, 2, 3], &[4, 5, 6, b'a', b'b']);
mk_test!(two_3(two), &RND[..496], 496 -> &[1, 2, 3], [&[4, 5, 6], &RND[..496]].concat());
mk_test!(two_4(two), &RND, 497 -> &[1, 2, 3], [&[4, 5, 6], &RND[..497]].concat());
mk_test!(two_5(two), &RND, 497, &RND[497..513], 16 -> &[1, 2, 3], [&[4, 5, 6], &RND[..497]].concat(), &RND[497..513]);

mk_test!(three_0(three), &[], 0 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9]);
mk_test!(three_1(three), b"a", 1 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9, b'a']);
mk_test!(three_2(three), b"ab", 2 -> &[1, 2, 3], &[4, 5, 6], &[7, 8, 9, b'a', b'b']);
mk_test!(three_3(three), &RND[..496], 496 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8, 9], &RND[..496]].concat());
mk_test!(three_4(three), &RND, 497 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8, 9], &RND[..497]].concat());
mk_test!(three_5(three), &RND, 497, &RND[497..513], 16 -> &[1, 2, 3], &[4, 5, 6], [&[7, 8, 9], &RND[..497]].concat(), &RND[497..513]);

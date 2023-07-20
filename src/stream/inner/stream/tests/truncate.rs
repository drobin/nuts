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
use crate::stream::{Error, OpenOptions};
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
                let mut stream = OpenOptions::new().write(true).truncate(true).open(container, id.clone()).unwrap();

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
    for options in [
        OpenOptions::new().truncate(true),
        OpenOptions::new().truncate(true).write(false),
    ] {
        let (container, id) = setup_one();
        let mut stream = options.open(container, id).unwrap();

        let err = stream.write(b"abc").unwrap_err();
        assert_error!(err, Error::NotWritable);
    }
}

mk_test!(one_0(one), &[], 0 -> &[]);
mk_test!(one_1(one), b"a", 1 -> b"a");
mk_test!(one_2(one), b"ab", 2 -> b"ab");
mk_test!(one_3(one), b"abc", 3 -> b"abc");
mk_test!(one_4(one), b"abcd", 4 -> b"abcd");
mk_test!(one_5(one), b"abcde", 5 -> b"abcde");
mk_test!(one_6(one), &RND[..499], 499 -> &RND[..499]);
mk_test!(one_7(one), &RND, 500 -> &RND[..500]);
mk_test!(one_8(one), &RND, 500, &RND[500..516], 16 -> &RND[..500], &RND[500..516]);

mk_test!(two_0(two), &[], 0 -> &[]);
mk_test!(two_1(two), b"a", 1 -> b"a");
mk_test!(two_2(two), b"ab", 2 -> b"ab");
mk_test!(two_3(two), b"abc", 3 -> b"abc");
mk_test!(two_4(two), b"abcd", 4 -> b"abcd");
mk_test!(two_5(two), b"abcde", 5 -> b"abcde");
mk_test!(two_6(two), &RND[..499], 499 -> &RND[..499]);
mk_test!(two_7(two), &RND, 500 -> &RND[..500]);
mk_test!(two_8(two), &RND, 500, &RND[500..501], 1 -> &RND[..500], &RND[500..501]);
mk_test!(two_9(two), &RND, 500, &RND[500..502], 2 -> &RND[..500], &RND[500..502]);
mk_test!(two_10(two), &RND, 500, &RND[500..503], 3 -> &RND[..500], &RND[500..503]);
mk_test!(two_11(two), &RND, 500, &RND[500..999], 499 -> &RND[..500], &RND[500..999]);
mk_test!(two_12(two), &RND, 500, &RND[500..], 500 -> &RND[..500], &RND[500..1000]);
mk_test!(two_13(two), &RND, 500, &RND[500..], 500, &RND[1000..1016], 16 -> &RND[..500], &RND[500..1000], &RND[1000..1016]);

mk_test!(three_0(three), &[], 0 -> &[]);
mk_test!(three_1(three), b"a", 1 -> b"a");
mk_test!(three_2(three), b"ab", 2 -> b"ab");
mk_test!(three_3(three), b"abc", 3 -> b"abc");
mk_test!(three_4(three), &RND[..499], 499 -> &RND[..499]);
mk_test!(three_5(three), &RND, 500 -> &RND[..500]);
mk_test!(three_6(three), &RND, 500, &RND[500..501], 1 -> &RND[..500], &RND[500..501]);
mk_test!(three_7(three), &RND, 500, &RND[500..502], 2 -> &RND[..500], &RND[500..502]);
mk_test!(three_8(three), &RND, 500, &RND[500..503], 3 -> &RND[..500], &RND[500..503]);
mk_test!(three_9(three), &RND, 500, &RND[500..999], 499 -> &RND[..500], &RND[500..999]);
mk_test!(three_10(three), &RND, 500, &RND[500..], 500 -> &RND[..500], &RND[500..1000]);
mk_test!(three_11(three), &RND, 500, &RND[500..], 500, &RND[1000..1001], 1 -> &RND[..500], &RND[500..1000], &RND[1000..1001]);
mk_test!(three_12(three), &RND, 500, &RND[500..], 500, &RND[1000..1002], 2 -> &RND[..500], &RND[500..1000], &RND[1000..1002]);
mk_test!(three_13(three), &RND, 500, &RND[500..], 500, &RND[1000..1003], 3 -> &RND[..500], &RND[500..1000], &RND[1000..1003]);
mk_test!(three_14(three), &RND, 500, &RND[500..], 500, &RND[1000..1499], 499 -> &RND[..500], &RND[500..1000], &RND[1000..1499]);
mk_test!(three_15(three), &RND, 500, &RND[500..], 500, &RND[1000..], 500 -> &RND[..500], &RND[500..1000], &RND[1000..1500]);
mk_test!(three_16(three), &RND, 500, &RND[500..], 500, &RND[1000..], 500, &RND[1500..1516], 16 -> &RND[..500], &RND[500..1000], &RND[1000..1500], &RND[1500..1516]);

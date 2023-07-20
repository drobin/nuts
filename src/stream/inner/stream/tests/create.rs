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

use crate::stream::inner::Inner;
use crate::stream::{Error, OpenOptions, Position};
use crate::tests::RND;

use crate::stream::testutils::setup_container;

macro_rules! mk_write_test {
    ($name:ident, $($in:expr, $inlen:literal),+ -> $($out:expr),+) => {
        #[test]
        fn $name() {
            let mut inner = {
                let mut container = setup_container();
                let id = container.aquire().unwrap();
                let mut stream = OpenOptions::new().write(true).create(true).open(container, id).unwrap();

                $(
                    assert_eq!(stream.write($in).unwrap(), $inlen);
                )*
                stream.flush().unwrap();

                let first = stream.inner.first().clone();
                Inner::open(stream.inner.into_container(), first).unwrap()
            };

            $(
                inner.goto_next().unwrap().unwrap();
                assert_eq!(inner.payload().unwrap(), $out);
            )*

            assert!(inner.goto_next().is_none());
        }
    };
}

macro_rules! mk_read_write_test {
    ($name:ident, $($in:expr, $inlen:literal),+ -> $($out:expr),+) => {
        #[test]
        fn $name() {
            let mut container = setup_container();
            let id = container.aquire().unwrap();
            let mut stream = OpenOptions::new().read(true).write(true).create(true).open(container, id).unwrap();
            let mut buf = [0; 512];

            $(
                assert_eq!(stream.write($in).unwrap(), $inlen);
            )*
            stream.flush().unwrap();

            stream.seek(Position::Start(0)).unwrap();

            $(
                let len = stream.read(&mut buf).unwrap();
                assert_eq!(&buf[..len], $out);
            )*
        }
    };
}

#[test]
fn no_write() {
    for options in [
        OpenOptions::new().create(true),
        OpenOptions::new().create(true).write(false),
    ] {
        let mut inner = {
            let mut container = setup_container();
            let id = container.aquire().unwrap();
            let mut stream = options.open(container, id).unwrap();

            let err = stream.write(b"abc").unwrap_err();
            assert_error!(err, Error::NotWritable);

            let first = stream.inner.first.clone();
            Inner::open(stream.inner.into_container(), first).unwrap()
        };

        inner.goto_next().unwrap().unwrap();
        assert_eq!(inner.payload().unwrap(), []);

        assert!(inner.goto_next().is_none());
    }
}

#[test]
fn no_read() {
    for options in [
        OpenOptions::new().write(true).create(true),
        OpenOptions::new().write(true).create(true).read(false),
    ] {
        let mut container = setup_container();
        let id = container.aquire().unwrap();
        let mut stream = options.open(container, id).unwrap();
        let mut buf = [0; 512];

        assert_eq!(stream.write(b"abc").unwrap(), 3);
        stream.flush().unwrap();

        stream.seek(Position::Start(0)).unwrap();

        let err = stream.read(&mut buf).unwrap_err();
        assert_error!(err, Error::NotReadable);
    }
}

mk_write_test!(write_0, &[], 0 -> &[]);
mk_write_test!(write_1, b"a", 1 -> &[b'a']);
mk_write_test!(write_2, b"ab", 2 -> &[b'a', b'b']);
mk_write_test!(write_3, &RND[..499], 499 -> &RND[..499]);
mk_write_test!(write_4, &RND, 500 -> &RND[..500]);
mk_write_test!(write_5, &RND, 500, &RND[500..516], 16 -> &RND[..500], &RND[500..516]);

mk_read_write_test!(read_0, &[], 0 -> &[]);
mk_read_write_test!(read_1, b"a", 1 -> &[b'a']);
mk_read_write_test!(read_2, b"ab", 2 -> &[b'a', b'b']);
mk_read_write_test!(read_3, &RND[..499], 499 -> &RND[..499]);
mk_read_write_test!(read_4, &RND, 500 -> &RND[..500]);
mk_read_write_test!(read_5, &RND, 500, &RND[500..516], 16 -> &RND[..500], &RND[500..516]);

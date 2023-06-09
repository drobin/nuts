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

use crate::stream::Inner;

use crate::stream::testutils::{setup_one, setup_three};

#[test]
fn walk() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert!(stream.current().is_none());
    assert!(stream.payload().is_none());

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.current().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), &[1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.current().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), &[4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.current().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), &[7, 8, 9]);

    assert!(stream.goto_next().is_none());
}

#[test]
fn get() {
    let (container, id) = setup_one();
    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_first().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);
}

#[test]
fn set_empty() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_set(&[]).unwrap(), 0);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), []);
}

#[test]
fn set_smaller() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_set(&[4, 5]).unwrap(), 2);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [4, 5]);
}

#[test]
fn set_equal_size() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_set(&[4, 5, 6]).unwrap(), 3);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);
}

#[test]
fn set_bigger() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_set(&[4, 5, 6, 7]).unwrap(), 4);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);
}

#[test]
fn set_overflow() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_set(&[b'x'; 512]).unwrap(), 500);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [b'x'; 500]);
}

#[test]
fn add_none() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_add(&[]).unwrap(), 0);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);
}

#[test]
fn add_some() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_add(&[4, 5]).unwrap(), 2);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3, 4, 5]);
}

#[test]
fn add_full() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_add(&[b'x'; 497]).unwrap(), 497);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap()[..3], [1, 2, 3]);
    assert_eq!(stream.payload().unwrap()[3..], [b'x'; 497]);
}

#[test]
fn add_overflow() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_add(&[b'x'; 500]).unwrap(), 497);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap()[..3], [1, 2, 3]);
    assert_eq!(stream.payload().unwrap()[3..], [b'x'; 497]);
}

#[test]
fn add_is_full() {
    let (container, id) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_add(&[b'x'; 497]).unwrap(), 497);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let (container, id) = {
        let mut stream = Inner::open(container, id.clone()).unwrap();

        assert_eq!(stream.goto_first().unwrap(), &id);
        assert_eq!(stream.payload_add(&[4, 5]).unwrap(), 0);
        stream.flush().unwrap();

        (stream.into_container(), id)
    };

    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert_eq!(stream.payload().unwrap()[..3], [1, 2, 3]);
    assert_eq!(stream.payload().unwrap()[3..], [b'x'; 497]);
}

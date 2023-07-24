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

use crate::stream::testutils::{setup_one, setup_three, setup_two};

#[test]
fn front_one() {
    let (container, id1, id2) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();
        let new_first = stream.insert_front().unwrap().clone();

        stream.payload_set(&[4, 5, 6, 7]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new_first, id)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn front_two() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new_first = stream.insert_front().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new_first, id1, id2)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn front_three() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new_first = stream.insert_front().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new_first, id1, id2, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn back_one() {
    let (container, id1, id2) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();
        let new_last = stream.insert_back().unwrap().clone();

        stream.payload_set(&[4, 5, 6, 7]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id, new_last)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn back_two() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new_last = stream.insert_back().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, id2, new_last)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn back_three() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new_last = stream.insert_back().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, id2, id3, new_last)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_one() {
    let (container, id1, id2) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[4, 5, 6, 7]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_one_next() {
    let (container, id1, id2) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[4, 5, 6, 7]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_two() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id1, id2)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_two_next() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id1, id2)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_two_next_next() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, new, id2)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_three() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id1, id2, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_three_next() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id1, id2, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_three_next_next() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, new, id2, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn prev_three_next_next_next() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_prev().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, id2, new, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_one() {
    let (container, id1, id2) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[4, 5, 6, 7]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_one_next() {
    let (container, id1, id2) = {
        let (container, id) = setup_one();
        let mut stream = Inner::open(container, id.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[4, 5, 6, 7]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id, new)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6, 7]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_two() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id1, id2)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_two_next() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, new, id2)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_two_next_next() {
    let (container, id1, id2, id3) = {
        let (container, (id1, id2)) = setup_two();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[7, 8, 9, 10]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, id2, new)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9, 10]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_three() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), new, id1, id2, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_three_next() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, new, id2, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_three_next_next() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, id2, new, id3)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

#[test]
fn next_three_next_next_next() {
    let (container, id1, id2, id3, id4) = {
        let (container, (id1, id2, id3)) = setup_three();
        let mut stream = Inner::open(container, id1.clone()).unwrap();

        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        stream.goto_next().unwrap().unwrap();
        let new = stream.insert_next().unwrap().clone();

        stream.payload_set(&[10, 11, 12, 13]).unwrap();
        stream.flush().unwrap();

        (stream.into_container(), id1, id2, id3, new)
    };

    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id4);
    assert_eq!(stream.payload().unwrap(), [10, 11, 12, 13]);

    assert!(stream.goto_next().is_none());

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id3);
    assert_eq!(stream.payload().unwrap(), [7, 8, 9]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.payload().unwrap(), [4, 5, 6]);

    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert_eq!(stream.payload().unwrap(), [1, 2, 3]);

    assert!(stream.goto_prev().is_none());
}

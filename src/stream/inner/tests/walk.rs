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

use crate::stream::testutils::{setup_one, setup_three, setup_two};

#[test]
fn one_first() {
    let (container, id) = setup_one();
    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_first().unwrap(), &id);
    assert!(stream.goto_next().is_none());
    assert!(stream.goto_prev().is_none());
}

#[test]
fn one_last() {
    let (container, id) = setup_one();
    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_last().unwrap(), &id);
    assert!(stream.goto_next().is_none());
    assert!(stream.goto_prev().is_none());
}

#[test]
fn one_next() {
    let (container, id) = setup_one();
    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert!(stream.goto_next().is_none());
    assert!(stream.goto_prev().is_none());
}

#[test]
fn one_prev() {
    let (container, id) = setup_one();
    let mut stream = Inner::open(container, id.clone()).unwrap();

    assert!(stream.goto_prev().is_none());
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id);
    assert!(stream.goto_next().is_none());
    assert!(stream.goto_prev().is_none());
}

#[test]
fn two_first() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_first().unwrap(), &id1);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn two_last() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_last().unwrap(), &id2);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn two_next() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn two_prev() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert!(stream.goto_prev().is_none());
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn three_first() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_first().unwrap(), &id1);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn three_last() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_last().unwrap(), &id3);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn three_next() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

#[test]
fn three_prev() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Inner::open(container, id1.clone()).unwrap();

    assert!(stream.goto_prev().is_none());
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id1);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_next().unwrap().unwrap(), &id3);
    assert!(stream.goto_next().is_none());
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id2);
    assert_eq!(stream.goto_prev().unwrap().unwrap(), &id1);
    assert!(stream.goto_prev().is_none());
}

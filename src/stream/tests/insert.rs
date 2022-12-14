// MIT License
//
// Copyright (c) 2022 Robin Doer
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

use crate::stream::Stream;

use super::{
    assert_next_is_none, assert_prev_is_none, next, prev, setup_container, setup_one, setup_three,
    setup_two,
};

#[test]
fn empty() {
    let mut container = setup_container();

    let id_x = {
        let mut stream = Stream::create(&mut container);

        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_prev_is_none!(stream);
}

#[test]
fn one_0() {
    let mut container = setup_container();
    let id = setup_one(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id);

        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id_x);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id_x);
    assert_prev_is_none!(stream);
}

#[test]
fn one_1() {
    let mut container = setup_container();
    let id = setup_one(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(next!(stream), &id);
        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id);
    assert_eq!(next!(stream), &id);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id);
    assert_prev_is_none!(stream);
}

#[test]
fn two_0() {
    let mut container = setup_container();
    let (id1, id2) = setup_two(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id1);
        assert_eq!(prev!(stream), &id_x);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id1);
    assert_eq!(prev!(stream), &id_x);
    assert_prev_is_none!(stream);
}

#[test]
fn two_1() {
    let mut container = setup_container();
    let (id1, id2) = setup_two(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id2);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id_x);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id2);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id_x);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[test]
fn two_2() {
    let mut container = setup_container();
    let (id1, id2) = setup_two(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[test]
fn three_0() {
    let mut container = setup_container();
    let (id1, id2, id3) = setup_three(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        assert_eq!(next!(stream), &id3);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_eq!(prev!(stream), &id_x);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id_x);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id3);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_eq!(prev!(stream), &id_x);
    assert_prev_is_none!(stream);
}

#[test]
fn three_1() {
    let mut container = setup_container();
    let (id1, id2, id3) = setup_three(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id2);
        assert_eq!(next!(stream), &id3);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id_x);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id3);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id_x);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[test]
fn three_2() {
    let mut container = setup_container();
    let (id1, id2, id3) = setup_three(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        let id_x = stream.insert(None).unwrap().clone();
        assert_eq!(next!(stream), &id3);
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id_x);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id_x);
    assert_eq!(next!(stream), &id3);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id_x);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

#[test]
fn three_3() {
    let mut container = setup_container();
    let (id1, id2, id3) = setup_three(&mut container);

    let id_x = {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(next!(stream), &id1);
        assert_eq!(next!(stream), &id2);
        assert_eq!(next!(stream), &id3);
        let id_x = stream.insert(None).unwrap().clone();
        assert_next_is_none!(stream);
        assert_eq!(prev!(stream), &id3);
        assert_eq!(prev!(stream), &id2);
        assert_eq!(prev!(stream), &id1);
        assert_prev_is_none!(stream);

        id_x
    };

    let mut stream = Stream::new(&mut container, &id1);
    assert_eq!(next!(stream), &id1);
    assert_eq!(next!(stream), &id2);
    assert_eq!(next!(stream), &id3);
    assert_eq!(next!(stream), &id_x);
    assert_next_is_none!(stream);
    assert_eq!(prev!(stream), &id3);
    assert_eq!(prev!(stream), &id2);
    assert_eq!(prev!(stream), &id1);
    assert_prev_is_none!(stream);
}

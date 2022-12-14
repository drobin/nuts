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

use super::{assert_next_is_none, next, setup_container, setup_one, setup_three, setup_two};

#[test]
fn empty() {
    let mut container = setup_container();
    let stream = Stream::create(&mut container);

    assert!(stream.current_id().is_none());
    assert!(stream.current_payload().is_none());
}

#[test]
fn one() {
    let mut container = setup_container();
    let id = setup_one(&mut container);
    let mut stream = Stream::new(&mut container, &id);

    assert!(stream.current_id().is_none());
    assert!(stream.current_payload().is_none());

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_id().unwrap(), &id);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);

    assert_next_is_none!(stream);
}

#[test]
fn two() {
    let mut container = setup_container();
    let (id1, id2) = setup_two(&mut container);
    let mut stream = Stream::new(&mut container, &id1);

    assert!(stream.current_id().is_none());
    assert!(stream.current_payload().is_none());

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_id().unwrap(), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);

    assert_eq!(next!(stream), &id2);
    assert_eq!(stream.current_id().unwrap(), &id2);
    assert_eq!(stream.current_payload().unwrap(), &[4, 5, 6]);

    assert_next_is_none!(stream);
}

#[test]
fn three() {
    let mut container = setup_container();
    let (id1, id2, id3) = setup_three(&mut container);
    let mut stream = Stream::new(&mut container, &id1);

    assert!(stream.current_id().is_none());
    assert!(stream.current_payload().is_none());

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_id().unwrap(), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);

    assert_eq!(next!(stream), &id2);
    assert_eq!(stream.current_id().unwrap(), &id2);
    assert_eq!(stream.current_payload().unwrap(), &[4, 5, 6]);

    assert_eq!(next!(stream), &id3);
    assert_eq!(stream.current_id().unwrap(), &id3);
    assert_eq!(stream.current_payload().unwrap(), &[7, 8, 9]);

    assert_next_is_none!(stream);
}

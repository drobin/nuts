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

use crate::stream::{Position, Stream};

use crate::stream::testutils::{setup_one, setup_three, setup_two};

#[test]
fn start_one() {
    let (container, id) = setup_one();
    let mut stream = Stream::open(container, id.clone()).unwrap();

    for (offs, nread, rbuf) in [
        (0, 3, [1, 2, 3]),
        (1, 2, [2, 3, 0]),
        (2, 1, [3, 0, 0]),
        (3, 0, [0, 0, 0]),
        (4, 0, [0, 0, 0]),
    ] {
        let mut buf = [0; 3];

        stream.seek(Position::Start(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), &id);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn start_two() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Stream::open(container, id1.clone()).unwrap();

    for (offs, cur, nread, rbuf) in [
        (0, &id1, 3, [1, 2, 3]),
        (1, &id1, 2, [2, 3, 0]),
        (2, &id1, 1, [3, 0, 0]),
        (3, &id1, 3, [4, 5, 6]),
        (4, &id2, 2, [5, 6, 0]),
        (5, &id2, 1, [6, 0, 0]),
        (6, &id2, 0, [0, 0, 0]),
        (7, &id2, 0, [0, 0, 0]),
    ] {
        let mut buf = [0; 3];

        stream.seek(Position::Start(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), cur);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn start_three() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Stream::open(container, id1.clone()).unwrap();

    for (offs, cur, nread, rbuf) in [
        (0, &id1, 3, [1, 2, 3]),
        (1, &id1, 2, [2, 3, 0]),
        (2, &id1, 1, [3, 0, 0]),
        (3, &id1, 3, [4, 5, 6]),
        (4, &id2, 2, [5, 6, 0]),
        (5, &id2, 1, [6, 0, 0]),
        (6, &id2, 3, [7, 8, 9]),
        (7, &id3, 2, [8, 9, 0]),
        (8, &id3, 1, [9, 0, 0]),
        (9, &id3, 0, [0, 0, 0]),
        (10, &id3, 0, [0, 0, 0]),
    ] {
        let mut buf = [0; 3];

        stream.seek(Position::Start(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), cur);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn end_one() {
    let (container, id) = setup_one();
    let mut stream = Stream::open(container, id.clone()).unwrap();

    for (offs, nread, rbuf) in [
        (1, 0, [0, 0, 0]),
        (0, 0, [0, 0, 0]),
        (-1, 1, [3, 0, 0]),
        (-2, 2, [2, 3, 0]),
        (-3, 3, [1, 2, 3]),
    ] {
        let mut buf = [0; 3];

        stream.seek(Position::End(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), &id);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn end_two() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Stream::open(container, id1.clone()).unwrap();

    for (offs, cur, nread, rbuf) in [
        (1, &id2, 0, [0, 0, 0]),
        (0, &id2, 0, [0, 0, 0]),
        (-1, &id2, 1, [6, 0, 0]),
        (-2, &id2, 2, [5, 6, 0]),
        (-3, &id2, 3, [4, 5, 6]),
        (-4, &id1, 1, [3, 0, 0]),
        (-5, &id1, 2, [2, 3, 0]),
        (-6, &id1, 3, [1, 2, 3]),
        (-7, &id1, 3, [1, 2, 3]),
    ] {
        let mut buf = [0; 3];

        stream.seek(Position::End(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), cur);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn end_three() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Stream::open(container, id1.clone()).unwrap();

    for (offs, cur, nread, rbuf) in [
        (1, &id3, 0, [0, 0, 0]),
        (0, &id3, 0, [0, 0, 0]),
        (-1, &id3, 1, [9, 0, 0]),
        (-2, &id3, 2, [8, 9, 0]),
        (-3, &id3, 3, [7, 8, 9]),
        (-4, &id2, 1, [6, 0, 0]),
        (-5, &id2, 2, [5, 6, 0]),
        (-6, &id2, 3, [4, 5, 6]),
        (-7, &id1, 1, [3, 0, 0]),
        (-8, &id1, 2, [2, 3, 0]),
        (-9, &id1, 3, [1, 2, 3]),
        (-10, &id1, 3, [1, 2, 3]),
    ] {
        let mut buf = [0; 3];

        stream.seek(Position::End(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), cur);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn current_one() {
    let (container, id) = setup_one();
    let mut stream = Stream::open(container, id.clone()).unwrap();

    stream.inner.goto_first().unwrap();

    for (coffs, offs, nread, rbuf) in [
        (0, -1, 3, [1, 2, 3]),
        (0, 0, 3, [1, 2, 3]),
        (0, 1, 2, [2, 3, 0]),
        (0, 2, 1, [3, 0, 0]),
        (0, 3, 0, [0, 0, 0]),
        (0, 4, 0, [0, 0, 0]),
        (1, -2, 3, [1, 2, 3]),
        (1, -1, 3, [1, 2, 3]),
        (1, 0, 2, [2, 3, 0]),
        (1, 1, 1, [3, 0, 0]),
        (1, 2, 0, [0, 0, 0]),
        (1, 3, 0, [0, 0, 0]),
        (2, -3, 3, [1, 2, 3]),
        (2, -2, 3, [1, 2, 3]),
        (2, -1, 2, [2, 3, 0]),
        (2, 0, 1, [3, 0, 0]),
        (2, 1, 0, [0, 0, 0]),
        (2, 2, 0, [0, 0, 0]),
        (3, -4, 3, [1, 2, 3]),
        (3, -3, 3, [1, 2, 3]),
        (3, -2, 2, [2, 3, 0]),
        (3, -1, 1, [3, 0, 0]),
        (3, 0, 0, [0, 0, 0]),
        (3, 1, 0, [0, 0, 0]),
    ] {
        let mut buf = [0; 3];

        stream.inner.cur.as_mut().unwrap().offs = coffs;
        stream.seek(Position::Current(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), &id);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn current_two() {
    let (container, (id1, id2)) = setup_two();
    let mut stream = Stream::open(container, id1.clone()).unwrap();

    for (next, coffs, offs, cur, nread, rbuf) in [
        (0, 0, -1, &id1, 3, [1, 2, 3]),
        (0, 0, 0, &id1, 3, [1, 2, 3]),
        (0, 0, 1, &id1, 2, [2, 3, 0]),
        (0, 0, 2, &id1, 1, [3, 0, 0]),
        (0, 0, 3, &id1, 3, [4, 5, 6]),
        (0, 0, 4, &id2, 2, [5, 6, 0]),
        (0, 0, 5, &id2, 1, [6, 0, 0]),
        (0, 0, 6, &id2, 0, [0, 0, 0]),
        (0, 0, 7, &id2, 0, [0, 0, 0]),
        (0, 1, -2, &id1, 3, [1, 2, 3]),
        (0, 1, -1, &id1, 3, [1, 2, 3]),
        (0, 1, 0, &id1, 2, [2, 3, 0]),
        (0, 1, 1, &id1, 1, [3, 0, 0]),
        (0, 1, 2, &id1, 3, [4, 5, 6]),
        (0, 1, 3, &id2, 2, [5, 6, 0]),
        (0, 1, 4, &id2, 1, [6, 0, 0]),
        (0, 1, 5, &id2, 0, [0, 0, 0]),
        (0, 1, 6, &id2, 0, [0, 0, 0]),
        (0, 2, -3, &id1, 3, [1, 2, 3]),
        (0, 2, -2, &id1, 3, [1, 2, 3]),
        (0, 2, -1, &id1, 2, [2, 3, 0]),
        (0, 2, 0, &id1, 1, [3, 0, 0]),
        (0, 2, 1, &id1, 3, [4, 5, 6]),
        (0, 2, 2, &id2, 2, [5, 6, 0]),
        (0, 2, 3, &id2, 1, [6, 0, 0]),
        (0, 2, 4, &id2, 0, [0, 0, 0]),
        (0, 2, 5, &id2, 0, [0, 0, 0]),
        (0, 3, -4, &id1, 3, [1, 2, 3]),
        (0, 3, -3, &id1, 3, [1, 2, 3]),
        (0, 3, -2, &id1, 2, [2, 3, 0]),
        (0, 3, -1, &id1, 1, [3, 0, 0]),
        (0, 3, 0, &id1, 3, [4, 5, 6]),
        (0, 3, 1, &id2, 2, [5, 6, 0]),
        (0, 3, 2, &id2, 1, [6, 0, 0]),
        (0, 3, 3, &id2, 0, [0, 0, 0]),
        (0, 3, 4, &id2, 0, [0, 0, 0]),
        (1, 0, -4, &id1, 3, [1, 2, 3]),
        (1, 0, -3, &id1, 3, [1, 2, 3]),
        (1, 0, -2, &id1, 2, [2, 3, 0]),
        (1, 0, -1, &id1, 1, [3, 0, 0]),
        (1, 0, 0, &id2, 3, [4, 5, 6]),
        (1, 0, 1, &id2, 2, [5, 6, 0]),
        (1, 0, 2, &id2, 1, [6, 0, 0]),
        (1, 0, 3, &id2, 0, [0, 0, 0]),
        (1, 0, 4, &id2, 0, [0, 0, 0]),
        (1, 1, -5, &id1, 3, [1, 2, 3]),
        (1, 1, -4, &id1, 3, [1, 2, 3]),
        (1, 1, -3, &id1, 2, [2, 3, 0]),
        (1, 1, -2, &id1, 1, [3, 0, 0]),
        (1, 1, -1, &id2, 3, [4, 5, 6]),
        (1, 1, 0, &id2, 2, [5, 6, 0]),
        (1, 1, 1, &id2, 1, [6, 0, 0]),
        (1, 1, 2, &id2, 0, [0, 0, 0]),
        (1, 1, 3, &id2, 0, [0, 0, 0]),
        (1, 2, -6, &id1, 3, [1, 2, 3]),
        (1, 2, -5, &id1, 3, [1, 2, 3]),
        (1, 2, -4, &id1, 2, [2, 3, 0]),
        (1, 2, -3, &id1, 1, [3, 0, 0]),
        (1, 2, -2, &id2, 3, [4, 5, 6]),
        (1, 2, -1, &id2, 2, [5, 6, 0]),
        (1, 2, 0, &id2, 1, [6, 0, 0]),
        (1, 2, 1, &id2, 0, [0, 0, 0]),
        (1, 2, 2, &id2, 0, [0, 0, 0]),
        (1, 3, -7, &id1, 3, [1, 2, 3]),
        (1, 3, -6, &id1, 3, [1, 2, 3]),
        (1, 3, -5, &id1, 2, [2, 3, 0]),
        (1, 3, -4, &id1, 1, [3, 0, 0]),
        (1, 3, -3, &id2, 3, [4, 5, 6]),
        (1, 3, -2, &id2, 2, [5, 6, 0]),
        (1, 3, -1, &id2, 1, [6, 0, 0]),
        (1, 3, 0, &id2, 0, [0, 0, 0]),
        (1, 3, 1, &id2, 0, [0, 0, 0]),
    ] {
        stream.inner.goto_first().unwrap();

        for _ in 0..next {
            stream.inner.goto_next().unwrap().unwrap();
        }

        let mut buf = [0; 3];

        stream.inner.cur.as_mut().unwrap().offs = coffs;
        stream.seek(Position::Current(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), cur);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

#[test]
fn current_three() {
    let (container, (id1, id2, id3)) = setup_three();
    let mut stream = Stream::open(container, id1.clone()).unwrap();

    for (next, coffs, offs, cur, nread, rbuf) in [
        (0, 0, -1, &id1, 3, [1, 2, 3]),
        (0, 0, 0, &id1, 3, [1, 2, 3]),
        (0, 0, 1, &id1, 2, [2, 3, 0]),
        (0, 0, 2, &id1, 1, [3, 0, 0]),
        (0, 0, 3, &id1, 3, [4, 5, 6]),
        (0, 0, 4, &id2, 2, [5, 6, 0]),
        (0, 0, 5, &id2, 1, [6, 0, 0]),
        (0, 0, 6, &id2, 3, [7, 8, 9]),
        (0, 0, 7, &id3, 2, [8, 9, 0]),
        (0, 0, 8, &id3, 1, [9, 0, 0]),
        (0, 0, 9, &id3, 0, [0, 0, 0]),
        (0, 0, 10, &id3, 0, [0, 0, 0]),
        (0, 1, -2, &id1, 3, [1, 2, 3]),
        (0, 1, -1, &id1, 3, [1, 2, 3]),
        (0, 1, 0, &id1, 2, [2, 3, 0]),
        (0, 1, 1, &id1, 1, [3, 0, 0]),
        (0, 1, 2, &id1, 3, [4, 5, 6]),
        (0, 1, 3, &id2, 2, [5, 6, 0]),
        (0, 1, 4, &id2, 1, [6, 0, 0]),
        (0, 1, 5, &id2, 3, [7, 8, 9]),
        (0, 1, 6, &id3, 2, [8, 9, 0]),
        (0, 1, 7, &id3, 1, [9, 0, 0]),
        (0, 1, 8, &id3, 0, [0, 0, 0]),
        (0, 1, 9, &id3, 0, [0, 0, 0]),
        (0, 2, -3, &id1, 3, [1, 2, 3]),
        (0, 2, -2, &id1, 3, [1, 2, 3]),
        (0, 2, -1, &id1, 2, [2, 3, 0]),
        (0, 2, 0, &id1, 1, [3, 0, 0]),
        (0, 2, 1, &id1, 3, [4, 5, 6]),
        (0, 2, 2, &id2, 2, [5, 6, 0]),
        (0, 2, 3, &id2, 1, [6, 0, 0]),
        (0, 2, 4, &id2, 3, [7, 8, 9]),
        (0, 2, 5, &id3, 2, [8, 9, 0]),
        (0, 2, 6, &id3, 1, [9, 0, 0]),
        (0, 2, 7, &id3, 0, [0, 0, 0]),
        (0, 2, 8, &id3, 0, [0, 0, 0]),
        (0, 3, -4, &id1, 3, [1, 2, 3]),
        (0, 3, -3, &id1, 3, [1, 2, 3]),
        (0, 3, -2, &id1, 2, [2, 3, 0]),
        (0, 3, -1, &id1, 1, [3, 0, 0]),
        (0, 3, 0, &id1, 3, [4, 5, 6]),
        (0, 3, 1, &id2, 2, [5, 6, 0]),
        (0, 3, 2, &id2, 1, [6, 0, 0]),
        (0, 3, 3, &id2, 3, [7, 8, 9]),
        (0, 3, 4, &id3, 2, [8, 9, 0]),
        (0, 3, 5, &id3, 1, [9, 0, 0]),
        (0, 3, 6, &id3, 0, [0, 0, 0]),
        (0, 3, 6, &id3, 0, [0, 0, 0]),
        (1, 0, -4, &id1, 3, [1, 2, 3]),
        (1, 0, -3, &id1, 3, [1, 2, 3]),
        (1, 0, -2, &id1, 2, [2, 3, 0]),
        (1, 0, -1, &id1, 1, [3, 0, 0]),
        (1, 0, 0, &id2, 3, [4, 5, 6]),
        (1, 0, 1, &id2, 2, [5, 6, 0]),
        (1, 0, 2, &id2, 1, [6, 0, 0]),
        (1, 0, 3, &id2, 3, [7, 8, 9]),
        (1, 0, 4, &id3, 2, [8, 9, 0]),
        (1, 0, 5, &id3, 1, [9, 0, 0]),
        (1, 0, 6, &id3, 0, [0, 0, 0]),
        (1, 0, 7, &id3, 0, [0, 0, 0]),
        (1, 1, -5, &id1, 3, [1, 2, 3]),
        (1, 1, -4, &id1, 3, [1, 2, 3]),
        (1, 1, -3, &id1, 2, [2, 3, 0]),
        (1, 1, -2, &id1, 1, [3, 0, 0]),
        (1, 1, -1, &id2, 3, [4, 5, 6]),
        (1, 1, 0, &id2, 2, [5, 6, 0]),
        (1, 1, 1, &id2, 1, [6, 0, 0]),
        (1, 1, 2, &id2, 3, [7, 8, 9]),
        (1, 1, 3, &id3, 2, [8, 9, 0]),
        (1, 1, 4, &id3, 1, [9, 0, 0]),
        (1, 1, 5, &id3, 0, [0, 0, 0]),
        (1, 1, 6, &id3, 0, [0, 0, 0]),
        (1, 2, -6, &id1, 3, [1, 2, 3]),
        (1, 2, -5, &id1, 3, [1, 2, 3]),
        (1, 2, -4, &id1, 2, [2, 3, 0]),
        (1, 2, -3, &id1, 1, [3, 0, 0]),
        (1, 2, -2, &id2, 3, [4, 5, 6]),
        (1, 2, -1, &id2, 2, [5, 6, 0]),
        (1, 2, 0, &id2, 1, [6, 0, 0]),
        (1, 2, 1, &id2, 3, [7, 8, 9]),
        (1, 2, 2, &id3, 2, [8, 9, 0]),
        (1, 2, 3, &id3, 1, [9, 0, 0]),
        (1, 2, 4, &id3, 0, [0, 0, 0]),
        (1, 2, 5, &id3, 0, [0, 0, 0]),
        (1, 3, -7, &id1, 3, [1, 2, 3]),
        (1, 3, -6, &id1, 3, [1, 2, 3]),
        (1, 3, -5, &id1, 2, [2, 3, 0]),
        (1, 3, -4, &id1, 1, [3, 0, 0]),
        (1, 3, -3, &id2, 3, [4, 5, 6]),
        (1, 3, -2, &id2, 2, [5, 6, 0]),
        (1, 3, -1, &id2, 1, [6, 0, 0]),
        (1, 3, 0, &id2, 3, [7, 8, 9]),
        (1, 3, 1, &id3, 2, [8, 9, 0]),
        (1, 3, 2, &id3, 1, [9, 0, 0]),
        (1, 3, 3, &id3, 0, [0, 0, 0]),
        (1, 3, 4, &id3, 0, [0, 0, 0]),
        (2, 0, -7, &id1, 3, [1, 2, 3]),
        (2, 0, -6, &id1, 3, [1, 2, 3]),
        (2, 0, -5, &id1, 2, [2, 3, 0]),
        (2, 0, -4, &id1, 1, [3, 0, 0]),
        (2, 0, -3, &id2, 3, [4, 5, 6]),
        (2, 0, -2, &id2, 2, [5, 6, 0]),
        (2, 0, -1, &id2, 1, [6, 0, 0]),
        (2, 0, 0, &id3, 3, [7, 8, 9]),
        (2, 0, 1, &id3, 2, [8, 9, 0]),
        (2, 0, 2, &id3, 1, [9, 0, 0]),
        (2, 0, 3, &id3, 0, [0, 0, 0]),
        (2, 0, 4, &id3, 0, [0, 0, 0]),
        (2, 1, -8, &id1, 3, [1, 2, 3]),
        (2, 1, -7, &id1, 3, [1, 2, 3]),
        (2, 1, -6, &id1, 2, [2, 3, 0]),
        (2, 1, -5, &id1, 1, [3, 0, 0]),
        (2, 1, -4, &id2, 3, [4, 5, 6]),
        (2, 1, -3, &id2, 2, [5, 6, 0]),
        (2, 1, -2, &id2, 1, [6, 0, 0]),
        (2, 1, -1, &id3, 3, [7, 8, 9]),
        (2, 1, 0, &id3, 2, [8, 9, 0]),
        (2, 1, 1, &id3, 1, [9, 0, 0]),
        (2, 1, 2, &id3, 0, [0, 0, 0]),
        (2, 1, 3, &id3, 0, [0, 0, 0]),
        (2, 2, -9, &id1, 3, [1, 2, 3]),
        (2, 2, -8, &id1, 3, [1, 2, 3]),
        (2, 2, -7, &id1, 2, [2, 3, 0]),
        (2, 2, -6, &id1, 1, [3, 0, 0]),
        (2, 2, -5, &id2, 3, [4, 5, 6]),
        (2, 2, -4, &id2, 2, [5, 6, 0]),
        (2, 2, -3, &id2, 1, [6, 0, 0]),
        (2, 2, -2, &id3, 3, [7, 8, 9]),
        (2, 2, -1, &id3, 2, [8, 9, 0]),
        (2, 2, 0, &id3, 1, [9, 0, 0]),
        (2, 2, 1, &id3, 0, [0, 0, 0]),
        (2, 2, 2, &id3, 0, [0, 0, 0]),
        (2, 3, -10, &id1, 3, [1, 2, 3]),
        (2, 3, -9, &id1, 3, [1, 2, 3]),
        (2, 3, -8, &id1, 2, [2, 3, 0]),
        (2, 3, -7, &id1, 1, [3, 0, 0]),
        (2, 3, -6, &id2, 3, [4, 5, 6]),
        (2, 3, -5, &id2, 2, [5, 6, 0]),
        (2, 3, -4, &id2, 1, [6, 0, 0]),
        (2, 3, -3, &id3, 3, [7, 8, 9]),
        (2, 3, -2, &id3, 2, [8, 9, 0]),
        (2, 3, -1, &id3, 1, [9, 0, 0]),
        (2, 3, 0, &id3, 0, [0, 0, 0]),
        (2, 3, 1, &id3, 0, [0, 0, 0]),
    ] {
        stream.inner.goto_first().unwrap();

        for _ in 0..next {
            stream.inner.goto_next().unwrap().unwrap();
        }

        let mut buf = [0; 3];

        stream.inner.cur.as_mut().unwrap().offs = coffs;
        stream.seek(Position::Current(offs)).unwrap();
        assert_eq!(stream.current().unwrap(), cur);
        assert_eq!(stream.read(&mut buf).unwrap(), nread);
        assert_eq!(buf, rbuf);
    }
}

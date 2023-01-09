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

use std::io::Read;
use testx::testx;

use crate::container::Container;
use crate::memory::{MemId, MemoryBackend};
use crate::stream::Stream;

#[testx(setup = super::setup_container)]
fn empty(mut container: Container<MemoryBackend>) {
    let mut stream = Stream::create(&mut container);
    let mut buf = [0; 64];

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_one)]
fn one_full(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_one)]
fn one_more(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);
    let mut buf = [0; 4];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_one)]
fn one_part(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);
    let mut buf = [0; 2];

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [3, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_two)]
fn two_full(mut container: Container<MemoryBackend>, (id1, _): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_two)]
fn two_more(mut container: Container<MemoryBackend>, (id1, _): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);
    let mut buf = [0; 4];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_two)]
fn two_part(mut container: Container<MemoryBackend>, (id1, _): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);
    let mut buf = [0; 2];

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [3, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [4, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [6, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_three)]
fn three_full(mut container: Container<MemoryBackend>, (id1, _, _): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);
    let mut buf = [0; 3];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [7, 8, 9]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_three)]
fn three_more(mut container: Container<MemoryBackend>, (id1, _, _): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);
    let mut buf = [0; 4];

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [1, 2, 3, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [4, 5, 6, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 3);
    assert_eq!(buf, [7, 8, 9, 0]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

#[testx(setup = super::setup_three)]
fn three_part(mut container: Container<MemoryBackend>, (id1, _, _): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);
    let mut buf = [0; 2];

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [1, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [3, 2]);

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [4, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [6, 5]);

    assert_eq!(stream.read(&mut buf).unwrap(), 2);
    assert_eq!(buf, [7, 8]);

    assert_eq!(stream.read(&mut buf).unwrap(), 1);
    assert_eq!(buf, [9, 8]);

    assert_eq!(stream.read(&mut buf).unwrap(), 0);
}

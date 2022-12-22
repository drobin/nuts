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

use testx::testx;

use crate::container::Container;
use crate::memory::{MemId, MemoryBackend};
use crate::stream::Stream;

#[testx(setup = super::setup_container)]
fn create(mut container: Container<MemoryBackend>) {
    let mut stream = Stream::create(&mut container);

    assert!(stream.current_id().is_none());
    assert!(stream.first_block().is_none());
    assert!(stream.last_block().is_none());
    assert!(stream.next_block().is_none());
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_one)]
fn one_first(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(stream.first_block().unwrap().unwrap(), &id);
    assert!(stream.next_block().is_none());
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_one)]
fn one_last(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(stream.last_block().unwrap().unwrap(), &id);
    assert!(stream.next_block().is_none());
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_one)]
fn one_next(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(stream.next_block().unwrap().unwrap(), &id);
    assert!(stream.next_block().is_none());
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_one)]
fn one_prev(mut container: Container<MemoryBackend>, id: MemId) {
    let mut stream = Stream::new(&mut container, &id);

    assert!(stream.prev_block().is_none());
    assert_eq!(stream.next_block().unwrap().unwrap(), &id);
    assert!(stream.next_block().is_none());
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_two)]
fn two_first(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(stream.first_block().unwrap().unwrap(), &id1);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id2);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_two)]
fn two_last(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(stream.last_block().unwrap().unwrap(), &id2);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_two)]
fn two_next(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(stream.next_block().unwrap().unwrap(), &id1);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id2);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_two)]
fn two_prev(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert!(stream.prev_block().is_none());
    assert_eq!(stream.next_block().unwrap().unwrap(), &id1);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id2);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_three)]
fn three_first(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(stream.first_block().unwrap().unwrap(), &id1);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id3);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_three)]
fn three_last(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(stream.last_block().unwrap().unwrap(), &id3);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_three)]
fn three_next(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(stream.next_block().unwrap().unwrap(), &id1);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id3);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

#[testx(setup = super::setup_three)]
fn three_prev(mut container: Container<MemoryBackend>, (id1, id2, id3): (MemId, MemId, MemId)) {
    let mut stream = Stream::new(&mut container, &id1);

    assert!(stream.prev_block().is_none());
    assert_eq!(stream.next_block().unwrap().unwrap(), &id1);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.next_block().unwrap().unwrap(), &id3);
    assert!(stream.next_block().is_none());
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id2);
    assert_eq!(stream.prev_block().unwrap().unwrap(), &id1);
    assert!(stream.prev_block().is_none());
}

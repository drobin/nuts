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

use nuts_bytes::Writer;

use crate::backend::{Backend, BlockId};
use crate::container::{Cipher, Container, CreateOptionsBuilder};
use crate::memory::{Id, MemoryBackend};

pub(super) fn setup_container() -> Container<MemoryBackend> {
    let options = CreateOptionsBuilder::<MemoryBackend>::new(MemoryBackend::new(), Cipher::None)
        .build()
        .unwrap();

    Container::create(options).unwrap()
}

fn make_block<B: Backend>(
    container: &mut Container<B>,
    id: &B::Id,
    prev: &B::Id,
    next: &B::Id,
    payload: &[u8],
) {
    let mut writer = Writer::new(vec![]);

    writer.serialize(prev).unwrap(); // prev
    writer.serialize(next).unwrap(); // next
    writer.write_u32(payload.len() as u32).unwrap(); // length
    writer.write_bytes(payload).unwrap();

    container.write(&id, writer.as_ref().as_ref()).unwrap();
}

pub(super) fn setup_one_with(payload: &[u8]) -> (Container<MemoryBackend>, Id) {
    let mut container = setup_container();
    let id = container.aquire().unwrap();
    let null = Id::null();

    make_block(&mut container, &id, &id, &null, payload);

    (container, id)
}

pub(super) fn setup_one() -> (Container<MemoryBackend>, Id) {
    setup_one_with(&[1, 2, 3])
}

pub(super) fn setup_two_with(
    payload1: &[u8],
    payload2: &[u8],
) -> (Container<MemoryBackend>, (Id, Id)) {
    let mut container = setup_container();
    let id1 = container.aquire().unwrap();
    let id2 = container.aquire().unwrap();
    let null = Id::null();

    make_block(&mut container, &id1, &id2, &id2, payload1);
    make_block(&mut container, &id2, &id1, &null, payload2);

    (container, (id1, id2))
}

pub(super) fn setup_two() -> (Container<MemoryBackend>, (Id, Id)) {
    setup_two_with(&[1, 2, 3], &[4, 5, 6])
}

pub(super) fn setup_three_with(
    payload1: &[u8],
    payload2: &[u8],
    payload3: &[u8],
) -> (Container<MemoryBackend>, (Id, Id, Id)) {
    let mut container = setup_container();
    let id1 = container.aquire().unwrap();
    let id2 = container.aquire().unwrap();
    let id3 = container.aquire().unwrap();
    let null = Id::null();

    make_block(&mut container, &id1, &id3, &id2, payload1);
    make_block(&mut container, &id2, &id1, &id3, payload2);
    make_block(&mut container, &id3, &id2, &null, payload3);

    (container, (id1, id2, id3))
}

pub(super) fn setup_three() -> (Container<MemoryBackend>, (Id, Id, Id)) {
    setup_three_with(&[1, 2, 3], &[4, 5, 6], &[7, 8, 9])
}

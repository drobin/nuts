// MIT License
//
// Copyright (c) 2023 Robin Doer
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

use std::io::Write;
use testx::testx;

use crate::container::Container;
use crate::memory::MemId;
use crate::memory::MemoryBackend;
use crate::openssl::rand::RND;
use crate::stream::Stream;

use super::{assert_next_is_none, next, MAX_PAYLOAD};

#[testx(setup = super::setup_container)]
fn empty_1_part_no_flush(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&[1, 2, 3]).unwrap(), 3);

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_1_part_flushed(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&[1, 2, 3]).unwrap(), 3);
        stream.flush().unwrap();

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), [1, 2, 3]);
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_1_full_no_flush(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&RND[..MAX_PAYLOAD]).unwrap(), MAX_PAYLOAD);

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_1_full_flushed(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&RND[..MAX_PAYLOAD]).unwrap(), MAX_PAYLOAD);
        stream.flush().unwrap();

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_2_part_aligned_no_flush(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&RND[..MAX_PAYLOAD]).unwrap(), MAX_PAYLOAD);
        assert_eq!(stream.write(&[1, 2, 3]).unwrap(), 3);

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_2_part_aligned_flushed(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&RND[..MAX_PAYLOAD]).unwrap(), MAX_PAYLOAD);
        assert_eq!(stream.write(&[1, 2, 3]).unwrap(), 3);
        stream.flush().unwrap();

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_2_part_unaligned_no_flush(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&RND[..400]).unwrap(), 400);
        assert_eq!(stream.write(&RND[400..800]).unwrap(), MAX_PAYLOAD - 400);
        assert_eq!(stream.write(&RND[493..800]).unwrap(), 800 - MAX_PAYLOAD);

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_container)]
fn empty_2_part_unaligned_flushed(mut container: Container<MemoryBackend>) {
    let id = {
        let mut stream = Stream::create(&mut container);

        assert_eq!(stream.write(&RND[..400]).unwrap(), 400);
        assert_eq!(stream.write(&RND[400..800]).unwrap(), MAX_PAYLOAD - 400);
        assert_eq!(stream.write(&RND[493..800]).unwrap(), 800 - MAX_PAYLOAD);
        stream.flush().unwrap();

        stream.first_id().cloned().unwrap()
    };

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(stream.current_payload().unwrap(), &RND[MAX_PAYLOAD..800]);
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one)]
fn one_part_make_full_no_flush(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(
            stream.write(&RND[..MAX_PAYLOAD - 3]).unwrap(),
            MAX_PAYLOAD - 3
        );
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[1, 2, 3], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one)]
fn one_part_make_full_flushed(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);
        assert_eq!(
            stream.write(&RND[..MAX_PAYLOAD - 3]).unwrap(),
            MAX_PAYLOAD - 3
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[1, 2, 3], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one)]
fn one_part_fill_next_no_flush(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(stream.write(&RND).unwrap(), MAX_PAYLOAD - 3);
        assert_eq!(
            stream
                .write(&RND[MAX_PAYLOAD - 3..MAX_PAYLOAD - 3 + 100])
                .unwrap(),
            100
        );
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[1, 2, 3], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    next!(stream);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one)]
fn one_part_fill_next_flushed(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(stream.write(&RND).unwrap(), MAX_PAYLOAD - 3);
        assert_eq!(
            stream
                .write(&RND[MAX_PAYLOAD - 3..MAX_PAYLOAD - 3 + 100])
                .unwrap(),
            100
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[1, 2, 3], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD - 3..MAX_PAYLOAD - 3 + 100]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one_full)]
fn one_full_fill_next_part_no_flush(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..MAX_PAYLOAD + 100]).unwrap(),
            100
        );
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one_full)]
fn one_full_fill_next_part_flushed(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..MAX_PAYLOAD + 100]).unwrap(),
            100
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD..MAX_PAYLOAD + 100]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one_full)]
fn one_full_fill_next_no_flush(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]).unwrap(),
            MAX_PAYLOAD
        );
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_one_full)]
fn one_full_fill_next_flushed(mut container: Container<MemoryBackend>, id: MemId) {
    {
        let mut stream = Stream::new(&mut container, &id);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]).unwrap(),
            MAX_PAYLOAD
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id);

    assert_eq!(next!(stream), &id);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_part_make_full_no_flush(
    mut container: Container<MemoryBackend>,
    (id1, id2): (MemId, MemId),
) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(
            stream.write(&RND[..MAX_PAYLOAD - 3]).unwrap(),
            MAX_PAYLOAD - 3
        );
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[4, 5, 6], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_part_make_full_flushed(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    {
        let mut stream = Stream::new(&mut container, &id1);
        assert_eq!(
            stream.write(&RND[..MAX_PAYLOAD - 3]).unwrap(),
            MAX_PAYLOAD - 3
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[4, 5, 6], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_part_fill_next_no_flush(
    mut container: Container<MemoryBackend>,
    (id1, id2): (MemId, MemId),
) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(stream.write(&RND).unwrap(), MAX_PAYLOAD - 3);
        assert_eq!(
            stream
                .write(&RND[MAX_PAYLOAD - 3..MAX_PAYLOAD - 3 + 100])
                .unwrap(),
            100
        );
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[4, 5, 6], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    next!(stream);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two)]
fn two_part_fill_next_flushed(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(stream.write(&RND).unwrap(), MAX_PAYLOAD - 3);
        assert_eq!(
            stream
                .write(&RND[MAX_PAYLOAD - 3..MAX_PAYLOAD - 3 + 100])
                .unwrap(),
            100
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(
        stream.current_payload().unwrap(),
        [&[4, 5, 6], &RND[..MAX_PAYLOAD - 3]].concat()
    );
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD - 3..MAX_PAYLOAD - 3 + 100]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two_full)]
fn two_full_fill_next_part_no_flush(
    mut container: Container<MemoryBackend>,
    (id1, id2): (MemId, MemId),
) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..MAX_PAYLOAD + 100]).unwrap(),
            100
        );
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert!(stream.current_payload().unwrap().is_empty());
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two_full)]
fn two_full_fill_next_part_flushed(
    mut container: Container<MemoryBackend>,
    (id1, id2): (MemId, MemId),
) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..MAX_PAYLOAD + 100]).unwrap(),
            100
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD..MAX_PAYLOAD + 100]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two_full)]
fn two_full_fill_next_no_flush(
    mut container: Container<MemoryBackend>,
    (id1, id2): (MemId, MemId),
) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]).unwrap(),
            MAX_PAYLOAD
        );
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]
    );
    assert_next_is_none!(stream);
}

#[testx(setup = super::setup_two_full)]
fn two_full_fill_next_flushed(mut container: Container<MemoryBackend>, (id1, id2): (MemId, MemId)) {
    {
        let mut stream = Stream::new(&mut container, &id1);

        assert_eq!(
            stream.write(&RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]).unwrap(),
            MAX_PAYLOAD
        );
        stream.flush().unwrap();
    }

    let mut stream = Stream::new(&mut container, &id1);

    assert_eq!(next!(stream), &id1);
    assert_eq!(stream.current_payload().unwrap(), &[1, 2, 3]);
    assert_eq!(next!(stream), &id2);
    assert_eq!(stream.current_payload().unwrap(), &RND[..MAX_PAYLOAD]);
    next!(stream);
    assert_eq!(
        stream.current_payload().unwrap(),
        &RND[MAX_PAYLOAD..2 * MAX_PAYLOAD]
    );
    assert_next_is_none!(stream);
}

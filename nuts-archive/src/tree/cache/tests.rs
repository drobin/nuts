// MIT License
//
// Copyright (c) 2023,2024 Robin Doer
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

use nuts_backend::BlockId;
use nuts_container::memory::MemoryBackend;

use crate::pager::Pager;
use crate::tests::setup_container_with_bsize;
use crate::tree::cache::Cache;

#[test]
fn new() {
    let pager = Pager::new(setup_container_with_bsize(12));
    let cache = Cache::<MemoryBackend>::new(&pager);

    assert!(cache.id.is_null());
    assert_eq!(cache.node.len(), 3);
    assert!(cache.node[0].is_null());
    assert!(cache.node[1].is_null());
    assert!(cache.node[2].is_null());
}

#[test]
fn refresh() {
    let mut container = setup_container_with_bsize(12);

    let id = container.aquire().unwrap();
    assert_eq!(
        container
            .write(&id, &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3])
            .unwrap(),
        12
    );

    let mut pager = Pager::new(container);

    let mut cache = Cache::<MemoryBackend>::new(&pager);
    assert!(cache.refresh(&mut pager, &id).unwrap());

    assert_eq!(cache.id, id);
    assert_eq!(cache.node.len(), 3);
    assert_eq!(cache.node[0], "1".parse().unwrap());
    assert_eq!(cache.node[1], "2".parse().unwrap());
    assert_eq!(cache.node[2], "3".parse().unwrap());

    assert!(!cache.refresh(&mut pager, &id).unwrap());

    assert_eq!(cache.id, id);
    assert_eq!(cache.node.len(), 3);
    assert_eq!(cache.node[0], "1".parse().unwrap());
    assert_eq!(cache.node[1], "2".parse().unwrap());
    assert_eq!(cache.node[2], "3".parse().unwrap());
}

#[test]
fn aquire_null_no_leaf() {
    let mut container = setup_container_with_bsize(12);

    let id = container.aquire().unwrap();
    assert_eq!(
        container
            .write(&id, &[0xff, 0xff, 0xff, 0xff, 0, 0, 0, 2, 0, 0, 0, 3])
            .unwrap(),
        12
    );

    let mut pager = Pager::new(container);

    let mut cache = Cache::<MemoryBackend>::new(&pager);

    assert!(cache.refresh(&mut pager, &id).unwrap());
    assert!(cache.aquire(&mut pager, 0, false).unwrap());

    let mut buf = [0; 12];
    let mut container = pager.into_container();

    assert_eq!(container.read(&id, &mut buf).unwrap(), 12);
    assert_eq!(buf, [0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3]);

    assert_eq!(container.read(&"2".parse().unwrap(), &mut buf).unwrap(), 12);
    assert_eq!(buf, [0xff; 12]);
}

#[test]
fn aquire_not_null_no_leaf() {
    let mut container = setup_container_with_bsize(12);

    let id = container.aquire().unwrap();
    assert_eq!(
        container
            .write(&id, &[0xff, 0xff, 0xff, 0xff, 0, 0, 0, 2, 0, 0, 0, 3])
            .unwrap(),
        12
    );

    let mut pager = Pager::new(container);

    let mut cache = Cache::<MemoryBackend>::new(&pager);

    assert!(cache.refresh(&mut pager, &id).unwrap());
    assert!(!cache.aquire(&mut pager, 1, false).unwrap());

    let mut buf = [0; 12];
    let mut container = pager.into_container();

    assert_eq!(container.read(&id, &mut buf).unwrap(), 12);
    assert_eq!(buf, [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 2, 0, 0, 0, 3]);
}

#[test]
fn aquire_null_leaf() {
    let mut container = setup_container_with_bsize(12);

    let id = container.aquire().unwrap();
    assert_eq!(
        container
            .write(&id, &[0xff, 0xff, 0xff, 0xff, 0, 0, 0, 2, 0, 0, 0, 3])
            .unwrap(),
        12
    );

    let mut pager = Pager::new(container);

    let mut cache = Cache::<MemoryBackend>::new(&pager);

    assert!(cache.refresh(&mut pager, &id).unwrap());
    assert!(cache.aquire(&mut pager, 0, true).unwrap());

    let mut buf = [0; 12];
    let mut container = pager.into_container();

    assert_eq!(container.read(&id, &mut buf).unwrap(), 12);
    assert_eq!(buf, [0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3]);

    assert_eq!(container.read(&"2".parse().unwrap(), &mut buf).unwrap(), 12);
    assert_eq!(buf, [0; 12]);
}

#[test]
fn aquire_not_null_leaf() {
    let mut container = setup_container_with_bsize(12);

    let id = container.aquire().unwrap();
    assert_eq!(
        container
            .write(&id, &[0xff, 0xff, 0xff, 0xff, 0, 0, 0, 2, 0, 0, 0, 3])
            .unwrap(),
        12
    );

    let mut pager = Pager::new(container);

    let mut cache = Cache::<MemoryBackend>::new(&pager);

    assert!(cache.refresh(&mut pager, &id).unwrap());
    assert!(!cache.aquire(&mut pager, 1, true).unwrap());

    let mut buf = [0; 12];
    let mut container = pager.into_container();

    assert_eq!(container.read(&id, &mut buf).unwrap(), 12);
    assert_eq!(buf, [0xff, 0xff, 0xff, 0xff, 0, 0, 0, 2, 0, 0, 0, 3]);
}

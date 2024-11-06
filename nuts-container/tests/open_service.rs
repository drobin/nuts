// MIT License
//
// Copyright (c) 2024 Robin Doer
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

mod common;

use nuts_container::{Container, OpenOptionsBuilder};
use nuts_memory::MemoryBackend;
use std::fs::File;

use crate::common::{fixture_path, SampleService};

fn open_backend_from_fixture(dir: &str, name: &str) -> MemoryBackend {
    let path = fixture_path(dir, name);
    let file = File::open(path).unwrap();

    serde_json::from_reader(file).unwrap()
}

fn open_container(backend: MemoryBackend) -> Container<MemoryBackend> {
    let options = OpenOptionsBuilder::new().build::<MemoryBackend>().unwrap();

    Container::open(backend, options).unwrap()
}

macro_rules! t {
    ($name:ident ( $migrate:expr ), $json:literal, $rev_in:literal -> $rev_out:literal) => {
        #[test]
        fn $name() {
            let backend = {
                let backend = open_backend_from_fixture("service", $json);
                let container = open_container(backend);

                assert_eq!(container.info().unwrap().revision, $rev_in);

                let service =
                    Container::open_service::<SampleService>(container, $migrate).unwrap();

                service.into_container().into_backend()
            };

            let container = open_container(backend);

            assert_eq!(container.info().unwrap().revision, $rev_out);
        }
    };
}

t!(open_0_6_8_no_migration(false), "0.6.8.json", 0 -> 0);
t!(open_0_6_8_migration(true), "0.6.8.json", 0 -> 2);
t!(open_0_7_0_no_migration(false), "0.7.0.json", 1 -> 1);
t!(open_0_7_0_migration(true), "0.7.0.json", 1 -> 2);
t!(open_0_7_1_no_migration(false), "0.7.1.json", 1 -> 1);
t!(open_0_7_1_migration(true), "0.7.1.json", 1 -> 2);
t!(open_0_7_3_no_migration(false), "0.7.3.json", 2 -> 2);
t!(open_0_7_3_migration(true), "0.7.3.json", 2 -> 2);

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

use nuts_archive::ArchiveFactory;
use nuts_container::{Cipher, Container, OpenOptionsBuilder};
use nuts_directory::{DirectoryBackend, OpenOptions};
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn password() -> Result<Vec<u8>, String> {
    Ok(b"sample".to_vec())
}

fn data_dir(name: &str) -> PathBuf {
    [MANIFEST_DIR, "data", name].iter().collect()
}

macro_rules! make_test {
    ($name:ident, $path:literal, $cipher:ident, $top_id:literal) => {
        #[test]
        fn $name() {
            let backend_options = OpenOptions::for_path(data_dir($path));
            let container_options = OpenOptionsBuilder::new()
                .with_password_callback(password)
                .build::<DirectoryBackend<PathBuf>>()
                .unwrap();
            let container = Container::open(backend_options, container_options).unwrap();

            let mut archive = Container::open_service::<ArchiveFactory>(container).unwrap();
            let container = archive.as_ref();

            let cipher = container.info().unwrap().cipher;
            assert_eq!(cipher, Cipher::$cipher);

            let top_id = container.top_id().unwrap();
            assert_eq!(top_id.to_string(), $top_id);

            let mut entry = archive.first().unwrap().unwrap();
            assert!(entry.is_file());
            assert_eq!(entry.name(), "f1");
            assert_eq!(
                entry.as_file_mut().unwrap().read_vec().unwrap(),
                b"content of f1\n"
            );

            let entry = entry.next().unwrap().unwrap();
            assert!(entry.is_directory());
            assert_eq!(entry.name(), "f2");

            let entry = entry.next().unwrap().unwrap();
            assert!(entry.is_symlink());
            assert_eq!(entry.name(), "f3");
            assert_eq!(entry.as_symlink().unwrap().target(), "f1");

            assert!(entry.next().is_none());
        }
    };
}

make_test!(
    compat_0_6_8_none,
    "0.6.8-none",
    None,
    "882d2bd07e91ce63c2e5f8ec28d31158"
);

make_test!(
    compat_0_6_8_aes128_ctr,
    "0.6.8-aes128ctr",
    Aes128Ctr,
    "0bccd7957a975a879946bb009f9d9998"
);

make_test!(
    compat_0_6_8_aes128_gcm,
    "0.6.8-aes128gcm",
    Aes128Gcm,
    "a258a48284da174ae87850f25d6921f0"
);

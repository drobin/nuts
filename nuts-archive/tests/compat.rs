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

// no compatibility tests for 0.7.0 - creation of archive is broken here

make_test!(
    compat_0_7_1_none,
    "0.7.1-none",
    None,
    "206f9e91ac3296f3563752361054fed8"
);

make_test!(
    compat_0_7_1_aes128_ctr,
    "0.7.1-aes128ctr",
    Aes128Ctr,
    "ec6f5f40e9d57d10904012cb2a6346f6"
);

make_test!(
    compat_0_7_1_aes192_ctr,
    "0.7.1-aes192ctr",
    Aes192Ctr,
    "b57de7da43a2a9ab2b725e336b3889fe"
);

make_test!(
    compat_0_7_1_aes256_ctr,
    "0.7.1-aes256ctr",
    Aes256Ctr,
    "99be9f6d09fdb9caf0d883883f547b72"
);

make_test!(
    compat_0_7_1_aes128_gcm,
    "0.7.1-aes128gcm",
    Aes128Gcm,
    "dae99bab892feb7b9e0eac8c7c7048a3"
);

make_test!(
    compat_0_7_1_aes192_gcm,
    "0.7.1-aes192gcm",
    Aes192Gcm,
    "e7da7461db40a382732ccd0b1d76e23e"
);

make_test!(
    compat_0_7_1_aes256_gcm,
    "0.7.1-aes256gcm",
    Aes256Gcm,
    "4b9cba4e31af1c86ded4408c00b888c3"
);

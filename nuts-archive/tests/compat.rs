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
use nuts_memory::MemoryBackend;
use std::fs::File;
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn password() -> Result<Vec<u8>, String> {
    Ok(b"sample".to_vec())
}

fn fixture_path(name: &str) -> PathBuf {
    [MANIFEST_DIR, "data", name].iter().collect()
}

// #[test]
// fn create_test_data() {
//     for (path, cipher) in [
//         ("0.7.3-none.json", Cipher::None),
//         ("0.7.3-aes128ctr.json", Cipher::Aes128Ctr),
//         ("0.7.3-aes128gcm.json", Cipher::Aes128Gcm),
//         ("0.7.3-aes192ctr.json", Cipher::Aes192Ctr),
//         ("0.7.3-aes192gcm.json", Cipher::Aes192Gcm),
//         ("0.7.3-aes256ctr.json", Cipher::Aes256Ctr),
//         ("0.7.3-aes256gcm.json", Cipher::Aes256Gcm),
//     ] {
//         let options = nuts_container::CreateOptionsBuilder::new(cipher)
//             .with_password_callback(password)
//             .build::<MemoryBackend>()
//             .unwrap();

//         let container = Container::create(MemoryBackend::new(), options).unwrap();
//         let mut archive = Container::create_service::<ArchiveFactory>(container).unwrap();

//         let mut entry = archive.append_file("f1").build().unwrap();
//         entry.write_all(b"content of f1").unwrap();

//         archive.append_directory("f2").build().unwrap();

//         archive.append_symlink("f3", "f1").build().unwrap();

//         let backend = archive.into_container().into_backend();
//         let file = File::create(fixture_path(path)).unwrap();

//         serde_json::to_writer(file, &backend).unwrap();
//     }
// }

macro_rules! make_test {
    ($name:ident, $path:literal, $cipher:ident) => {
        #[test]
        fn $name() {
            let file = File::open(fixture_path($path)).unwrap();
            let backend: MemoryBackend = serde_json::from_reader(file).unwrap();

            let options = OpenOptionsBuilder::new()
                .with_password_callback(password)
                .build::<MemoryBackend>()
                .unwrap();
            let container = Container::open(backend, options).unwrap();

            let mut archive = Container::open_service::<ArchiveFactory>(container, false).unwrap();
            let container = archive.as_ref();

            let cipher = container.info().unwrap().cipher;
            assert_eq!(cipher, Cipher::$cipher);

            let top_id = container.top_id().unwrap();
            assert_eq!(top_id.to_string(), "1");

            let mut entry = archive.first().unwrap().unwrap();
            assert!(entry.is_file());
            assert_eq!(entry.name(), "f1");
            assert_eq!(
                entry.as_file_mut().unwrap().read_vec().unwrap(),
                b"content of f1"
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

make_test!(compat_0_6_8_none, "0.6.8-none.json", None);
make_test!(compat_0_6_8_aes128_ctr, "0.6.8-aes128ctr.json", Aes128Ctr);
make_test!(compat_0_6_8_aes128_gcm, "0.6.8-aes128gcm.json", Aes128Gcm);

make_test!(compat_0_7_0_none, "0.7.0-none.json", None);
make_test!(compat_0_7_0_aes128_ctr, "0.7.0-aes128ctr.json", Aes128Ctr);
make_test!(compat_0_7_0_aes192_ctr, "0.7.0-aes192ctr.json", Aes192Ctr);
make_test!(compat_0_7_0_aes256_ctr, "0.7.0-aes256ctr.json", Aes256Ctr);
make_test!(compat_0_7_0_aes128_gcm, "0.7.0-aes128gcm.json", Aes128Gcm);
make_test!(compat_0_7_0_aes192_gcm, "0.7.0-aes192gcm.json", Aes192Gcm);
make_test!(compat_0_7_0_aes256_gcm, "0.7.0-aes256gcm.json", Aes256Gcm);

make_test!(compat_0_7_1_none, "0.7.1-none.json", None);
make_test!(compat_0_7_1_aes128_ctr, "0.7.1-aes128ctr.json", Aes128Ctr);
make_test!(compat_0_7_1_aes192_ctr, "0.7.1-aes192ctr.json", Aes192Ctr);
make_test!(compat_0_7_1_aes256_ctr, "0.7.1-aes256ctr.json", Aes256Ctr);
make_test!(compat_0_7_1_aes128_gcm, "0.7.1-aes128gcm.json", Aes128Gcm);
make_test!(compat_0_7_1_aes192_gcm, "0.7.1-aes192gcm.json", Aes192Gcm);
make_test!(compat_0_7_1_aes256_gcm, "0.7.1-aes256gcm.json", Aes256Gcm);

make_test!(compat_0_7_3_none, "0.7.3-none.json", None);
make_test!(compat_0_7_3_aes128_ctr, "0.7.3-aes128ctr.json", Aes128Ctr);
make_test!(compat_0_7_3_aes192_ctr, "0.7.3-aes192ctr.json", Aes192Ctr);
make_test!(compat_0_7_3_aes256_ctr, "0.7.3-aes256ctr.json", Aes256Ctr);
make_test!(compat_0_7_3_aes128_gcm, "0.7.3-aes128gcm.json", Aes128Gcm);
make_test!(compat_0_7_3_aes192_gcm, "0.7.3-aes192gcm.json", Aes192Gcm);
make_test!(compat_0_7_3_aes256_gcm, "0.7.3-aes256gcm.json", Aes256Gcm);

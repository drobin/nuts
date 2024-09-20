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

use nuts_container::{Cipher, Container, Digest, Kdf, OpenOptionsBuilder};
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
    ($name:ident, $path:literal, $id:literal, | $info:ident | $assert_info:block) => {
        #[test]
        fn $name() {
            let backend_options = OpenOptions::for_path(data_dir($path));
            let container_options = OpenOptionsBuilder::new()
                .with_password_callback(password)
                .build::<DirectoryBackend<PathBuf>>()
                .unwrap();

            let mut container = Container::open(backend_options, container_options).unwrap();

            assert!(container.top_id().is_none());

            let $info = container.info().unwrap();
            $assert_info

            let id = $id.parse().unwrap();
            let mut buf = [0; 8];
            let n = container.read(&id, &mut buf).unwrap();

            assert_eq!(n, 8);
            assert_eq!(&buf, b"0.6.8\n\0\0");
        }
    };
}

make_test!(
    compat_0_6_8_none,
    "0.6.8-none",
    "ef6a70398d3c4ae8813c4832ce191df5",
    |info| {
        assert_eq!(info.backend.bsize, 512);
        assert_eq!(info.revision, 0);
        assert_eq!(info.cipher, Cipher::None);
        assert_eq!(info.kdf, Kdf::None);
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_6_8_aes128_ctr,
    "0.6.8-aes128ctr",
    "7fa14f6895f307961b70ff5a6256e9e2",
    |info| {
        assert_eq!(info.backend.bsize, 512);
        assert_eq!(info.revision, 0);
        assert_eq!(info.cipher, Cipher::Aes128Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha1 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_6_8_aes128_gcm,
    "0.6.8-aes128gcm",
    "ea9743995df9d8032c5c251b316a22ac",
    |info| {
        assert_eq!(info.backend.bsize, 512);
        assert_eq!(info.revision, 0);
        assert_eq!(info.cipher, Cipher::Aes128Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha1 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

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

use nuts_container::{Cipher, Container, Digest, Kdf, OpenOptionsBuilder};
use nuts_memory::MemoryBackend;
use std::fs::File;

use crate::common::{fixture_password, fixture_path};

// #[test]
// fn create_test_data() {
//     const VERSION: &[u8] = b"0.7.3";

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

//         let mut container = Container::create(MemoryBackend::new(), options).unwrap();
//         let id = container.aquire().unwrap();

//         container.write(&id, VERSION).unwrap();

//         let backend = container.into_backend();
//         let file = File::create(fixture_path(path)).unwrap();

//         serde_json::to_writer(file, &backend).unwrap();
//     }
// }

macro_rules! make_test {
    ($name:ident, $path:literal, $version:literal, | $info:ident | $assert_info:block) => {
        #[test]
        fn $name() {
            let file = File::open(fixture_path("compat", $path)).unwrap();
            let backend: MemoryBackend = serde_json::from_reader(file).unwrap();

            let options = OpenOptionsBuilder::new()
                .with_password_callback(fixture_password)
                .build::<MemoryBackend>()
                .unwrap();

            let mut container = Container::open(backend, options).unwrap();

            assert!(container.top_id().is_none());

            let $info = container.info().unwrap();
            $assert_info

            let id = "1".parse().unwrap();
            let mut buf = [0; 8];
            let n = container.read(&id, &mut buf).unwrap();

            assert_eq!(n, 8);
            assert_eq!(&buf, $version);
        }
    };
}

make_test!(
    compat_0_6_8_none,
    "0.6.8-none.json",
    b"0.6.8\0\0\0",
    |info| {
        assert_eq!(info.revision, 0);
        assert_eq!(info.cipher, Cipher::None);
        assert_eq!(info.kdf, Kdf::None);
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_6_8_aes128_ctr,
    "0.6.8-aes128ctr.json",
    b"0.6.8\0\0\0",
    |info| {
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
    "0.6.8-aes128gcm.json",
    b"0.6.8\0\0\0",
    |info| {
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

make_test!(
    compat_0_7_0_none,
    "0.7.0-none.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::None);
        assert_eq!(info.kdf, Kdf::None);
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_0_aes128ctr,
    "0.7.0-aes128ctr.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes128Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_0_aes192ctr,
    "0.7.0-aes192ctr.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes192Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_0_aes256ctr,
    "0.7.0-aes256ctr.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes256Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_0_aes128gcm,
    "0.7.0-aes128gcm.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes128Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_0_aes192gcm,
    "0.7.0-aes192gcm.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes192Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_0_aes256gcm,
    "0.7.0-aes256gcm.json",
    b"0.7.0\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes256Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_1_none,
    "0.7.1-none.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::None);
        assert_eq!(info.kdf, Kdf::None);
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_1_aes128ctr,
    "0.7.1-aes128ctr.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes128Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_1_aes192ctr,
    "0.7.1-aes192ctr.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes192Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_1_aes256ctr,
    "0.7.1-aes256ctr.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes256Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_1_aes128gcm,
    "0.7.1-aes128gcm.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes128Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_1_aes192gcm,
    "0.7.1-aes192gcm.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes192Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_1_aes256gcm,
    "0.7.1-aes256gcm.json",
    b"0.7.1\0\0\0",
    |info| {
        assert_eq!(info.revision, 1);
        assert_eq!(info.cipher, Cipher::Aes256Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_3_none,
    "0.7.3-none.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::None);
        assert_eq!(info.kdf, Kdf::None);
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_3_aes128ctr,
    "0.7.3-aes128ctr.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::Aes128Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_3_aes192ctr,
    "0.7.3-aes192ctr.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::Aes192Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_3_aes256ctr,
    "0.7.3-aes256ctr.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::Aes256Ctr);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 512);
    }
);

make_test!(
    compat_0_7_3_aes128gcm,
    "0.7.3-aes128gcm.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::Aes128Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_3_aes192gcm,
    "0.7.3-aes192gcm.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::Aes192Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

make_test!(
    compat_0_7_3_aes256gcm,
    "0.7.3-aes256gcm.json",
    b"0.7.3\0\0\0",
    |info| {
        assert_eq!(info.revision, 2);
        assert_eq!(info.cipher, Cipher::Aes256Gcm);
        assert!(matches!(
            info.kdf,
            Kdf::Pbkdf2 {
                digest,
                iterations,
                salt
            } if digest == Digest::Sha256 && iterations == 65536 && salt.len() == 16
        ));
        assert_eq!(info.bsize_gross, 512);
        assert_eq!(info.bsize_net, 496);
    }
);

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

macro_rules! write_tests {
    ($size:literal, full -> $full:expr, less -> $less:expr, empty -> $empty:expr) => {
        #[test]
        fn full() {
            let (mut container, id) = setup_container();

            let n = container.write(&id, &RND).unwrap();
            let data = container.backend().get(&id).unwrap();

            assert_eq!(n, $size);
            assert_eq!(data, &$full);
        }

        #[test]
        fn less() {
            let (mut container, id) = setup_container();

            let n = container.write(&id, &RND[..$size - 1]).unwrap();
            let data = container.backend().get(&id).unwrap();

            assert_eq!(n, $size - 1);
            assert_eq!(data, &$less);
        }

        #[test]
        fn empty() {
            let (mut container, id) = setup_container();

            let n = container.write(&id, &[]).unwrap();
            let data = container.backend().get(&id).unwrap();

            assert_eq!(n, 0);
            assert_eq!(data, $empty);
        }

        #[test]
        fn more() {
            let (mut container, id) = setup_container();

            let n = container.write(&id, &RND[..$size + 1]).unwrap();
            let data = container.backend().get(&id).unwrap();

            assert_eq!(n, $size);
            assert_eq!(data, &$full);
        }

        #[test]
        fn null() {
            let (mut container, _) = setup_container();

            let err = container.write(&Id::null(), &[]).unwrap_err();
            assert!(matches!(err, Error::NullId));
        }

        #[test]
        fn no_such_id() {
            let (mut container, id) = setup_container();

            container.release(id.clone()).unwrap();

            let err = container.write(&id, &mut []).unwrap_err();

            let err = into_error!(err, Error::Backend);
            let err_id = into_error!(err, MemoryError::NoSuchId);
            assert_eq!(err_id, id);
        }
    };
}

mod none {
    use nuts_backend::BlockId;

    use crate::container::{Cipher, Container, CreateOptionsBuilder, Error};
    use crate::memory::{Error as MemoryError, Id, MemoryBackend};
    use crate::tests::{into_error, RND};

    fn setup_container() -> (Container<MemoryBackend>, Id) {
        let backend = MemoryBackend::new();
        let options = CreateOptionsBuilder::new(Cipher::None)
            .build::<MemoryBackend>()
            .unwrap();

        let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();
        let id = container.aquire().unwrap();

        (container, id)
    }

    write_tests!(512, full -> RND[..512], less -> [&RND[..511], &[0]].concat(), empty -> [0; 512]);
}

mod aed128_ctr {
    use nuts_backend::BlockId;

    use crate::container::tests::CTEXT_AES128_CTR;
    use crate::container::{Cipher, Container, CreateOptionsBuilder, Digest, Error, Kdf};
    use crate::memory::{Error as MemoryError, Id, MemoryBackend};
    use crate::tests::{into_error, RND};

    const LESS: [u8; 512] = [
        0xfb, 0x7b, 0x8e, 0xef, 0x3e, 0xff, 0xde, 0x50, 0x70, 0xaa, 0x93, 0x9c, 0xf1, 0x44, 0xa1,
        0x12, 0xad, 0x3f, 0x96, 0xb8, 0x26, 0xab, 0xb4, 0xd, 0x51, 0xad, 0x11, 0xdf, 0x16, 0x5d,
        0x79, 0x5a, 0xf4, 0x99, 0x3, 0x2a, 0x8c, 0x8d, 0x1, 0x2a, 0x7f, 0x67, 0x65, 0x9d, 0xab,
        0xab, 0x9d, 0x5c, 0x13, 0xcc, 0xba, 0x19, 0x22, 0x54, 0x8, 0x95, 0x9e, 0x5f, 0xb2, 0xa7,
        0x4e, 0x79, 0x41, 0xaa, 0xa1, 0xd5, 0x5a, 0xe6, 0xc5, 0xd5, 0x78, 0x92, 0xed, 0x35, 0x94,
        0x61, 0x58, 0xfa, 0x9a, 0x78, 0x1c, 0xf5, 0x7f, 0xc1, 0xd1, 0xf6, 0xf3, 0xd1, 0xea, 0x79,
        0x82, 0xa7, 0xa, 0x44, 0x49, 0xe0, 0xcf, 0x96, 0xa4, 0x4d, 0xc7, 0x7f, 0x14, 0xda, 0x68,
        0x88, 0xd1, 0xa0, 0x55, 0xf2, 0xf7, 0x6, 0x8c, 0x38, 0x8d, 0x3f, 0x10, 0xc0, 0xdf, 0x2e,
        0xf7, 0x23, 0xa4, 0x73, 0xa3, 0x62, 0x39, 0xc6, 0xbc, 0x2c, 0x10, 0xeb, 0x41, 0x36, 0x1d,
        0x73, 0x66, 0x67, 0x97, 0xa1, 0x3, 0x54, 0xec, 0xa, 0x4c, 0xff, 0xb8, 0xc2, 0x83, 0x73,
        0xe6, 0xc7, 0x4f, 0x5b, 0xb7, 0xf0, 0xa3, 0xe, 0xfa, 0xc0, 0x6, 0x51, 0x3d, 0xae, 0x87,
        0x53, 0xdf, 0xd2, 0x91, 0xa, 0x2c, 0x14, 0x5a, 0xbd, 0x95, 0x37, 0x62, 0x3c, 0x56, 0x3b,
        0xc4, 0x5d, 0xc6, 0x74, 0xa3, 0x8e, 0xaa, 0x66, 0xbb, 0x21, 0x98, 0xf7, 0x1b, 0x65, 0x33,
        0x1b, 0xd0, 0xf6, 0x13, 0x9a, 0x24, 0xb0, 0xb0, 0x6, 0x39, 0x67, 0xc2, 0x6d, 0x76, 0xea,
        0x48, 0xc4, 0xc2, 0xa6, 0xa0, 0x36, 0x58, 0x86, 0xa8, 0xaa, 0xf6, 0x7b, 0xf9, 0xa5, 0xa1,
        0x34, 0x20, 0x1a, 0x6f, 0xff, 0x98, 0xe8, 0x9b, 0x24, 0x48, 0x55, 0x38, 0xc8, 0xa9, 0x55,
        0xd9, 0xb7, 0xaa, 0xd0, 0x2, 0xa4, 0x4e, 0x28, 0xce, 0x79, 0x29, 0xc, 0x95, 0x3, 0x64, 0x1,
        0x1a, 0xe1, 0x61, 0x36, 0x52, 0xc9, 0xf, 0x7e, 0x2b, 0xef, 0xfb, 0x31, 0x8d, 0x2b, 0x6e,
        0xe2, 0xb, 0x2d, 0xa0, 0x4f, 0x4b, 0xef, 0x21, 0xb3, 0x3c, 0x86, 0xb0, 0x11, 0x6d, 0xdc,
        0x1e, 0x5e, 0x25, 0x35, 0x51, 0xd4, 0xef, 0xc6, 0xc4, 0x86, 0xff, 0x2e, 0xdd, 0x2, 0x16,
        0xe, 0x8a, 0x7d, 0xc5, 0x2c, 0x43, 0xfe, 0x4a, 0xf7, 0x2a, 0x3f, 0x72, 0xba, 0x4b, 0x45,
        0xe2, 0x6f, 0x71, 0xdb, 0x0, 0x29, 0xed, 0xb5, 0xa, 0x3b, 0x62, 0x69, 0xff, 0x68, 0x56,
        0xed, 0x18, 0xcd, 0x6c, 0x62, 0x8e, 0x7e, 0x38, 0x52, 0xa6, 0x7d, 0xe8, 0xfa, 0x16, 0x96,
        0x72, 0x9c, 0x75, 0x15, 0xac, 0x78, 0x15, 0xf5, 0xb8, 0x7f, 0x3e, 0x47, 0x2c, 0x3b, 0x16,
        0x68, 0xc0, 0x6e, 0x8d, 0xa1, 0x6, 0x60, 0x7d, 0x5, 0xaa, 0xfd, 0x68, 0xd2, 0xf1, 0x14,
        0x4e, 0x1d, 0xb8, 0xf6, 0x1, 0xcc, 0x10, 0x58, 0x3, 0x2a, 0xac, 0xbe, 0xc5, 0xc2, 0xe9,
        0xeb, 0xb3, 0xf7, 0xe2, 0xcc, 0xe1, 0x1d, 0x7f, 0x76, 0x80, 0x97, 0xd7, 0x42, 0xbf, 0x5e,
        0x49, 0x6, 0x7f, 0x5c, 0x5b, 0x2d, 0xbe, 0x9f, 0x7d, 0xa4, 0x77, 0xbe, 0x1, 0x7e, 0xd4,
        0xa7, 0x54, 0xbe, 0xbb, 0xa3, 0xd2, 0xdd, 0x94, 0x2, 0x82, 0x4d, 0x55, 0x9e, 0xb4, 0xbb,
        0x1, 0x50, 0x24, 0xab, 0x5a, 0xca, 0x34, 0x3a, 0x92, 0xc7, 0xd5, 0xff, 0x58, 0x9c, 0xe6,
        0x9c, 0x30, 0x7f, 0x33, 0x10, 0xf7, 0x8a, 0xd3, 0xc3, 0x2d, 0xff, 0x12, 0x9b, 0x51, 0x8a,
        0xa2, 0x5, 0xb4, 0x90, 0x66, 0xec, 0x4e, 0x0, 0x8c, 0x33, 0xcf, 0x1b, 0xae, 0xb7, 0xe7,
        0xd8, 0x5d, 0xcc, 0x8a, 0x88, 0xb9, 0x9d, 0x10, 0xf3, 0x68, 0x2f, 0xf2, 0x82, 0xf8, 0xd0,
        0xc, 0x2b, 0x98, 0x3d, 0xae, 0xc5, 0x68, 0x1a, 0xc2, 0xca, 0xc9, 0xcf, 0x7d, 0x16, 0xe8,
        0xf2,
    ];

    const EMPTY: [u8; 512] = [
        0x6a, 0xbb, 0x3c, 0x20, 0xd9, 0x2e, 0xc0, 0xb3, 0x69, 0xbd, 0x57, 0xd4, 0xb, 0x91, 0x8e,
        0x22, 0x57, 0x75, 0x88, 0x24, 0xf3, 0xf, 0x29, 0xb3, 0x51, 0x61, 0x53, 0xde, 0xe, 0xe8,
        0xc7, 0x55, 0xb8, 0xe8, 0x96, 0xdf, 0x7, 0xd7, 0xa0, 0x92, 0x50, 0x18, 0x86, 0xe1, 0x7d,
        0x29, 0xac, 0xe9, 0xc7, 0x45, 0x42, 0xd7, 0xfe, 0xf7, 0xa5, 0x6b, 0xed, 0x30, 0x5e, 0x5b,
        0x24, 0x32, 0xca, 0xd1, 0x5e, 0x97, 0x39, 0xbb, 0xbb, 0x5c, 0xde, 0x66, 0x76, 0xfa, 0x78,
        0x29, 0x57, 0xf2, 0xad, 0xe2, 0x5d, 0x16, 0x81, 0x35, 0x9b, 0xd5, 0x40, 0x30, 0x31, 0x8b,
        0xe0, 0x45, 0xb2, 0x94, 0xe1, 0x41, 0x9d, 0xeb, 0x44, 0x45, 0x3c, 0x4d, 0x62, 0x18, 0x25,
        0xf6, 0xd4, 0x18, 0xc6, 0x74, 0xdf, 0x7e, 0x3e, 0x89, 0xd, 0xf2, 0xe5, 0x6d, 0x52, 0xca,
        0x4b, 0x3a, 0x8c, 0xef, 0x40, 0x29, 0xb8, 0xa4, 0xe5, 0x71, 0xbf, 0x62, 0xeb, 0x78, 0x15,
        0xfa, 0xbf, 0x85, 0x93, 0x24, 0xeb, 0xcd, 0xb9, 0x4e, 0x4e, 0xc, 0x15, 0xbe, 0x7d, 0x5,
        0xa1, 0x48, 0x9d, 0x42, 0xd, 0xa1, 0xad, 0xa5, 0x5b, 0xb4, 0xbe, 0x47, 0x47, 0xa4, 0x75,
        0x69, 0x78, 0x8, 0x51, 0x89, 0xef, 0x6a, 0x29, 0xd5, 0x48, 0xae, 0x8a, 0x39, 0xd0, 0x49,
        0xe6, 0x9e, 0x5e, 0x99, 0x4d, 0x19, 0x87, 0x65, 0xee, 0xb0, 0xac, 0xc2, 0x9f, 0x36, 0x32,
        0xde, 0xde, 0x8d, 0xb8, 0xae, 0xb4, 0xe6, 0x5c, 0xcc, 0xaf, 0x58, 0xeb, 0xf5, 0x3, 0x28,
        0x4d, 0x56, 0x5f, 0x28, 0xdd, 0x2b, 0x10, 0x12, 0xfa, 0x6f, 0x71, 0x6, 0xa9, 0x61, 0x25,
        0x28, 0x30, 0xb5, 0x85, 0xa1, 0x1d, 0xa9, 0x15, 0x18, 0x7c, 0x9f, 0xe4, 0x94, 0x63, 0xdf,
        0x34, 0xbe, 0x81, 0x8e, 0x18, 0x2a, 0xad, 0xe0, 0x89, 0x9d, 0x1, 0x15, 0x79, 0xb6, 0x62,
        0x9d, 0x8b, 0x81, 0x12, 0xf6, 0xd2, 0x8a, 0x5f, 0x7e, 0x73, 0x53, 0x86, 0x50, 0xf, 0x45,
        0xec, 0xfd, 0x4c, 0xa, 0xdc, 0xf8, 0x20, 0x70, 0xb, 0x88, 0x21, 0x99, 0x44, 0x28, 0x5f,
        0x50, 0x6, 0x4d, 0xfa, 0xab, 0xa9, 0x15, 0x4a, 0x3c, 0x7f, 0xc1, 0x88, 0xa7, 0xfc, 0x49,
        0x83, 0xaa, 0x5, 0xed, 0x6a, 0x7, 0xb9, 0xbf, 0x11, 0x78, 0x5c, 0xf8, 0xcc, 0x8e, 0xf8,
        0x6f, 0x16, 0x9a, 0x7a, 0x20, 0x57, 0x8a, 0xbc, 0x32, 0xdf, 0x6d, 0x73, 0xf5, 0xad, 0xfa,
        0xea, 0xfb, 0x6b, 0xb, 0x4a, 0x8d, 0x41, 0x68, 0x48, 0x21, 0x95, 0x18, 0x7, 0xe, 0x24,
        0xd3, 0xe6, 0x4c, 0x87, 0xe7, 0xb1, 0x1f, 0x61, 0x40, 0x4a, 0x20, 0xdc, 0xe8, 0xd3, 0x81,
        0xe1, 0x6f, 0x7f, 0x11, 0xb, 0x44, 0x7b, 0xc1, 0x51, 0xfd, 0x70, 0x94, 0x64, 0xd5, 0xbe,
        0x43, 0x94, 0x79, 0x8e, 0x91, 0xf0, 0xe0, 0x49, 0x3e, 0xe6, 0xfb, 0x73, 0x97, 0xac, 0x0,
        0x89, 0x2, 0x72, 0xe6, 0xf3, 0xc6, 0x1e, 0x86, 0x3c, 0x6d, 0x6e, 0xee, 0x4b, 0x3c, 0x6f,
        0x64, 0x7a, 0x60, 0x91, 0x9, 0xf1, 0xd2, 0xbe, 0x4e, 0x67, 0xc2, 0xff, 0x51, 0xcd, 0x8a,
        0xc5, 0xb, 0xfe, 0xd8, 0x22, 0xe, 0x69, 0xbb, 0xc1, 0x9b, 0x93, 0xc5, 0x33, 0x55, 0x17,
        0xa0, 0xf5, 0xfa, 0xcf, 0x15, 0x91, 0x35, 0x61, 0xe7, 0xe5, 0xc7, 0x53, 0xc, 0x21, 0x4a,
        0x7f, 0x4c, 0x49, 0x7e, 0xf9, 0x36, 0x9c, 0x46, 0x56, 0xe1, 0xd2, 0x5d, 0xb7, 0x52, 0x63,
        0x57, 0x14, 0xd8, 0x13, 0xc5, 0x23, 0x57, 0x7e, 0x77, 0xb4, 0xbb, 0x5f, 0xe4, 0xb4, 0xb4,
        0x47, 0x96, 0x8, 0x99, 0x7a, 0xe7, 0x20, 0xae, 0xba, 0x43, 0x2a, 0x26, 0x2, 0x7d, 0x34,
        0x58, 0xc, 0x6f, 0x98, 0xe, 0x5f, 0x94, 0x79, 0x7c, 0x68, 0x8e, 0xe8, 0x78, 0x82, 0xcb,
        0x9f, 0xf2,
    ];

    fn setup_container() -> (Container<MemoryBackend>, Id) {
        let backend = MemoryBackend::new();
        let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
            .with_password_callback(|| Ok(b"abc".to_vec()))
            .with_kdf(Kdf::pbkdf2(Digest::Sha1, 65536, b"123"))
            .build::<MemoryBackend>()
            .unwrap();

        let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();
        let id = container.aquire().unwrap();

        (container, id)
    }

    write_tests!(512, full -> CTEXT_AES128_CTR, less -> LESS, empty -> EMPTY);
}

mod aes128_gcm {
    use nuts_backend::BlockId;

    use crate::container::tests::CTEXT_AES128_GCM;
    use crate::container::{Cipher, Container, CreateOptionsBuilder, Digest, Error, Kdf};
    use crate::memory::{Error as MemoryError, Id, MemoryBackend};
    use crate::tests::{into_error, RND};

    const LESS: [u8; 512] = [
        0x49, 0x6a, 0x82, 0xb1, 0x3d, 0x3d, 0x37, 0x5f, 0x8e, 0x9b, 0x18, 0x1c, 0x5b, 0x99, 0x64,
        0x25, 0x12, 0xf3, 0x33, 0x94, 0x82, 0x5e, 0x12, 0x21, 0x23, 0xb8, 0xb9, 0x95, 0x56, 0x7f,
        0xc, 0x3a, 0x4b, 0x7, 0x68, 0x1c, 0x36, 0xfa, 0x2f, 0xb5, 0x3b, 0xaf, 0xc1, 0xf3, 0x83,
        0xba, 0x1a, 0x14, 0x9d, 0xd3, 0x3f, 0x3a, 0x8c, 0x92, 0xb, 0x58, 0x8b, 0x37, 0xd0, 0x39,
        0x2c, 0xfe, 0xa8, 0xeb, 0xe3, 0x6c, 0x83, 0xf1, 0x9, 0x41, 0x9f, 0x6c, 0x50, 0xc6, 0x12,
        0xb4, 0xce, 0x13, 0x79, 0xb6, 0x12, 0x4f, 0x28, 0xf5, 0x79, 0xce, 0x29, 0x1f, 0x53, 0x92,
        0xd5, 0x51, 0xf8, 0xed, 0x5f, 0x74, 0xfe, 0xfe, 0x6c, 0x19, 0x5e, 0x86, 0x3c, 0xfc, 0x6b,
        0x1f, 0xe3, 0xb6, 0xdb, 0x50, 0xdf, 0x6d, 0x62, 0xd1, 0xf2, 0x4e, 0xdf, 0xbc, 0xcc, 0x74,
        0x47, 0xf8, 0x40, 0xe9, 0x4d, 0x39, 0x15, 0xa0, 0x90, 0x16, 0xc, 0x2c, 0xa1, 0x8b, 0x42,
        0xc2, 0x0, 0x6a, 0xa7, 0xe3, 0x83, 0xf0, 0xa3, 0x74, 0x8b, 0x87, 0x76, 0xa1, 0xdd, 0x3c,
        0xdb, 0xf6, 0x6e, 0xaa, 0x45, 0xd8, 0x4, 0x1a, 0x3b, 0x30, 0x1c, 0xb, 0xe3, 0xa5, 0x57,
        0x36, 0xc6, 0x2b, 0x23, 0x7d, 0x79, 0x99, 0x45, 0x89, 0xfa, 0xce, 0xc8, 0xdf, 0x61, 0x11,
        0xce, 0xd7, 0xa0, 0x29, 0xe5, 0xec, 0xf4, 0xff, 0x4, 0xeb, 0xed, 0xad, 0x0, 0xd3, 0xc1,
        0x20, 0x44, 0x22, 0x5f, 0x73, 0x3e, 0xa2, 0x7e, 0xe6, 0xfa, 0x52, 0xec, 0x27, 0x37, 0xf0,
        0x72, 0x18, 0xe7, 0x3c, 0x4d, 0x7c, 0x7c, 0xb8, 0x6d, 0xa9, 0x59, 0x13, 0xdd, 0x2f, 0xb6,
        0x30, 0xf8, 0x2c, 0x4f, 0x4e, 0xda, 0x29, 0x3, 0x64, 0x98, 0x6e, 0xa9, 0x5d, 0x6b, 0xa5,
        0xc6, 0x33, 0xb8, 0x10, 0x32, 0x24, 0x4b, 0xe6, 0xa0, 0x15, 0x89, 0x6f, 0x7a, 0x7b, 0x51,
        0xca, 0xf4, 0xba, 0x2e, 0xcb, 0x80, 0xd0, 0x6c, 0x5c, 0x3b, 0x95, 0x11, 0xc6, 0x8e, 0x9b,
        0x33, 0x11, 0xf4, 0x59, 0xde, 0xae, 0x45, 0x45, 0xb7, 0x6c, 0xfe, 0xc8, 0x9d, 0x75, 0xd,
        0xa5, 0xfa, 0x4b, 0x93, 0x2e, 0x46, 0xc4, 0x5d, 0x4f, 0x86, 0x41, 0x2e, 0x12, 0x1c, 0xb7,
        0xf4, 0x92, 0xf2, 0x1, 0x36, 0x95, 0x75, 0x3e, 0x1b, 0x8a, 0x71, 0x17, 0x9f, 0xe8, 0xa4,
        0x8b, 0xcd, 0xe3, 0x23, 0x66, 0x42, 0xb6, 0x48, 0xbd, 0xfc, 0xf7, 0x77, 0x71, 0x77, 0x78,
        0xf6, 0x6a, 0xfe, 0xfd, 0x0, 0x0, 0x8a, 0x29, 0xc0, 0x59, 0x41, 0xb4, 0xf1, 0xf7, 0xec,
        0xd8, 0x5, 0x28, 0xa8, 0xe3, 0xbe, 0x16, 0x6d, 0x81, 0x35, 0xc9, 0x6e, 0x6f, 0xb6, 0x0,
        0xbc, 0xd5, 0x3c, 0x36, 0x7c, 0x94, 0x36, 0x1f, 0x5c, 0x25, 0x80, 0xca, 0x1d, 0xe5, 0xec,
        0x30, 0x47, 0xbc, 0x10, 0x6e, 0x54, 0x7a, 0x51, 0x56, 0x6d, 0x6, 0x1b, 0x86, 0x8, 0x9f,
        0x9b, 0x1f, 0x38, 0x29, 0x17, 0xed, 0x61, 0xeb, 0xbb, 0xe8, 0xb2, 0xe5, 0xcb, 0xef, 0x58,
        0x8b, 0x4b, 0x9e, 0xc4, 0xe7, 0x85, 0xb, 0xb5, 0xe7, 0x5a, 0x7d, 0x2c, 0xfe, 0x9, 0x81,
        0xd4, 0xcc, 0x66, 0xde, 0x13, 0xbb, 0x4c, 0x9d, 0x83, 0x6b, 0x71, 0xf1, 0xc5, 0xe1, 0xf2,
        0x92, 0x8f, 0xf9, 0x38, 0x7, 0xfd, 0x28, 0x42, 0xcf, 0x6b, 0x1d, 0xe8, 0x23, 0x89, 0xe6,
        0xe3, 0x50, 0xa1, 0x5e, 0xac, 0x3c, 0x31, 0xe3, 0xb0, 0x74, 0xcc, 0x2d, 0xd6, 0xb8, 0xa3,
        0x22, 0x6c, 0xbd, 0x44, 0xd, 0x2c, 0xfe, 0x16, 0xd2, 0x79, 0x33, 0xb5, 0x6c, 0xcc, 0x72,
        0xe0, 0xd1, 0x6a, 0x8, 0x83, 0x89, 0x11, 0xc8, 0x72, 0x85, 0xc0, 0x14, 0x8f, 0x20, 0xc9,
        0x9f, 0x2, 0x47, 0xbf, 0x86, 0xfe, 0x60, 0x6f, 0xe8, 0xf2, 0x82, 0x70, 0x89, 0xa3, 0x68,
        0xef, 0x23,
    ];

    const EMPTY: [u8; 512] = [
        0xd8, 0xaa, 0x30, 0x7e, 0xda, 0xec, 0x29, 0xbc, 0x97, 0x8c, 0xdc, 0x54, 0xa1, 0x4c, 0x4b,
        0x15, 0xe8, 0xb9, 0x2d, 0x8, 0x57, 0xfa, 0x8f, 0x9f, 0x23, 0x74, 0xfb, 0x94, 0x4e, 0xca,
        0xb2, 0x35, 0x7, 0x76, 0xfd, 0xe9, 0xbd, 0xa0, 0x8e, 0xd, 0x14, 0xd0, 0x22, 0x8f, 0x55,
        0x38, 0x2b, 0xa1, 0x49, 0x5a, 0xc7, 0xf4, 0x50, 0x31, 0xa6, 0xa6, 0xf8, 0x58, 0x3c, 0xc5,
        0x46, 0xb5, 0x23, 0x90, 0x1c, 0x2e, 0xe0, 0xac, 0x77, 0xc8, 0x39, 0x98, 0xcb, 0x9, 0xfe,
        0xfc, 0xc1, 0x1b, 0x4e, 0x2c, 0x53, 0xac, 0xd6, 0x1, 0x33, 0xed, 0x9a, 0xfe, 0x88, 0x60,
        0xb7, 0xb3, 0x40, 0x3d, 0xf7, 0xd5, 0xac, 0x83, 0x8c, 0x11, 0xa5, 0xb4, 0x4a, 0x3e, 0x26,
        0x61, 0xe6, 0xe, 0x48, 0xd6, 0xf7, 0x15, 0xd0, 0x60, 0x72, 0x83, 0x2a, 0x11, 0x41, 0x90,
        0xfb, 0xe1, 0x68, 0x75, 0xae, 0x72, 0x94, 0xc2, 0xc9, 0x4b, 0xa3, 0xa5, 0xb, 0xc5, 0x4a,
        0x4b, 0xd9, 0x88, 0xa3, 0x66, 0x6b, 0x69, 0xf6, 0x30, 0x89, 0x74, 0xdb, 0xdd, 0x23, 0x4a,
        0x9c, 0x79, 0xbc, 0xb3, 0xff, 0x89, 0xa, 0xb1, 0x9a, 0x44, 0xa4, 0x1d, 0x99, 0xaf, 0xa5,
        0xc, 0x61, 0xf1, 0xe3, 0xfe, 0xba, 0xe7, 0x36, 0xe1, 0x27, 0x57, 0x20, 0xda, 0xe7, 0x63,
        0xec, 0x14, 0x38, 0xc4, 0xb, 0x7b, 0xd9, 0xfc, 0x51, 0x7a, 0xd9, 0x98, 0x84, 0x80, 0xc0,
        0xe5, 0x4a, 0x59, 0xf4, 0x47, 0xae, 0xf4, 0x92, 0x2c, 0x6c, 0x6d, 0xc5, 0xbf, 0x42, 0x32,
        0x77, 0x8a, 0x7a, 0xb2, 0x30, 0x61, 0x34, 0x2c, 0x3f, 0x6c, 0xde, 0x6e, 0x8d, 0xeb, 0x32,
        0x2c, 0xe8, 0x83, 0xa5, 0x10, 0x5f, 0x68, 0x8d, 0x58, 0xac, 0xa4, 0x75, 0x1, 0xa1, 0x2f,
        0x2b, 0x3a, 0x93, 0x4e, 0x28, 0xaa, 0xa8, 0x2e, 0xe7, 0xf1, 0xa1, 0x76, 0x96, 0xce, 0x57,
        0x56, 0x65, 0xda, 0x5d, 0xb, 0x0, 0x93, 0x3c, 0x5c, 0x63, 0x29, 0x6c, 0xa7, 0xc, 0xf5,
        0xb1, 0xe, 0xb3, 0x7e, 0xa2, 0x19, 0x2e, 0xda, 0x9d, 0x57, 0xe3, 0xd7, 0x69, 0x4c, 0x3f,
        0x29, 0xe2, 0x58, 0x4c, 0xb0, 0xbe, 0x5, 0xf8, 0xb5, 0x3d, 0x6, 0x59, 0x9b, 0x3d, 0xfc,
        0x61, 0x36, 0x7d, 0x91, 0x99, 0xbe, 0x8f, 0x7f, 0x40, 0x5, 0x7, 0xd0, 0x21, 0xdc, 0x17,
        0xa1, 0x39, 0x16, 0x28, 0x9d, 0x15, 0x15, 0x19, 0x3a, 0x29, 0xa1, 0x66, 0xed, 0x25, 0xea,
        0x4a, 0x7c, 0x8d, 0x3b, 0x26, 0xef, 0x45, 0x3f, 0xb0, 0x2a, 0x72, 0xd1, 0x1e, 0x3, 0xde,
        0x9d, 0x91, 0xf8, 0x5a, 0x11, 0xa3, 0x71, 0x19, 0x34, 0xc7, 0x96, 0x8c, 0xc0, 0x49, 0xba,
        0x4b, 0xd2, 0x83, 0x49, 0xfa, 0x71, 0x4b, 0xbe, 0x70, 0xdd, 0x5a, 0xa3, 0x11, 0xe2, 0xa3,
        0x67, 0x9d, 0xd8, 0x26, 0x9, 0xa5, 0x56, 0x8, 0x30, 0x88, 0xd7, 0xc4, 0xaf, 0x61, 0x5d,
        0xfb, 0xf6, 0xf9, 0x38, 0x6, 0xe7, 0x9e, 0x70, 0xf8, 0xf3, 0x5c, 0x9c, 0x57, 0x91, 0x88,
        0xb1, 0x78, 0xf8, 0x2a, 0xb2, 0x2f, 0xf4, 0xb5, 0x36, 0x40, 0x1b, 0xa4, 0x11, 0xc5, 0x75,
        0xc5, 0x60, 0xcc, 0xb8, 0x8a, 0x16, 0xf7, 0xfb, 0xd6, 0xf2, 0x60, 0x79, 0xa3, 0x2a, 0x51,
        0x89, 0x7b, 0x53, 0xd3, 0xb9, 0x36, 0xd7, 0x17, 0x12, 0x1c, 0x1d, 0x6e, 0xd0, 0xf0, 0x30,
        0x7a, 0x80, 0xd8, 0x5f, 0x66, 0x1a, 0x5a, 0x2f, 0x35, 0x56, 0x33, 0x8f, 0x73, 0x71, 0x91,
        0xff, 0xda, 0x60, 0xe3, 0x58, 0x69, 0x45, 0x26, 0xa5, 0x41, 0xbb, 0x25, 0x93, 0xd6, 0x71,
        0x40, 0x9f, 0x3f, 0x5d, 0x73, 0xe6, 0x88, 0xfb, 0xd8, 0x35, 0x82, 0x1d, 0x7f, 0xdf, 0x5,
        0x9f, 0x6e, 0xdf, 0xc5, 0xb7, 0xc3, 0xdc, 0x44, 0xca, 0x2, 0x95, 0x7e, 0xe3, 0x35, 0x5a,
        0xaa, 0xe7,
    ];

    fn setup_container() -> (Container<MemoryBackend>, Id) {
        let backend = MemoryBackend::new();
        let options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
            .with_password_callback(|| Ok(b"abc".to_vec()))
            .with_kdf(Kdf::pbkdf2(Digest::Sha1, 65536, b"123"))
            .build::<MemoryBackend>()
            .unwrap();

        let mut container = Container::<MemoryBackend>::create(backend, options).unwrap();
        let id = container.aquire().unwrap();

        (container, id)
    }

    write_tests!(496, full -> CTEXT_AES128_GCM, less -> LESS, empty -> EMPTY);
}

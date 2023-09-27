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

macro_rules! read_tests {
    ($size:literal $(, $args:expr)*) => {
        #[test]
        fn full() {
            let (mut container, id) = setup_container($($args),*);
            let mut buf = [0; $size];

            let n = container.read(&id, &mut buf).unwrap();
            assert_eq!(n, $size);
            assert_eq!(buf, RND[..$size]);
        }

        #[test]
        fn less() {
            let (mut container, id) = setup_container($($args),*);
            let mut buf = [0; $size - 1];

            let n = container.read(&id, &mut buf).unwrap();
            assert_eq!(n, $size - 1);
            assert_eq!(buf, RND[..$size - 1]);
        }

        #[test]
        fn empty() {
            let (mut container, id) = setup_container($($args),*);

            let n = container.read(&id, &mut []).unwrap();
            assert_eq!(n, 0);
        }

        #[test]
        fn more() {
            let (mut container, id) = setup_container($($args),*);
            let mut buf = [b'x'; $size + 1];

            let n = container.read(&id, &mut buf).unwrap();
            assert_eq!(n, $size);
            assert_eq!(buf, [&RND[..$size], &[b'x']].concat()[..]);
        }

        #[test]
        fn null() {
            let (mut container, _) = setup_container($($args),*);

            let err = container.read(&Id::null(), &mut []).unwrap_err();
            assert!(matches!(err, Error::NullId));
        }

        #[test]
        fn no_such_id() {
            let (mut container, id) = setup_container($($args),*);

            container.release(id.clone()).unwrap();

            let err = container.read(&id, &mut []).unwrap_err();

            let err = into_error!(err, Error::Backend);
            let err_id = into_error!(err, MemoryError::NoSuchId);
            assert_eq!(err_id, id);
        }
    };
}

mod none {
    use crate::backend::BlockId;
    use crate::container::{Cipher, Container, CreateOptionsBuilder, Error};
    use crate::memory::{Error as MemoryError, Id, MemoryBackend};
    use crate::tests::{into_error, RND};

    fn setup_container() -> (Container<MemoryBackend>, Id) {
        let mut backend = MemoryBackend::new();

        let id = backend.insert_data(&RND).unwrap();

        let options = CreateOptionsBuilder::new(Cipher::None)
            .build::<MemoryBackend>()
            .unwrap();
        let container = Container::<MemoryBackend>::create(backend, options).unwrap();

        (container, id)
    }

    read_tests!(512);
}

mod aes128_ctr {
    use crate::backend::BlockId;
    use crate::container::tests::CTEXT_AES128_CTR;
    use crate::container::{Cipher, Container, CreateOptionsBuilder, Digest, Error, Kdf};
    use crate::memory::{Error as MemoryError, Id, MemoryBackend};
    use crate::tests::{into_error, RND};

    fn setup_container() -> (Container<MemoryBackend>, Id) {
        let mut backend = MemoryBackend::new();

        let id = backend.insert_data(&CTEXT_AES128_CTR).unwrap();

        let options = CreateOptionsBuilder::new(Cipher::Aes128Ctr)
            .with_password_callback(|| Ok(b"abc".to_vec()))
            .with_kdf(Kdf::pbkdf2(Digest::Sha1, 65536, b"123"))
            .build::<MemoryBackend>()
            .unwrap();
        let container = Container::<MemoryBackend>::create(backend, options).unwrap();

        (container, id)
    }

    read_tests!(512);
}

mod aes128_gcm {
    use crate::backend::BlockId;
    use crate::container::tests::CTEXT_AES128_GCM;
    use crate::container::{Cipher, Container, CreateOptionsBuilder, Digest, Error, Kdf};
    use crate::memory::{Error as MemoryError, Id, MemoryBackend};
    use crate::tests::{into_error, RND};

    fn setup_container(data: &[u8]) -> (Container<MemoryBackend>, Id) {
        let mut backend = MemoryBackend::new();

        let id = backend.insert_data(data).unwrap();

        let options = CreateOptionsBuilder::new(Cipher::Aes128Gcm)
            .with_password_callback(|| Ok(b"abc".to_vec()))
            .with_kdf(Kdf::pbkdf2(Digest::Sha1, 65536, b"123"))
            .build::<MemoryBackend>()
            .unwrap();
        let container = Container::<MemoryBackend>::create(backend, options).unwrap();

        (container, id)
    }

    read_tests!(496, &CTEXT_AES128_GCM);

    #[test]
    fn not_trustworthy() {
        let data = [&[CTEXT_AES128_GCM[0] + 1], &CTEXT_AES128_GCM[1..]].concat();
        let (mut container, id) = setup_container(&data);
        let mut buf = [0; 496];

        let err = container.read(&id, &mut buf).unwrap_err();
        assert!(matches!(err, Error::NotTrustworthy));
    }
}

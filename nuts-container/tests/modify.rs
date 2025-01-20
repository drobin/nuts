// MIT License
//
// Copyright (c) 2024,2025 Robin Doer
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

use nuts_container::{Container, ContainerResult, Error, ModifyOptionsBuilder, OpenOptionsBuilder};
use nuts_memory::MemoryBackend;
use std::fs::File;

use crate::common::fixture_path;

const OLD_PW: &[u8] = b"sample";
const NEW_PW: &[u8] = b"123";

fn open_backend_from_fixture(dir: &str, name: &str) -> MemoryBackend {
    let path = fixture_path(dir, name);
    let file = File::open(path).unwrap();

    serde_json::from_reader(file).unwrap()
}

fn open_container(
    backend: MemoryBackend,
    password: &'static [u8],
) -> ContainerResult<Container<MemoryBackend>, MemoryBackend> {
    let options = OpenOptionsBuilder::new()
        .with_password_callback(move || Ok(password.to_vec()))
        .build::<MemoryBackend>()
        .unwrap();

    Container::open(backend, options)
}

fn backend_with_changed_password(name: &str) -> MemoryBackend {
    let backend = open_backend_from_fixture("compat", name);
    let mut container = open_container(backend, OLD_PW).unwrap();

    let options = ModifyOptionsBuilder::default()
        .change_password(|| Ok(NEW_PW.to_vec()))
        .build();

    container.modify(options).unwrap();
    container.into_backend()
}

fn backend_with_changed_password_err(name: &str) -> Error<MemoryBackend> {
    let backend = backend_with_changed_password(name);
    open_container(backend, OLD_PW).unwrap_err()
}

macro_rules! password_test {
    ($name:ident, $json:literal, $cipher:ident) => {
        #[test]
        fn $name() {
            let backend = backend_with_changed_password($json);
            let container = open_container(backend, NEW_PW).unwrap();
            let info = container.info().unwrap();

            assert_eq!(info.revision, REVISION);
            assert_eq!(info.cipher, Cipher::$cipher);
        }
    };
}

macro_rules! old_password_test {
    ($name:ident ( none ), $json:literal) => {
        #[test]
        fn $name() {
            let backend = backend_with_changed_password($json);
            let container = open_container(backend, OLD_PW).unwrap();
            let info = container.info().unwrap();

            assert_eq!(info.revision, REVISION);
            assert_eq!(info.cipher, Cipher::None);
        }
    };

    ($name:ident, $json:literal) => {
        #[test]
        fn $name() {
            let err = backend_with_changed_password_err($json);

            assert!(matches!(err, Error::Header(cause)
                if matches!(cause, HeaderError::WrongPassword)));
        }
    };
}

macro_rules! kdf_test {
    ($name:ident, $json:literal, $cipher:ident) => {
        #[test]
        fn $name() {
            let backend = {
                let backend = open_backend_from_fixture("compat", $json);
                let mut container = open_container(backend, OLD_PW).unwrap();
                let info = container.info().unwrap();

                assert_eq!(info.revision, REVISION);
                assert_eq!(info.cipher, Cipher::$cipher);

                if info.cipher == Cipher::None {
                    assert_eq!(info.kdf, Kdf::None);
                } else {
                    assert!(matches!(info.kdf, Kdf::Pbkdf2 { digest, iterations, salt }
                        if digest == DIGEST && iterations == 65536 && salt != b"123"
                    ));
                }

                let options = ModifyOptionsBuilder::default()
                    .change_kdf(Kdf::pbkdf2(Digest::Sha512, 4711, b"123"))
                    .build();
                container.modify(options).unwrap();

                container.into_backend()
            };

            let container = open_container(backend, OLD_PW).unwrap();
            let info = container.info().unwrap();

            assert_eq!(info.revision, REVISION);
            assert_eq!(info.cipher, Cipher::$cipher);

            if info.cipher == Cipher::None {
                assert_eq!(info.kdf, Kdf::None);
            } else {
                assert_eq!(info.kdf, Kdf::pbkdf2(Digest::Sha512, 4711, b"123"));
            }
        }
    };
}

mod v_0_6_8 {
    use nuts_container::{Cipher, Digest, Error, HeaderError, Kdf, ModifyOptionsBuilder};

    use crate::{
        backend_with_changed_password, backend_with_changed_password_err,
        open_backend_from_fixture, open_container, NEW_PW, OLD_PW,
    };

    const REVISION: u32 = 0;
    const DIGEST: Digest = Digest::Sha1;

    password_test!(password_none, "0.6.8-none.json", None);
    old_password_test!(old_password_none(none), "0.6.8-none.json");
    kdf_test!(kdf_none, "0.6.8-none.json", None);

    password_test!(password_aes128ctr, "0.6.8-aes128ctr.json", Aes128Ctr);
    old_password_test!(old_password_aes128ctr, "0.6.8-aes128ctr.json");
    kdf_test!(kdf_aes128ctr, "0.6.8-aes128ctr.json", Aes128Ctr);

    password_test!(password_aes128gcm, "0.6.8-aes128gcm.json", Aes128Gcm);
    old_password_test!(old_password_aes128gcm, "0.6.8-aes128gcm.json");
    kdf_test!(kdf_aes128gcm, "0.6.8-aes128gcm.json", Aes128Gcm);
}

mod v_0_7_0 {
    use nuts_container::{Cipher, Digest, Error, HeaderError, Kdf, ModifyOptionsBuilder};

    use crate::{
        backend_with_changed_password, backend_with_changed_password_err,
        open_backend_from_fixture, open_container, NEW_PW, OLD_PW,
    };

    const REVISION: u32 = 1;
    const DIGEST: Digest = Digest::Sha256;

    password_test!(password_none, "0.7.0-none.json", None);
    old_password_test!(old_password_none(none), "0.7.0-none.json");
    kdf_test!(kdf_none, "0.7.0-none.json", None);

    password_test!(password_aes128ctr, "0.7.0-aes128ctr.json", Aes128Ctr);
    old_password_test!(old_password_aes128ctr, "0.7.0-aes128ctr.json");
    kdf_test!(kdf_aes128ctr, "0.7.0-aes128ctr.json", Aes128Ctr);

    password_test!(password_aes128gcm, "0.7.0-aes128gcm.json", Aes128Gcm);
    old_password_test!(old_password_aes128gcm, "0.7.0-aes128gcm.json");
    kdf_test!(kdf_aes128gcm, "0.7.0-aes128gcm.json", Aes128Gcm);

    password_test!(password_aes192ctr, "0.7.0-aes192ctr.json", Aes192Ctr);
    old_password_test!(old_password_aes192ctr, "0.7.0-aes192ctr.json");
    kdf_test!(kdf_aes192ctr, "0.7.0-aes192ctr.json", Aes192Ctr);

    password_test!(password_aes192gcm, "0.7.0-aes192gcm.json", Aes192Gcm);
    old_password_test!(old_password_aes192gcm, "0.7.0-aes192gcm.json");
    kdf_test!(kdf_aes192gcm, "0.7.0-aes192gcm.json", Aes192Gcm);

    password_test!(password_aes256ctr, "0.7.0-aes256ctr.json", Aes256Ctr);
    old_password_test!(old_password_aes256ctr, "0.7.0-aes256ctr.json");
    kdf_test!(kdf_aes256ctr, "0.7.0-aes256ctr.json", Aes256Ctr);

    password_test!(password_aes256gcm, "0.7.0-aes256gcm.json", Aes256Gcm);
    old_password_test!(old_password_aes256gcm, "0.7.0-aes256gcm.json");
    kdf_test!(kdf_aes256gcm, "0.7.0-aes256gcm.json", Aes256Gcm);
}

mod v_0_7_1 {
    use nuts_container::{Cipher, Digest, Error, HeaderError, Kdf, ModifyOptionsBuilder};

    use crate::{
        backend_with_changed_password, backend_with_changed_password_err,
        open_backend_from_fixture, open_container, NEW_PW, OLD_PW,
    };

    const REVISION: u32 = 1;
    const DIGEST: Digest = Digest::Sha256;

    password_test!(password_none, "0.7.1-none.json", None);
    old_password_test!(old_password_none(none), "0.7.1-none.json");
    kdf_test!(kdf_none, "0.7.1-none.json", None);

    password_test!(password_aes128ctr, "0.7.1-aes128ctr.json", Aes128Ctr);
    old_password_test!(old_password_aes128ctr, "0.7.1-aes128ctr.json");
    kdf_test!(kdf_aes128ctr, "0.7.1-aes128ctr.json", Aes128Ctr);

    password_test!(password_aes128gcm, "0.7.1-aes128gcm.json", Aes128Gcm);
    old_password_test!(old_password_aes128gcm, "0.7.1-aes128gcm.json");
    kdf_test!(kdf_aes128gcm, "0.7.1-aes128gcm.json", Aes128Gcm);

    password_test!(password_aes192ctr, "0.7.1-aes192ctr.json", Aes192Ctr);
    old_password_test!(old_password_aes192ctr, "0.7.1-aes192ctr.json");
    kdf_test!(kdf_aes192ctr, "0.7.1-aes192ctr.json", Aes192Ctr);

    password_test!(password_aes192gcm, "0.7.1-aes192gcm.json", Aes192Gcm);
    old_password_test!(old_password_aes192gcm, "0.7.1-aes192gcm.json");
    kdf_test!(kdf_aes192gcm, "0.7.1-aes192gcm.json", Aes192Gcm);

    password_test!(password_aes256ctr, "0.7.1-aes256ctr.json", Aes256Ctr);
    old_password_test!(old_password_aes256ctr, "0.7.1-aes256ctr.json");
    kdf_test!(kdf_aes256ctr, "0.7.1-aes256ctr.json", Aes256Ctr);

    password_test!(password_aes256gcm, "0.7.1-aes256gcm.json", Aes256Gcm);
    old_password_test!(old_password_aes256gcm, "0.7.1-aes256gcm.json");
    kdf_test!(kdf_aes256gcm, "0.7.1-aes256gcm.json", Aes256Gcm);
}

mod v_0_7_3 {
    use nuts_container::{Cipher, Digest, Error, HeaderError, Kdf, ModifyOptionsBuilder};

    use crate::{
        backend_with_changed_password, backend_with_changed_password_err,
        open_backend_from_fixture, open_container, NEW_PW, OLD_PW,
    };

    const REVISION: u32 = 2;
    const DIGEST: Digest = Digest::Sha256;

    password_test!(password_none, "0.7.3-none.json", None);
    old_password_test!(old_password_none(none), "0.7.3-none.json");
    kdf_test!(kdf_none, "0.7.3-none.json", None);

    password_test!(password_aes128ctr, "0.7.3-aes128ctr.json", Aes128Ctr);
    old_password_test!(old_password_aes128ctr, "0.7.3-aes128ctr.json");
    kdf_test!(kdf_aes128ctr, "0.7.3-aes128ctr.json", Aes128Ctr);

    password_test!(password_aes128gcm, "0.7.3-aes128gcm.json", Aes128Gcm);
    old_password_test!(old_password_aes128gcm, "0.7.3-aes128gcm.json");
    kdf_test!(kdf_aes128gcm, "0.7.3-aes128gcm.json", Aes128Gcm);

    password_test!(password_aes192ctr, "0.7.3-aes192ctr.json", Aes192Ctr);
    old_password_test!(old_password_aes192ctr, "0.7.3-aes192ctr.json");
    kdf_test!(kdf_aes192ctr, "0.7.3-aes192ctr.json", Aes192Ctr);

    password_test!(password_aes192gcm, "0.7.3-aes192gcm.json", Aes192Gcm);
    old_password_test!(old_password_aes192gcm, "0.7.3-aes192gcm.json");
    kdf_test!(kdf_aes192gcm, "0.7.3-aes192gcm.json", Aes192Gcm);

    password_test!(password_aes256ctr, "0.7.3-aes256ctr.json", Aes256Ctr);
    old_password_test!(old_password_aes256ctr, "0.7.3-aes256ctr.json");
    kdf_test!(kdf_aes256ctr, "0.7.3-aes256ctr.json", Aes256Ctr);

    password_test!(password_aes256gcm, "0.7.3-aes256gcm.json", Aes256Gcm);
    old_password_test!(old_password_aes256gcm, "0.7.3-aes256gcm.json");
    kdf_test!(kdf_aes256gcm, "0.7.3-aes256gcm.json", Aes256Gcm);
}

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

use nuts_container::{
    Cipher, CipherError, Container, ContainerResult, Error, HeaderError, ModifyOptionsBuilder,
    OpenOptionsBuilder,
};
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

            assert_eq!(container.info().unwrap().cipher, Cipher::$cipher);
        }
    };
}

macro_rules! old_password_test {
    ($name:ident ( none ), $json:literal) => {
        #[test]
        fn $name() {
            let backend = backend_with_changed_password($json);
            let container = open_container(backend, OLD_PW).unwrap();

            assert_eq!(container.info().unwrap().cipher, Cipher::None);
        }
    };

    ($name:ident ( wrong ), $json:literal) => {
        #[test]
        fn $name() {
            let err = backend_with_changed_password_err($json);

            assert!(matches!(err, Error::Header(cause)
                if matches!(cause, HeaderError::WrongPassword)));
        }
    };

    ($name:ident ( trust ), $json:literal) => {
        #[test]
        fn $name() {
            let err = backend_with_changed_password_err($json);

            assert!(matches!(err, Error::Header(ref cause)
                if matches!(cause, HeaderError::Cipher(cause)
                    if matches!(cause, CipherError::NotTrustworthy))));
        }
    };
}

password_test!(password_0_6_8_none, "0.6.8-none.json", None);
old_password_test!(old_password_0_6_8_none(none), "0.6.8-none.json");
password_test!(password_0_6_8_aes128ctr, "0.6.8-aes128ctr.json", Aes128Ctr);
old_password_test!(old_password_0_6_8_aes128ctr(wrong), "0.6.8-aes128ctr.json");
password_test!(password_0_6_8_aes128gcm, "0.6.8-aes128gcm.json", Aes128Gcm);
old_password_test!(old_password_0_6_8_aes128gcm(trust), "0.6.8-aes128gcm.json");

password_test!(password_0_7_0_none, "0.7.0-none.json", None);
old_password_test!(old_password_0_7_0_none(none), "0.7.0-none.json");
password_test!(password_0_7_0_aes128ctr, "0.7.0-aes128ctr.json", Aes128Ctr);
old_password_test!(old_password_0_7_0_aes128ctr(wrong), "0.7.0-aes128ctr.json");
password_test!(password_0_7_0_aes128gcm, "0.7.0-aes128gcm.json", Aes128Gcm);
old_password_test!(old_password_0_7_0_aes128gcm(trust), "0.7.0-aes128gcm.json");
password_test!(password_0_7_0_aes192ctr, "0.7.0-aes192ctr.json", Aes192Ctr);
old_password_test!(old_password_0_7_0_aes192ctr(wrong), "0.7.0-aes192ctr.json");
password_test!(password_0_7_0_aes192gcm, "0.7.0-aes192gcm.json", Aes192Gcm);
old_password_test!(old_password_0_7_0_aes192gcm(trust), "0.7.0-aes192gcm.json");
password_test!(password_0_7_0_aes256ctr, "0.7.0-aes256ctr.json", Aes256Ctr);
old_password_test!(old_password_0_7_0_aes256ctr(wrong), "0.7.0-aes256ctr.json");
password_test!(password_0_7_0_aes256gcm, "0.7.0-aes256gcm.json", Aes256Gcm);
old_password_test!(old_password_0_7_0_aes256gcm(trust), "0.7.0-aes256gcm.json");

password_test!(password_0_7_1_none, "0.7.1-none.json", None);
old_password_test!(old_password_0_7_1_none(none), "0.7.1-none.json");
password_test!(password_0_7_1_aes128ctr, "0.7.1-aes128ctr.json", Aes128Ctr);
old_password_test!(old_password_0_7_1_aes128ctr(wrong), "0.7.1-aes128ctr.json");
password_test!(password_0_7_1_aes128gcm, "0.7.1-aes128gcm.json", Aes128Gcm);
old_password_test!(old_password_0_7_1_aes128gcm(trust), "0.7.1-aes128gcm.json");
password_test!(password_0_7_1_aes192ctr, "0.7.1-aes192ctr.json", Aes192Ctr);
old_password_test!(old_password_0_7_1_aes192ctr(wrong), "0.7.1-aes192ctr.json");
password_test!(password_0_7_1_aes192gcm, "0.7.1-aes192gcm.json", Aes192Gcm);
old_password_test!(old_password_0_7_1_aes192gcm(trust), "0.7.1-aes192gcm.json");
password_test!(password_0_7_1_aes256ctr, "0.7.1-aes256ctr.json", Aes256Ctr);
old_password_test!(old_password_0_7_1_aes256ctr(wrong), "0.7.1-aes256ctr.json");
password_test!(password_0_7_1_aes256gcm, "0.7.1-aes256gcm.json", Aes256Gcm);
old_password_test!(old_password_0_7_1_aes256gcm(trust), "0.7.1-aes256gcm.json");

password_test!(password_0_7_3_none, "0.7.3-none.json", None);
old_password_test!(old_password_0_7_3_none(none), "0.7.3-none.json");
password_test!(password_0_7_3_aes128ctr, "0.7.3-aes128ctr.json", Aes128Ctr);
old_password_test!(old_password_0_7_3_aes128ctr(wrong), "0.7.3-aes128ctr.json");
password_test!(password_0_7_3_aes128gcm, "0.7.3-aes128gcm.json", Aes128Gcm);
old_password_test!(old_password_0_7_3_aes128gcm(trust), "0.7.3-aes128gcm.json");
password_test!(password_0_7_3_aes192ctr, "0.7.3-aes192ctr.json", Aes192Ctr);
old_password_test!(old_password_0_7_3_aes192ctr(wrong), "0.7.3-aes192ctr.json");
password_test!(password_0_7_3_aes192gcm, "0.7.3-aes192gcm.json", Aes192Gcm);
old_password_test!(old_password_0_7_3_aes192gcm(trust), "0.7.3-aes192gcm.json");
password_test!(password_0_7_3_aes256ctr, "0.7.3-aes256ctr.json", Aes256Ctr);
old_password_test!(old_password_0_7_3_aes256ctr(wrong), "0.7.3-aes256ctr.json");
password_test!(password_0_7_3_aes256gcm, "0.7.3-aes256gcm.json", Aes256Gcm);
old_password_test!(old_password_0_7_3_aes256gcm(trust), "0.7.3-aes256gcm.json");

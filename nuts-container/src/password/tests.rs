// MIT License
//
// Copyright (c) 2022-2024 Robin Doer
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

use std::rc::Rc;

use crate::password::{PasswordError, PasswordStore};

#[test]
fn with_value() {
    let mut store = PasswordStore::with_value(&[1, 2, 3]);
    assert_eq!(store.value().unwrap(), [1, 2, 3]);
}

#[test]
fn no_callback() {
    let mut store = PasswordStore::new(None);
    let err = store.value().unwrap_err();
    assert!(matches!(err, PasswordError::NoPassword));
}

#[test]
fn error_from_callback() {
    let mut store = PasswordStore::new(Some(Rc::new(|| Err(String::from("some error")))));

    let err = store.value().unwrap_err();
    assert!(matches!(err, PasswordError::PasswordCallback(msg) if msg == "some error"));
}

#[test]
fn value_from_callback() {
    let mut store = PasswordStore::new(Some(Rc::new(|| Ok(vec![1, 2, 3]))));

    let value1 = store.value().unwrap();
    assert_eq!(value1, [1, 2, 3]);

    {
        let value2 = store.value().unwrap();
        assert_eq!(value2, [1, 2, 3]);
    }
}

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

use bson::de::Error;
use bson::doc;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::info::PluginInfo;
use crate::tool::PluginError;

#[test]
fn de_rev0() {
    let doc = doc! { "name": "foo", "version": "xxx" };
    let info: PluginInfo = bson::from_document(doc).unwrap();

    assert_eq!(info.name, "foo");
    assert_eq!(info.version, "xxx");
    assert_eq!(info.revision, 0);
}

#[test]
fn de_explicit_rev0() {
    let doc = doc! { "name": "foo", "version": "xxx", "revision": 0 };
    let err = bson::from_document::<PluginInfo>(doc).unwrap_err();

    match err {
        Error::DeserializationError { message, .. } => {
            assert_eq!(message, "revision 0 cannot be explicity specified")
        }
        _ => panic!("invalid error"),
    }
}

#[test]
fn de_rev1() {
    let doc = doc! { "name": "foo", "version": "xxx", "revision": 1 };
    let info: PluginInfo = bson::from_document(doc).unwrap();

    assert_eq!(info.name, "foo");
    assert_eq!(info.version, "xxx");
    assert_eq!(info.revision, 1);
}

#[test]
fn ser() {
    let info = PluginInfo::new("foo", "xxx");
    let doc = bson::to_document(&info).unwrap();

    assert_eq!(doc.len(), 3);
    assert_eq!(doc.get_str("name").unwrap(), "foo");
    assert_eq!(doc.get_str("version").unwrap(), "xxx");
    assert_eq!(doc.get_i64("revision").unwrap(), 1);
}

#[test]
fn try_from_rev0() {
    let map: HashMap<_, _> = [
        ("name".to_string(), "foo".to_string()),
        ("version".to_string(), "xxx".to_string()),
    ]
    .into();
    let info: PluginInfo = TryFrom::try_from(map).unwrap();

    assert_eq!(info.name, "foo");
    assert_eq!(info.version, "xxx");
    assert_eq!(info.revision, 0);
}

#[test]
fn try_from_rev1() {
    let map: HashMap<_, _> = [
        ("name".to_string(), "foo".to_string()),
        ("version".to_string(), "xxx".to_string()),
        ("revision".to_string(), "1".to_string()),
    ]
    .into();
    let info: PluginInfo = TryFrom::try_from(map).unwrap();

    assert_eq!(info.name, "foo");
    assert_eq!(info.version, "xxx");
    assert_eq!(info.revision, 1);
}

#[test]
fn try_from_no_name() {
    let map: HashMap<_, _> = [("version".to_string(), "xxx".to_string())].into();
    let err = <PluginInfo as TryFrom<HashMap<_, _>>>::try_from(map).unwrap_err();

    assert!(matches!(err, PluginError::InvalidResponse))
}

#[test]
fn try_from_no_version() {
    let map: HashMap<_, _> = [("name".to_string(), "foo".to_string())].into();
    let err = <PluginInfo as TryFrom<HashMap<_, _>>>::try_from(map).unwrap_err();

    assert!(matches!(err, PluginError::InvalidResponse))
}

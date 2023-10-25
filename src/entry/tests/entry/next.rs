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

use crate::tests::{setup_container, setup_container_with_bsize};
use crate::Archive;

const BYTES: [u8; 160] = [
    0xae, 0x0e, 0xed, 0x65, 0xeb, 0x02, 0x5c, 0x36, 0x14, 0x61, 0xa9, 0x09, 0xbc, 0x9f, 0x6e, 0xa2,
    0x48, 0xd3, 0x08, 0xca, 0xa6, 0xf8, 0x57, 0x38, 0x81, 0xe8, 0x3f, 0xfb, 0x0c, 0x1a, 0x78, 0xc3,
    0xf1, 0x35, 0xc6, 0x48, 0x25, 0x31, 0x41, 0xdf, 0x16, 0xc6, 0xa5, 0xeb, 0xab, 0xe9, 0x61, 0x58,
    0x64, 0xbc, 0xb2, 0x9b, 0x1a, 0x96, 0xf8, 0xdc, 0x5d, 0xa9, 0x06, 0xda, 0x89, 0xa2, 0x4f, 0x94,
    0x8a, 0x2b, 0x0b, 0x6c, 0x8f, 0x5f, 0x8d, 0x7c, 0x45, 0x0a, 0xe9, 0x8d, 0xf3, 0x8f, 0xe5, 0xf5,
    0xe3, 0x2b, 0xab, 0xe2, 0xc1, 0xb5, 0xd1, 0x0b, 0xa3, 0x70, 0xa6, 0xfb, 0xf4, 0xa9, 0xc5, 0xe5,
    0xda, 0x5b, 0x74, 0x01, 0x3a, 0x5e, 0x93, 0x40, 0x46, 0x77, 0x2e, 0xcc, 0x14, 0xec, 0x58, 0xfd,
    0x9d, 0x60, 0x51, 0x73, 0x38, 0xe2, 0x45, 0xdd, 0xf9, 0xb6, 0xca, 0x09, 0xe1, 0x67, 0x05, 0x34,
    0xdf, 0xc2, 0x33, 0x02, 0x5f, 0x9a, 0x69, 0x5f, 0xc8, 0x1e, 0xf0, 0x7d, 0x59, 0xe6, 0xd1, 0x6f,
    0x60, 0x1f, 0x02, 0x4a, 0x76, 0xea, 0xd0, 0x9f, 0x56, 0x03, 0xf6, 0xc6, 0xc4, 0x9b, 0xe6, 0x6c,
];

macro_rules! assert_entry {
    ($entry:expr, $name:literal, $size:literal) => {{
        assert_eq!($entry.name(), $name);
        assert_eq!($entry.size(), $size);

        let data = $entry.read_vec().unwrap();
        assert_eq!(data.len(), $size);
        assert_eq!(data, BYTES[..$size]);
    }};
}

macro_rules! mk_test {
    ($name:ident $(, ( $fname:literal, $nbytes:literal ) )* $( -> ( $last_fname:literal, $last_nbytes:literal ) )+ ) => {
        #[test]
        fn $name() {
            let mut archive = Archive::create(setup_container_with_bsize(76), false).unwrap();

            $(
                let mut entry = archive.append($fname).build().unwrap();
                entry.write_all(&BYTES[..$nbytes]).unwrap();
            )*

            $(
                let mut entry = archive.append($last_fname).build().unwrap();
                entry.write_all(&BYTES[..$last_nbytes]).unwrap();
            )*

            let mut entry = archive.first().unwrap().unwrap();

            $(
                assert_entry!(entry, $fname, $nbytes);
                let mut entry = entry.next().unwrap().unwrap();
            )*

            $(
                assert_entry!(entry, $last_fname, $last_nbytes);
                assert!(entry.next().is_none());
            )*
        }
    };
}

#[test]
fn t0() {
    let mut archive = Archive::create(setup_container(), false).unwrap();
    assert!(archive.first().is_none());
}

mk_test!(t1_0 -> ("f1", 0));
mk_test!(t1_38 -> ("f1", 38));
mk_test!(t1_76 -> ("f1", 76));
mk_test!(t1_114 -> ("f1", 114));
mk_test!(t1_152 -> ("f1", 152));
mk_test!(t2_0, ("f1", 0) -> ("f2", 0));
mk_test!(t2_38, ("f1", 38) -> ("f2", 38));
mk_test!(t2_76, ("f1", 76) -> ("f2", 76));
mk_test!(t2_114, ("f1", 114) -> ("f2", 114));
mk_test!(t2_152, ("f1", 152) -> ("f2", 152));
mk_test!(t3_0, ("f1", 0), ("f2", 0) -> ("f3", 0));
mk_test!(t3_38, ("f1", 38), ("f2", 0) -> ("f3", 38));
mk_test!(t3_76, ("f1", 76), ("f2", 0) -> ("f3", 76));
mk_test!(t3_114, ("f1", 114), ("f2", 0) -> ("f3", 114));
mk_test!(t3_152, ("f1", 152), ("f2", 0) -> ("f3", 152));

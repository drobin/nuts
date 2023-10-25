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

const BYTES: [u8; 192] = [
    0x2e, 0x71, 0x50, 0x63, 0x63, 0x01, 0xde, 0x1d, 0x33, 0xcc, 0x5c, 0x38, 0x13, 0x36, 0x9d, 0x08,
    0xd9, 0x94, 0xf0, 0x91, 0x8d, 0x59, 0x11, 0x6d, 0xaf, 0x45, 0x0c, 0xd2, 0x97, 0x60, 0xce, 0xbd,
    0xf9, 0x54, 0xdb, 0x72, 0x7e, 0xe2, 0x12, 0x09, 0x25, 0x13, 0x95, 0x37, 0xb2, 0xcb, 0xa8, 0x7b,
    0xc3, 0xba, 0x96, 0xb1, 0xf5, 0xe6, 0x20, 0x5c, 0xc0, 0xd9, 0x70, 0x7d, 0x1a, 0xb8, 0xc0, 0x4c,
    0xfa, 0x6e, 0x47, 0xd1, 0x66, 0xfc, 0xab, 0x4c, 0x3d, 0xf2, 0xbc, 0x0a, 0x15, 0x80, 0x39, 0x1e,
    0xa6, 0xfa, 0x68, 0x8d, 0xfa, 0x5f, 0x9c, 0x41, 0xd5, 0x0b, 0x9f, 0x7b, 0x9f, 0xc6, 0x8e, 0x8d,
    0x41, 0x04, 0xd9, 0x19, 0x40, 0xea, 0xd1, 0x78, 0x61, 0x9b, 0xda, 0xe3, 0xd2, 0x13, 0x31, 0xb6,
    0x3b, 0xce, 0x58, 0xc1, 0xf9, 0xe7, 0x75, 0xb5, 0x10, 0x9c, 0x6f, 0x6e, 0x28, 0xaa, 0x23, 0x3c,
    0x5c, 0x6c, 0x0c, 0x5f, 0x62, 0x72, 0xb6, 0xfe, 0x2b, 0x2e, 0x2b, 0xf6, 0x8f, 0xcb, 0xd1, 0x8b,
    0x2d, 0xeb, 0x5e, 0x40, 0xb4, 0xe1, 0xe4, 0x3f, 0x56, 0x0e, 0xdd, 0x40, 0x3f, 0xdf, 0xb6, 0xed,
    0xfe, 0x45, 0xb7, 0x05, 0x0d, 0xb5, 0xeb, 0x29, 0x5c, 0xb1, 0xd6, 0x0c, 0xfd, 0x98, 0x57, 0x67,
    0xb0, 0x4c, 0xd0, 0x44, 0xc1, 0x61, 0xe1, 0x8e, 0x64, 0x6c, 0x45, 0xdd, 0x77, 0xf6, 0x5f, 0x8b,
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
            let mut archive = Archive::create(setup_container_with_bsize(92), false).unwrap();

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
mk_test!(t1_46 -> ("f1", 46));
mk_test!(t1_92 -> ("f1", 92));
mk_test!(t1_138 -> ("f1", 138));
mk_test!(t1_184 -> ("f1", 184));
mk_test!(t2_0, ("f1", 0) -> ("f2", 0));
mk_test!(t2_46, ("f1", 46) -> ("f2", 46));
mk_test!(t2_92, ("f1", 92) -> ("f2", 92));
mk_test!(t2_138, ("f1", 138) -> ("f2", 138));
mk_test!(t2_184, ("f1", 184) -> ("f2", 184));
mk_test!(t3_0, ("f1", 0), ("f2", 0) -> ("f3", 0));
mk_test!(t3_46, ("f1", 46), ("f2", 46) -> ("f3", 46));
mk_test!(t3_92, ("f1", 92), ("f2", 92) -> ("f3", 92));
mk_test!(t3_138, ("f1", 138), ("f2", 138) -> ("f3", 138));
mk_test!(t3_184, ("f1", 184), ("f2", 184) -> ("f3", 184));

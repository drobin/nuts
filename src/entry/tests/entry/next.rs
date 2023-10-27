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

use crate::entry::tests::FULL;
use crate::tests::{setup_container, setup_container_with_bsize};
use crate::Archive;

const BYTES: [u8; 224] = [
    0xb9, 0x40, 0xa9, 0xb0, 0x80, 0x80, 0x86, 0xba, 0xe8, 0x58, 0x4a, 0x60, 0xb1, 0xb7, 0xf7, 0x1d,
    0xd5, 0x9a, 0x0d, 0xd2, 0xe5, 0x30, 0x73, 0x49, 0xfe, 0x7a, 0x62, 0xf1, 0xb0, 0x93, 0x41, 0xdb,
    0x3b, 0x02, 0xbd, 0x83, 0xfd, 0x6a, 0x86, 0xb9, 0x90, 0xe8, 0x66, 0x7f, 0xdd, 0xc9, 0x74, 0xcb,
    0xc1, 0x9f, 0x80, 0x2a, 0xe4, 0x3a, 0x58, 0x07, 0xdb, 0xae, 0xe6, 0xee, 0x20, 0xa4, 0x03, 0x80,
    0x93, 0x24, 0xf2, 0x96, 0x85, 0xf9, 0x7e, 0x12, 0x4a, 0x67, 0x54, 0x44, 0x98, 0xce, 0x6b, 0x3f,
    0x87, 0x8c, 0xc5, 0xf1, 0x20, 0xbb, 0x0f, 0x9b, 0x4a, 0x64, 0xc6, 0xac, 0xea, 0x74, 0x3e, 0x80,
    0xb8, 0xcf, 0x10, 0x48, 0x17, 0x79, 0x78, 0xfc, 0x25, 0xdc, 0x62, 0x65, 0x6f, 0x92, 0x61, 0x9e,
    0xe4, 0xef, 0x64, 0x4d, 0xb0, 0x57, 0x9f, 0xee, 0x8d, 0xc6, 0xff, 0x6f, 0x90, 0x72, 0x26, 0x30,
    0x5e, 0x30, 0x84, 0x49, 0x23, 0x84, 0xc3, 0x19, 0x5c, 0xc7, 0x5f, 0xf6, 0xc8, 0x8a, 0x90, 0x96,
    0x0d, 0xb2, 0xc8, 0x30, 0x44, 0x45, 0xe2, 0x2d, 0x5f, 0x0b, 0x53, 0x93, 0xf0, 0xa4, 0x6b, 0x54,
    0x2a, 0x68, 0x70, 0xc1, 0x19, 0xbf, 0x51, 0x52, 0x87, 0x37, 0xed, 0xf7, 0xac, 0xff, 0x66, 0xc3,
    0x7d, 0xdc, 0x91, 0x38, 0x0a, 0xae, 0x74, 0x65, 0xc1, 0x17, 0x18, 0x83, 0xdc, 0xfb, 0x46, 0xe3,
    0x85, 0xb7, 0xee, 0x57, 0xaf, 0xba, 0x7a, 0x01, 0xd2, 0x6f, 0xe5, 0xde, 0x32, 0xa2, 0x09, 0xe5,
    0x2e, 0x19, 0x54, 0x3e, 0xe2, 0x07, 0xbe, 0x0a, 0xfc, 0x47, 0xe4, 0xc4, 0xd8, 0xaf, 0x3c, 0x3d,
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
            let mut archive = Archive::create(setup_container_with_bsize(FULL as u32), false).unwrap();

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
mk_test!(t1_53 -> ("f1", 53));
mk_test!(t1_106 -> ("f1", 106));
mk_test!(t1_159 -> ("f1", 159));
mk_test!(t1_212 -> ("f1", 212));
mk_test!(t2_0, ("f1", 0) -> ("f2", 0));
mk_test!(t2_53, ("f1", 53) -> ("f2", 53));
mk_test!(t2_106, ("f1", 106) -> ("f2", 106));
mk_test!(t2_159, ("f1", 159) -> ("f2", 159));
mk_test!(t2_212, ("f1", 212) -> ("f2", 212));
mk_test!(t3_0, ("f1", 0), ("f2", 0) -> ("f3", 0));
mk_test!(t3_53, ("f1", 53), ("f2", 53) -> ("f3", 53));
mk_test!(t3_106, ("f1", 106), ("f2", 106) -> ("f3", 106));
mk_test!(t3_159, ("f1", 159), ("f2", 159) -> ("f3", 159));
mk_test!(t3_212, ("f1", 212), ("f2", 212) -> ("f3", 212));

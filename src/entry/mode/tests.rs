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

use nuts_bytes::{Reader, Writer};

use crate::entry::mode::{Group, Mode};

macro_rules! can_test {
    ($name:ident, $method: ident ( $group:ident ), $mask:literal) => {
        #[test]
        fn $name() {
            assert!(Mode($mask).$method(Group::$group))
        }
    };
}

macro_rules! cannot_test {
    ($name:ident, $method:ident ( $group:ident )) => {
        #[test]
        fn $name() {
            assert!(!Mode(0x0000).$method(Group::$group))
        }
    };
}

macro_rules! set_test {
    ($name:ident, $method:ident ( $group:ident), $mask:literal ) => {
        #[test]
        fn $name() {
            let mut mode = Mode(0x0000);

            mode.$method(Group::$group, true);

            assert_eq!(mode.0, $mask);
        }
    };
}

macro_rules! unset_test {
    ($name:ident, $method:ident ( $group:ident ), $mask:literal) => {
        #[test]
        fn $name() {
            let mut mode = Mode(0xffff);

            mode.$method(Group::$group, false);

            assert_eq!(mode.0, $mask);
        }
    };
}

#[test]
fn ser() {
    let mut writer = Writer::new(vec![]);

    writer.serialize(&Mode(4711u16)).unwrap();

    assert_eq!(writer.into_target(), [0x12, 0x67]);
}

#[test]
fn de() {
    let mut reader = Reader::new([0x12, 0x67].as_slice());

    let mode = reader.deserialize::<Mode>().unwrap();
    assert_eq!(mode.0, 4711);
}

#[test]
fn file() {
    assert_eq!(Mode::file().0, 0b00000001_01101111);
}

#[test]
fn directory() {
    assert_eq!(Mode::directory().0, 0b00000011_01101111);
}

#[test]
fn symlink() {
    assert_eq!(Mode::symlink().0, 0b00000101_01101111);
}

can_test!(can_read_user, can_read(User), 0b00000000_00000001);
can_test!(can_write_user, can_write(User), 0b00000000_00000010);
can_test!(can_execute_user, can_execute(User), 0b00000000_00000100);
can_test!(can_read_group, can_read(Group), 0b00000000_00001000);
can_test!(can_write_group, can_write(Group), 0b00000000_00010000);
can_test!(can_execute_group, can_execute(Group), 0b00000000_00100000);
can_test!(can_read_other, can_read(Other), 0b00000000_01000000);
can_test!(can_write_other, can_write(Other), 0b00000000_10000000);
can_test!(can_execute_other, can_execute(Other), 0b00000001_00000000);

cannot_test!(cannot_read_user, can_read(User));
cannot_test!(cannot_write_user, can_write(User));
cannot_test!(cannot_execute_user, can_execute(User));
cannot_test!(cannot_read_group, can_read(Group));
cannot_test!(cannot_write_group, can_write(Group));
cannot_test!(cannot_execute_group, can_execute(Group));
cannot_test!(cannot_read_other, can_read(Other));
cannot_test!(cannot_write_other, can_write(Other));
cannot_test!(cannot_execute_other, can_execute(Other));

set_test!(set_user_user, set_readable(User), 0b00000000_00000001);
set_test!(set_write_user, set_writable(User), 0b00000000_00000010);
set_test!(set_execute_user, set_executable(User), 0b00000000_00000100);
set_test!(set_read_group, set_readable(Group), 0b00000000_00001000);
set_test!(set_write_group, set_writable(Group), 0b00000000_00010000);
set_test!(
    set_execute_group,
    set_executable(Group),
    0b00000000_00100000
);
set_test!(set_read_other, set_readable(Other), 0b00000000_01000000);
set_test!(set_write_other, set_writable(Other), 0b00000000_10000000);
set_test!(
    set_execute_other,
    set_executable(Other),
    0b00000001_00000000
);

unset_test!(unset_read_user, set_readable(User), 0b11111111_11111110);
unset_test!(unset_write_user, set_writable(User), 0b11111111_11111101);
unset_test!(
    unset_execute_user,
    set_executable(User),
    0b11111111_11111011
);
unset_test!(unset_read_group, set_readable(Group), 0b11111111_11110111);
unset_test!(unset_write_group, set_writable(Group), 0b11111111_11101111);
unset_test!(
    unset_execute_group,
    set_executable(Group),
    0b11111111_11011111
);
unset_test!(unset_read_other, set_readable(Other), 0b11111111_10111111);
unset_test!(unset_write_other, set_writable(Other), 0b11111111_01111111);
unset_test!(
    unset_execute_other,
    set_executable(Other),
    0b11111110_11111111
);

#[test]
fn is_file() {
    assert!(Mode(0b00000000_00000000).is_file());
}

#[test]
fn is_not_file() {
    assert!(!Mode(0b00000010_00000000).is_file());
    assert!(!Mode(0b00000100_00000000).is_file());
    assert!(!Mode(0b00000110_00000000).is_file());
}

#[test]
fn is_directory() {
    assert!(Mode(0b00000010_00000000).is_directory());
}

#[test]
fn is_not_directory() {
    assert!(!Mode(0b00000000_00000000).is_directory());
    assert!(!Mode(0b00000100_00000000).is_directory());
    assert!(!Mode(0b00000110_00000000).is_directory());
}

#[test]
fn is_symlink() {
    assert!(Mode(0b00000100_00000000).is_symlink());
}

#[test]
fn is_not_symlink() {
    assert!(!Mode(0b00000000_00000000).is_symlink());
    assert!(!Mode(0b00000010_00000000).is_symlink());
    assert!(!Mode(0b00000110_00000000).is_symlink());
}

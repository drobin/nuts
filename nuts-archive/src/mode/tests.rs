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

use super::{Group, Mode};

#[test]
fn group_user_into_readable_flag() {
    assert_eq!(Group::User.into_readable_flag(), 0b100);
}

#[test]
fn group_group_into_readable_flag() {
    assert_eq!(Group::Group.into_readable_flag(), 0b100000);
}

#[test]
fn group_other_into_readable_flag() {
    assert_eq!(Group::Other.into_readable_flag(), 0b100000000);
}

#[test]
fn group_user_into_writable_flag() {
    assert_eq!(Group::User.into_writable_flag(), 0b010);
}

#[test]
fn group_group_into_writable_flag() {
    assert_eq!(Group::Group.into_writable_flag(), 0b010000);
}

#[test]
fn group_other_into_writable_flag() {
    assert_eq!(Group::Other.into_writable_flag(), 0b010000000);
}

#[test]
fn group_user_into_executable_flag() {
    assert_eq!(Group::User.into_executable_flag(), 0b001);
}

#[test]
fn group_group_into_executable_flag() {
    assert_eq!(Group::Group.into_executable_flag(), 0b001000);
}

#[test]
fn group_other_into_executable_flag() {
    assert_eq!(Group::Other.into_executable_flag(), 0b001000000);
}

#[test]
fn mode_from_mask_unknown() {
    let mode = Mode::from_mask(0x000000);

    assert!(!mode.is_file());
    assert!(!mode.is_directory());
    assert!(!mode.is_symlink());
}

#[test]
fn mode_from_mask_file() {
    let mode = Mode::from_mask(0x010000);

    assert!(mode.is_file());
    assert!(!mode.is_directory());
    assert!(!mode.is_symlink());
}

#[test]
fn mode_from_mask_directory() {
    let mode = Mode::from_mask(0x020000);

    assert!(!mode.is_file());
    assert!(mode.is_directory());
    assert!(!mode.is_symlink());
}

#[test]
fn mode_from_mask_symlink() {
    let mode = Mode::from_mask(0x030000);

    assert!(!mode.is_file());
    assert!(!mode.is_directory());
    assert!(mode.is_symlink());
}

#[test]
fn mode_from_mask_readable() {
    let mode = Mode::from_mask(0b00_0000000100100100);

    assert!(mode.is_readable(Group::User));
    assert!(mode.is_readable(Group::Group));
    assert!(mode.is_readable(Group::Other));
}

#[test]
fn mode_from_mask_not_readable() {
    let mode = Mode::from_mask(0b00_0000000011011011);

    assert!(!mode.is_readable(Group::User));
    assert!(!mode.is_readable(Group::Group));
    assert!(!mode.is_readable(Group::Other));
}

#[test]
fn mode_from_mask_writable() {
    let mode = Mode::from_mask(0b00_0000000010010010);

    assert!(mode.is_writable(Group::User));
    assert!(mode.is_writable(Group::Group));
    assert!(mode.is_writable(Group::Other));
}

#[test]
fn mode_from_mask_not_writable() {
    let mode = Mode::from_mask(0b00_0000000101101101);

    assert!(!mode.is_writable(Group::User));
    assert!(!mode.is_writable(Group::Group));
    assert!(!mode.is_writable(Group::Other));
}

#[test]
fn mode_from_mask_executable() {
    let mode = Mode::from_mask(0b00_0000000001001001);

    assert!(mode.is_executable(Group::User));
    assert!(mode.is_executable(Group::Group));
    assert!(mode.is_executable(Group::Other));
}

#[test]
fn mode_from_mask_not_executable() {
    let mode = Mode::from_mask(0b00_0000000110110110);

    assert!(!mode.is_executable(Group::User));
    assert!(!mode.is_executable(Group::Group));
    assert!(!mode.is_executable(Group::Other));
}

#[test]
fn mode_file() {
    assert_eq!(Mode::file().0, 0b01_0000000100100110);
}
#[test]
fn mode_directory() {
    assert_eq!(Mode::directory().0, 0b10_0000000101101111);
}

#[test]
fn mode_symlink() {
    assert_eq!(Mode::symlink().0, 0b11_0000000101101111);
}

#[test]
fn mode_is_readable() {
    let mode = Mode(0b100100100);

    assert!(mode.is_readable(Group::User));
    assert!(mode.is_readable(Group::Group));
    assert!(mode.is_readable(Group::Other));
}

#[test]
fn mode_not_readable() {
    let mode = Mode(0b011011011);

    assert!(!mode.is_readable(Group::User));
    assert!(!mode.is_readable(Group::Group));
    assert!(!mode.is_readable(Group::Other));
}

#[test]
fn mode_set_readable() {
    let mut mode = Mode(0b000000000);

    mode.set_readable(Group::User, true);
    mode.set_readable(Group::Group, true);
    mode.set_readable(Group::Other, true);

    assert_eq!(mode.0, 0b100100100);
}

#[test]
fn mode_unset_readable() {
    let mut mode = Mode(0b111111111);

    mode.set_readable(Group::User, false);
    mode.set_readable(Group::Group, false);
    mode.set_readable(Group::Other, false);

    assert_eq!(mode.0, 0b011011011);
}

#[test]
fn mode_is_writable() {
    let mode = Mode(0b010010010);

    assert!(mode.is_writable(Group::User));
    assert!(mode.is_writable(Group::Group));
    assert!(mode.is_writable(Group::Other));
}

#[test]
fn mode_not_writable() {
    let mode = Mode(0b101101101);

    assert!(!mode.is_writable(Group::User));
    assert!(!mode.is_writable(Group::Group));
    assert!(!mode.is_writable(Group::Other));
}

#[test]
fn mode_set_writable() {
    let mut mode = Mode(0b000000000);

    mode.set_writable(Group::User, true);
    mode.set_writable(Group::Group, true);
    mode.set_writable(Group::Other, true);

    assert_eq!(mode.0, 0b010010010);
}

#[test]
fn mode_unset_writable() {
    let mut mode = Mode(0b111111111);

    mode.set_writable(Group::User, false);
    mode.set_writable(Group::Group, false);
    mode.set_writable(Group::Other, false);

    assert_eq!(mode.0, 0b101101101);
}

#[test]
fn mode_is_executable() {
    let mode = Mode(0b001001001);

    assert!(mode.is_executable(Group::User));
    assert!(mode.is_executable(Group::Group));
    assert!(mode.is_executable(Group::Other));
}

#[test]
fn mode_not_executable() {
    let mode = Mode(0b110110110);

    assert!(!mode.is_executable(Group::User));
    assert!(!mode.is_executable(Group::Group));
    assert!(!mode.is_executable(Group::Other));
}

#[test]
fn mode_set_executable() {
    let mut mode = Mode(0b000000000);

    mode.set_executable(Group::User, true);
    mode.set_executable(Group::Group, true);
    mode.set_executable(Group::Other, true);

    assert_eq!(mode.0, 0b001001001);
}

#[test]
fn mode_unset_executable() {
    let mut mode = Mode(0b111111111);

    mode.set_executable(Group::User, false);
    mode.set_executable(Group::Group, false);
    mode.set_executable(Group::Other, false);

    assert_eq!(mode.0, 0b110110110);
}

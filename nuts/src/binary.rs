// MIT License
//
// Copyright (c) 2020 Robin Doer
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

use byteorder::{ByteOrder, NetworkEndian};

use crate::error::Error;
use crate::result::Result;

pub type AsFunc<U, T> = fn(U) -> Result<T>;

pub fn read_array<'a>(data: &'a [u8], offset: &mut u32, size: u32) -> Result<&'a [u8]> {
    let start = *offset as usize;
    let end = start + size as usize;
    let slice: &[u8] = data.get(start..end).ok_or(Error::NoData)?;

    *offset += size;

    Ok(slice)
}

pub fn read_array_as<'a, T>(
    data: &'a [u8],
    offset: &mut u32,
    size: u32,
    convert: AsFunc<&'a [u8], T>,
) -> Result<T> {
    read_array(data, offset, size).and_then(|slice| convert(slice))
}

pub fn read_vec(data: &[u8], offset: &mut u32) -> Result<Vec<u8>> {
    let size = read_u32(data, offset)?;
    let vec = read_array(data, offset, size)?;

    Ok(vec.to_vec())
}

pub fn read_u8(data: &[u8], offset: &mut u32) -> Result<u8> {
    let pos = *offset as usize;
    let num = data.get(pos).ok_or(Error::NoData)?;

    *offset += 1;

    Ok(*num)
}

pub fn read_u8_as<T>(data: &[u8], offset: &mut u32, convert: AsFunc<u8, T>) -> Result<T> {
    read_u8(data, offset).and_then(|num| convert(num))
}

pub fn read_u32(data: &[u8], offset: &mut u32) -> Result<u32> {
    let start = *offset as usize;
    let end = start + 4;
    let source = data.get(start..end).ok_or(Error::NoData)?;
    let num = NetworkEndian::read_u32(source);

    *offset += 4;

    Ok(num)
}

// pub fn read_u32_as<T>(data: &[u8], offset: &mut u32, convert: AsFunc<u32, T>) -> Result<T> {
//     read_u32(data, offset).and_then(|num| convert(num))
// }

pub fn read_u64(data: &[u8], offset: &mut u32) -> Result<u64> {
    let start = *offset as usize;
    let end = start + 8;
    let source = data.get(start..end).ok_or(Error::NoData)?;
    let num = NetworkEndian::read_u64(source);

    *offset += 8;

    Ok(num)
}

pub fn write_array(target: &mut [u8], offset: &mut u32, data: &[u8]) -> Result<()> {
    let start = *offset as usize;
    let end = start + data.len();
    let slice = target.get_mut(start..end).ok_or(Error::NoSpace)?;

    slice.copy_from_slice(data);
    *offset += slice.len() as u32;

    Ok(())
}

pub fn write_vec(target: &mut [u8], offset: &mut u32, data: &Vec<u8>) -> Result<()> {
    write_u32(target, offset, data.len() as u32)?;
    write_array(target, offset, data)?;

    Ok(())
}

pub fn write_u8(target: &mut [u8], offset: &mut u32, num: u8) -> Result<()> {
    let pos = *offset as usize;
    let n = target.get_mut(pos).ok_or(Error::NoSpace)?;

    *n = num;
    *offset += 1;

    Ok(())
}

pub fn write_u8_as<T>(
    target: &mut [u8],
    offset: &mut u32,
    val: T,
    convert: AsFunc<T, u8>,
) -> Result<()> {
    convert(val).and_then(|num| write_u8(target, offset, num))
}

pub fn write_u32(target: &mut [u8], offset: &mut u32, num: u32) -> Result<()> {
    let start = *offset as usize;
    let end = start + 4;
    let slice = target.get_mut(start..end).ok_or(Error::NoSpace)?;

    NetworkEndian::write_u32(slice, num);
    *offset += 4;

    Ok(())
}

pub fn write_u64(target: &mut [u8], offset: &mut u32, num: u64) -> Result<()> {
    let start = *offset as usize;
    let end = start + 8;
    let slice = target.get_mut(start..end).ok_or(Error::NoSpace)?;

    NetworkEndian::write_u64(slice, num);
    *offset += 8;

    Ok(())
}

// pub fn write_u32_as<T>(
//     target: &mut [u8],
//     offset: &mut u32,
//     val: T,
//     convert: AsFunc<T, u32>,
// ) -> Result<()> {
//     convert(val).and_then(|num| write_u32(target, offset, num))
// }

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

#[cfg(test)]
mod tests;

use serde::{ser, Serialize};

use crate::error::{Error, Result};
use crate::options::Int;

#[derive(Debug)]
enum Target<'a> {
    Vec(Vec<u8>),
    Slice { buf: &'a mut [u8], offs: usize },
}

impl<'a> Target<'a> {
    fn append(&mut self, bytes: &[u8]) -> Result<()> {
        match self {
            Target::Vec(vec) => Ok(vec.extend_from_slice(bytes)),
            Target::Slice { buf, offs } => match buf.get_mut(*offs..*offs + bytes.len()) {
                Some(buf) => {
                    buf.copy_from_slice(bytes);
                    *offs += bytes.len();
                    Ok(())
                }
                None => Err(Error::NoSpace),
            },
        }
    }
}

/// A cursor like utility that writes structured data into a binary stream.
///
/// Data can be written into
/// * A `Vec<u8>` structure. The vector grows automatically when appending
///   data.
/// * A mutable `[u8]` slice. Data are appended to the slice but an
///   [`Error::NoSpace`] error is generated if the end of the slice is reached.
#[derive(Debug)]
pub struct Writer<'a> {
    int: Int,
    target: Target<'a>,
}

macro_rules! write_fixint_primitive {
    ($name:ident -> $ty:ty) => {
        fn $name(&mut self, value: $ty) -> Result<()> {
            self.write_bytes(&value.to_be_bytes())
        }
    };
}

macro_rules! write_var_primitive {
    ($name:ident ($num:literal) -> $ty:ty) => {
        fn $name(&mut self, value: $ty) -> Result<()> {
            let mut bytes = [$num; std::mem::size_of::<$ty>() + 1];

            bytes
                .get_mut(1..)
                .unwrap()
                .copy_from_slice(&value.to_be_bytes());

            self.write_bytes(&bytes)
        }
    };
}

impl<'a> Writer<'a> {
    pub(crate) fn for_vec(int: Int, vec: Vec<u8>) -> Writer<'a> {
        Writer {
            int,
            target: Target::Vec(vec),
        }
    }

    pub(crate) fn for_slice(int: Int, buf: &'a mut [u8]) -> Writer<'a> {
        Writer {
            int,
            target: Target::Slice { buf, offs: 0 },
        }
    }

    /// Returns the current position of the writer.
    pub fn position(&self) -> usize {
        match &self.target {
            Target::Vec(vec) => vec.len(),
            Target::Slice { buf: _, offs } => *offs,
        }
    }

    /// Consumes this writer, returning the underlying value.
    ///
    /// When writing into a slice, it is converted into a `Vec`.
    pub fn into_vec(self) -> Vec<u8> {
        match self.target {
            Target::Vec(vec) => vec,
            Target::Slice { buf, offs: _ } => buf.to_vec(),
        }
    }

    /// Gets a reference to the underlying value in this writer.
    pub fn as_slice(&self) -> &[u8] {
        match &self.target {
            Target::Vec(vec) => vec,
            Target::Slice { buf, offs: _ } => buf,
        }
    }

    write_var_primitive!(write_var_251 (251) -> u16);
    write_var_primitive!(write_var_252 (252) -> u32);
    write_var_primitive!(write_var_253 (253) -> u64);
    write_var_primitive!(write_var_254 (254) -> u128);

    write_fixint_primitive!(write_fix_u16 -> u16);
    write_fixint_primitive!(write_fix_u32 -> u32);
    write_fixint_primitive!(write_fix_u64 -> u64);
    write_fixint_primitive!(write_fix_u128 -> u128);

    fn write_var_u16(&mut self, value: u16) -> Result<()> {
        if value < 251 {
            self.write_u8(value as u8)
        } else {
            self.write_var_251(value)
        }
    }

    fn write_var_u32(&mut self, value: u32) -> Result<()> {
        if value < 251 {
            self.write_u8(value as u8)
        } else if value < 2u32.pow(16) {
            self.write_var_251(value as u16)
        } else {
            self.write_var_252(value)
        }
    }

    fn write_var_u64(&mut self, value: u64) -> Result<()> {
        if value < 251 {
            self.write_u8(value as u8)
        } else if value < 2u64.pow(16) {
            self.write_var_251(value as u16)
        } else if value < 2u64.pow(32) {
            self.write_var_252(value as u32)
        } else {
            self.write_var_253(value)
        }
    }

    fn write_var_u128(&mut self, value: u128) -> Result<()> {
        if value < 251 {
            self.write_u8(value as u8)
        } else if value < 2u128.pow(16) {
            self.write_var_251(value as u16)
        } else if value < 2u128.pow(32) {
            self.write_var_252(value as u32)
        } else if value < 2u128.pow(64) {
            self.write_var_253(value as u64)
        } else {
            self.write_var_254(value)
        }
    }

    /// Appends an `u8` value at the end of this writer.
    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write_bytes(&[value])
    }

    /// Appends an `u16` value at the end of this writer.
    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        match self.int {
            Int::Fix => self.write_fix_u16(value),
            Int::Var => self.write_var_u16(value),
        }
    }

    /// Appends an `u32` value at the end of this writer.
    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        match self.int {
            Int::Fix => self.write_fix_u32(value),
            Int::Var => self.write_var_u32(value),
        }
    }

    /// Appends an `u64` value at the end of this writer.
    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        match self.int {
            Int::Fix => self.write_fix_u64(value),
            Int::Var => self.write_var_u64(value),
        }
    }

    /// Appends an `u128` value at the end of this writer.
    pub fn write_u128(&mut self, value: u128) -> Result<()> {
        match self.int {
            Int::Fix => self.write_fix_u128(value),
            Int::Var => self.write_var_u128(value),
        }
    }

    /// Appends the given `bytes` at the end of this writer.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.target.append(bytes)
    }
}

impl<'a, 'b> ser::Serializer for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.write_u8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_u64(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.write_u32(v as u32)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write_u64(v.len() as u64)
            .and_then(|()| self.write_bytes(v))
    }

    fn serialize_none(self) -> Result<()> {
        self.write_u8(0)
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<()> {
        self.write_u8(1).and_then(|()| value.serialize(self))
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.write_u32(variant_index)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.write_u32(variant_index)
            .and_then(|()| value.serialize(self))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self> {
        len.ok_or(Error::RequiredLength)
            .and_then(|len| self.write_u64(len as u64))
            .map(|()| self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self> {
        self.write_u32(variant_index).map(|()| self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self> {
        len.ok_or(Error::RequiredLength)
            .and_then(|len| self.write_u64(len as u64))
            .map(|()| self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self> {
        self.write_u32(variant_index).map(|()| self)
    }
}

impl<'a, 'b> ser::SerializeSeq for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTuple for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTupleStruct for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTupleVariant for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeMap for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, _key: &T) -> Result<()> {
        unimplemented!()
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, _value: &T) -> Result<()> {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        Ok(())
    }

    fn serialize_entry<K: Serialize + ?Sized, V: Serialize + ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()> {
        key.serialize(&mut **self)
            .and_then(|()| value.serialize(&mut **self))
    }
}

impl<'a, 'b> ser::SerializeStruct for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeStructVariant for &'b mut Writer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

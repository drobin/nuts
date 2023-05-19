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
#[cfg(doc)]
use crate::options::Options;
use crate::target::PutBytes;

/// A cursor like utility that writes structured data into an arbitrary target.
///
/// The target must implement the [`PutBytes`] trait which supports writing
/// binary data into it.
///
/// The [`Options`] type is used to construct an instance of this `Writer`. See
/// [`Options::build_writer()`] for more information.
#[derive(Debug)]
pub struct Writer<T> {
    int: Int,
    target: T,
}

macro_rules! write_fixint_primitive {
    ($name:ident -> $ty:ty) => {
        fn $name(&mut self, value: $ty) -> Result<usize> {
            const N: usize = std::mem::size_of::<$ty>();
            self.target.put_bytes(&value.to_be_bytes()).map(|()| N)
        }
    };
}

macro_rules! write_var_primitive {
    ($name:ident ($num:literal) -> $ty:ty) => {
        fn $name(&mut self, value: $ty) -> Result<usize> {
            const N: usize = std::mem::size_of::<$ty>() + 1;
            let mut bytes = [$num; N];

            bytes
                .get_mut(1..)
                .unwrap()
                .copy_from_slice(&value.to_be_bytes());

            self.target.put_bytes(&bytes).map(|()| N)
        }
    };
}

impl<T: PutBytes> Writer<T> {
    pub(crate) fn new(int: Int, target: T) -> Writer<T> {
        Writer { int, target }
    }

    /// Consumes this `Writer`, returning the underlying target.
    pub fn into_target(self) -> T {
        self.target
    }

    write_var_primitive!(write_var_251 (251) -> u16);
    write_var_primitive!(write_var_252 (252) -> u32);
    write_var_primitive!(write_var_253 (253) -> u64);
    write_var_primitive!(write_var_254 (254) -> u128);

    write_fixint_primitive!(write_fix_u16 -> u16);
    write_fixint_primitive!(write_fix_u32 -> u32);
    write_fixint_primitive!(write_fix_u64 -> u64);
    write_fixint_primitive!(write_fix_u128 -> u128);

    fn write_var_u16(&mut self, value: u16) -> Result<usize> {
        if value < 251 {
            self.write_u8(value as u8)
        } else {
            self.write_var_251(value)
        }
    }

    fn write_var_u32(&mut self, value: u32) -> Result<usize> {
        if value < 251 {
            self.write_u8(value as u8)
        } else if value < 2u32.pow(16) {
            self.write_var_251(value as u16)
        } else {
            self.write_var_252(value)
        }
    }

    fn write_var_u64(&mut self, value: u64) -> Result<usize> {
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

    fn write_var_u128(&mut self, value: u128) -> Result<usize> {
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
    pub fn write_u8(&mut self, value: u8) -> Result<usize> {
        self.write_bytes(&[value])
    }

    /// Appends an `u16` value at the end of this writer.
    pub fn write_u16(&mut self, value: u16) -> Result<usize> {
        match self.int {
            Int::Fix => self.write_fix_u16(value),
            Int::Var => self.write_var_u16(value),
        }
    }

    /// Appends an `u32` value at the end of this writer.
    pub fn write_u32(&mut self, value: u32) -> Result<usize> {
        match self.int {
            Int::Fix => self.write_fix_u32(value),
            Int::Var => self.write_var_u32(value),
        }
    }

    /// Appends an `u64` value at the end of this writer.
    pub fn write_u64(&mut self, value: u64) -> Result<usize> {
        match self.int {
            Int::Fix => self.write_fix_u64(value),
            Int::Var => self.write_var_u64(value),
        }
    }

    /// Appends an `u128` value at the end of this writer.
    pub fn write_u128(&mut self, value: u128) -> Result<usize> {
        match self.int {
            Int::Fix => self.write_fix_u128(value),
            Int::Var => self.write_var_u128(value),
        }
    }

    /// Appends the given `bytes` at the end of this writer.
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<usize> {
        self.target.put_bytes(bytes).map(|()| bytes.len())
    }
}

impl<T> AsRef<T> for Writer<T> {
    fn as_ref(&self) -> &T {
        &self.target
    }
}

impl<'a, P: PutBytes> ser::Serializer for &'a mut Writer<P> {
    type Ok = usize;
    type Error = Error;
    type SerializeSeq = StateSerializer<'a, P>;
    type SerializeTuple = StateSerializer<'a, P>;
    type SerializeTupleStruct = StateSerializer<'a, P>;
    type SerializeTupleVariant = StateSerializer<'a, P>;
    type SerializeMap = StateSerializer<'a, P>;
    type SerializeStruct = StateSerializer<'a, P>;
    type SerializeStructVariant = StateSerializer<'a, P>;

    fn serialize_bool(self, v: bool) -> Result<usize> {
        self.write_u8(if v { 1 } else { 0 })
    }

    fn serialize_i8(self, _v: i8) -> Result<usize> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<usize> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<usize> {
        unimplemented!()
    }

    fn serialize_i64(self, _v: i64) -> Result<usize> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<usize> {
        self.write_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<usize> {
        self.write_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<usize> {
        self.write_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<usize> {
        self.write_u64(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<usize> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<usize> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<usize> {
        self.write_u32(v as u32)
    }

    fn serialize_str(self, v: &str) -> Result<usize> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<usize> {
        self.write_u64(v.len() as u64)
            .and_then(|a| self.write_bytes(v).map(|b| a + b))
    }

    fn serialize_none(self) -> Result<usize> {
        self.write_u8(0)
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<usize> {
        self.write_u8(1)
            .and_then(|a| value.serialize(self).map(|b| a + b))
    }

    fn serialize_unit(self) -> Result<usize> {
        Ok(0)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<usize> {
        Ok(0)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<usize> {
        self.write_u32(variant_index)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<usize> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<usize> {
        self.write_u32(variant_index)
            .and_then(|a| value.serialize(self).map(|b| a + b))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        len.ok_or(Error::RequiredLength)
            .and_then(|len| self.write_u64(len as u64))
            .map(|n| StateSerializer::new(self, n))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(StateSerializer::new(self, 0))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(StateSerializer::new(self, 0))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.write_u32(variant_index)
            .map(|n| StateSerializer::new(self, n))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        len.ok_or(Error::RequiredLength)
            .and_then(|len| self.write_u64(len as u64))
            .map(|n| StateSerializer::new(self, n))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(StateSerializer::new(self, 0))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.write_u32(variant_index)
            .map(|n| StateSerializer::new(self, n))
    }
}

pub struct StateSerializer<'a, T> {
    writer: &'a mut Writer<T>,
    ok: usize,
}

impl<'a, T> StateSerializer<'a, T> {
    fn new(writer: &'a mut Writer<T>, ok: usize) -> StateSerializer<T> {
        StateSerializer { writer, ok }
    }
}

impl<'a, P: PutBytes> ser::SerializeSeq for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.writer).map(|n| {
            self.ok += n;
            ()
        })
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }
}

impl<'a, P: PutBytes> ser::SerializeTuple for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.writer).map(|n| {
            self.ok += n;
            ()
        })
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }
}

impl<'a, P: PutBytes> ser::SerializeTupleStruct for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.writer).map(|n| {
            self.ok += n;
            ()
        })
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }
}

impl<'a, P: PutBytes> ser::SerializeTupleVariant for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.writer).map(|n| {
            self.ok += n;
            ()
        })
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }
}

impl<'a, P: PutBytes> ser::SerializeMap for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, _key: &T) -> Result<()> {
        unimplemented!()
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, _value: &T) -> Result<()> {
        unimplemented!()
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }

    fn serialize_entry<K: Serialize + ?Sized, V: Serialize + ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()> {
        key.serialize(&mut *self.writer).and_then(|a| {
            value.serialize(&mut *self.writer).map(|b| {
                self.ok += a + b;
                ()
            })
        })
    }
}

impl<'a, P: PutBytes> ser::SerializeStruct for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(&mut *self.writer).map(|n| {
            self.ok += n;
            ()
        })
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }
}

impl<'a, P: PutBytes> ser::SerializeStructVariant for StateSerializer<'a, P> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(&mut *self.writer).map(|n| {
            self.ok += n;
            ()
        })
    }

    fn end(self) -> Result<usize> {
        Ok(self.ok)
    }
}

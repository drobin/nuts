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

use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;
use std::borrow::Cow;
use std::str;

use crate::error::{Error, Result};
use crate::source::TakeBytes;

macro_rules! read_primitive {
    ($(#[$outer:meta])* $name:ident -> $ty:ty) => {
        $(#[$outer])*
        pub fn $name(&mut self) -> Result<$ty> {
            let mut bytes = [0; std::mem::size_of::<$ty>()];
            self.source
                .take_bytes_to(&mut bytes)
                .map(|()| <$ty>::from_be_bytes(bytes))
        }
    };
}

/// A cursor like utility that reads structured data from an arbitrary source.
///
/// The source must implement the [`TakeBytes`] trait which supports reading
/// binary data from it.
pub struct Reader<T> {
    source: T,
}

impl<'tb, T: TakeBytes<'tb>> Reader<T> {
    /// Creates a new `Reader` instance.
    ///
    /// The source of the reader is passed to the function. Every type that
    /// implements the [`TakeBytes`] trait can be the source of this reader.
    pub fn new(source: T) -> Reader<T> {
        Reader { source }
    }

    /// Deserializes from this binary representation into a data structure
    /// which implements [Serde](https://www.serde.rs) [`Deserialize`] trait.
    ///
    /// This is simply a wrapper for
    ///
    /// ```text
    /// D::deserialize(self)
    /// ```
    pub fn deserialize<D: Deserialize<'tb>>(&mut self) -> Result<D> {
        D::deserialize(self)
    }

    read_primitive!(
        /// Reads an `i8` value from the reader.
        read_i8 -> i8
    );

    read_primitive!(
        /// Reads an `u8` value from the reader.
        read_u8 -> u8
    );

    read_primitive!(
        /// Reads an `i16` value from the reader.
        read_i16 -> i16
    );

    read_primitive!(
        /// Reads an `u16` value from the reader.
        read_u16 -> u16
    );

    read_primitive!(
        /// Reads an `i32` value from the reader.
        read_i32 -> i32
    );

    read_primitive!(
        /// Reads an `u32` value from the reader.
        read_u32 -> u32
    );

    read_primitive!(
        /// Reads an `i64` value from the reader.
        read_i64 -> i64
    );

    read_primitive!(
        /// Reads an `u64` value from the reader.
        read_u64 -> u64
    );

    read_primitive!(
        /// Reads an `i128` value from the reader.
        read_i128 -> i128
    );

    read_primitive!(
        /// Reads an `u128` value from the reader.
        read_u128 -> u128
    );

    /// Reads `n` bytes from the reader.
    ///
    /// If possible a slice of borrowed data of the given size (`n`) wrapped
    /// into [`Cow::Borrowed`] is returned.
    ///
    /// If the data cannot be borrowed a [`Vec<u8>`] wrapped into a
    /// [`Cow::Owned`] is returned.
    ///
    /// # Errors
    ///
    /// If not enough data are available an [`Error::Eof`] error is returned.
    pub fn read_bytes(&mut self, n: usize) -> Result<Cow<'tb, [u8]>> {
        self.source.take_bytes(n)
    }

    /// Reads some bytes from the reader and puts them into the given buffer
    /// `buf`.
    ///
    /// # Errors
    ///
    /// If not enough data are available to fill `buf` an [`Error::Eof`] error
    /// is returned.
    pub fn read_bytes_to(&mut self, buf: &mut [u8]) -> Result<()> {
        self.source.take_bytes_to(buf)
    }
}

impl<'tb, T: TakeBytes<'tb>> AsRef<T> for Reader<T> {
    fn as_ref(&self) -> &T {
        &self.source
    }
}

impl<'a, 'de, 'tb: 'de, T: TakeBytes<'tb>> de::Deserializer<'de> for &'a mut Reader<T> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u8()?;
        visitor.visit_bool(n != 0)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_i8()?;
        visitor.visit_i8(n)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_i16()?;
        visitor.visit_i16(n)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_i32()?;
        visitor.visit_i32(n)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_i64()?;
        visitor.visit_i64(n)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u8()?;
        visitor.visit_u8(n)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u16()?;
        visitor.visit_u16(n)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u32()?;
        visitor.visit_u32(n)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u64()?;
        visitor.visit_u64(n)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_f64<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u32()?;

        match char::from_u32(n) {
            Some(c) => visitor.visit_char(c),
            None => Err(Error::InvalidChar(n)),
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.read_u64()? as usize;

        match self.read_bytes(len)? {
            Cow::Borrowed(bytes) => match str::from_utf8(bytes) {
                Ok(s) => visitor.visit_borrowed_str(s),
                Err(err) => Err(Error::InvalidString(err)),
            },
            Cow::Owned(bytes) => match String::from_utf8(bytes) {
                Ok(s) => visitor.visit_string(s),
                Err(err) => Err(Error::InvalidString(err.utf8_error())),
            },
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.read_u64()? as usize;

        match self.read_bytes(len)? {
            Cow::Borrowed(bytes) => visitor.visit_borrowed_bytes(bytes),
            Cow::Owned(bytes) => visitor.visit_byte_buf(bytes),
        }
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let n = self.read_u8()?;

        if n == 0 {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.read_u64()? as usize;
        visitor.visit_seq(SequenceReader::new(self, len))
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(SequenceReader::new(self, len))
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_seq(SequenceReader::new(self, len))
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let len = self.read_u64()? as usize;
        visitor.visit_map(SequenceReader::new(self, len))
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_seq(SequenceReader::new(self, fields.len()))
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_enum(EnumReader::new(self))
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_u32(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }
}

struct SequenceReader<'a, T> {
    reader: &'a mut Reader<T>,
    cur: usize,
    len: usize,
}

impl<'a, T> SequenceReader<'a, T> {
    fn new(reader: &'a mut Reader<T>, len: usize) -> Self {
        SequenceReader {
            reader,
            cur: 0,
            len,
        }
    }
}

impl<'a, 'de, 'tb: 'de, B: TakeBytes<'tb>> SeqAccess<'de> for SequenceReader<'a, B> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        if self.cur < self.len {
            seed.deserialize(&mut *self.reader).map(|value| {
                self.cur += 1;
                Some(value)
            })
        } else {
            Ok(None)
        }
    }
}

impl<'a, 'de, 'tb: 'de, B: TakeBytes<'tb>> MapAccess<'de> for SequenceReader<'a, B> {
    type Error = Error;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if self.cur < self.len {
            seed.deserialize(&mut *self.reader).map(|value| {
                self.cur += 1;
                Some(value)
            })
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut *self.reader)
    }
}

struct EnumReader<'a, T> {
    reader: &'a mut Reader<T>,
}

impl<'a, T> EnumReader<'a, T> {
    fn new(reader: &'a mut Reader<T>) -> Self {
        EnumReader { reader }
    }
}

impl<'a, 'de, 'tb: 'de, B: TakeBytes<'tb>> EnumAccess<'de> for EnumReader<'a, B> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let value = seed.deserialize(&mut *self.reader)?;
        Ok((value, self))
    }
}

impl<'a, 'de, 'tb: 'de, B: TakeBytes<'tb>> VariantAccess<'de> for EnumReader<'a, B> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(&mut *self.reader)
    }

    fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        de::Deserializer::deserialize_tuple(self.reader, len, visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        de::Deserializer::deserialize_struct(self.reader, "", fields, visitor)
    }
}

The [Serde data model] defines the following types:

* **14 primitive types**
  * `bool`
    * An `u8` value of `0` is defined to be `false`.
    * Any other value is `true`.
    * Note that an `u8` value of `1` is serialized for `true`.
  * `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`
    * Are serialized in big endian encoding. See the corresponding
      `<type>::from_be_bytes` functions for more information.
    * They will take up exactly 1 (for `i8`/`u8`), 2 (for `i16`/`u16`),
      4 (for `i32`/`u32`), 8 (for `i64`/`u64`) and 16 (for `i128`/`u128`)
      bytes.
  * `f32`, `f64`
    * Are serialized in big endian encoding. See the corresponding
      `<type>::from_be_bytes` functions for more information.
    * They will take up exactly 4 (for `f32`) and 8 (for `f64`) bytes.
  * `char`
    * Is casted into a `u32` for serialization.
    * Note that not all `u32`s are valid `char`s, so deserialization might
      fail.
* **string**
  * Serializes the number of bytes as an `u64` value followed by the byte data
    itself.
* **byte array - `[u8]`**
  * Similar to strings, serializes the number of bytes as an `u64` value
    followed by the byte data itself.
* **option**
  * An `u8` value of `0` is defined to be [`None`]. Any other `u8` is a
    [`Some`] value.
  * A [`None`] value is deserialized as an `u8` value of `0`.
  * A [`Some`] value is deserialized as an `u8` value of `1` followed by the
    contained deserialized value.
  * Note that an `u8` value of `1` is taken for serialization of a [`Some`]
    value.
* **unit**
  * No data are required here.
* **unit_struct**
  * No data are required here.
* **unit_variant**
  * The index of the variant is encoded as an `u32` value.
* **newtype_struct**
  * The wrapped value is serialized.
* **newtype_variant**
  * The index of the variant is serialized as an `u32` value followed by
    wrapped, serialized value.
* **seq**
  * Serializes the number of elements as an `u64` value followed by serialized
    elements itself.
* **tuple**
  * Serializes the elements of the tuple one after the other.
* **tuple_struct**
  * Serializes the wrapped elements of the tuple struct one after ther other.
* **tuple_variant**
  * The index of the variant is serialized as an `u32` value followed by
    wrapped, serialized values.
* **map**
  * Serializes the number of elements in the map as an `u64` value followed by
    (key, value) pair of all elements.
  * Note that iterating over a map does not preserve the order. So, serializing
    a map a second time can change to order.
* **struct**
  * Serializes the fields of the struct in the order in which they were defined.
* **struct_variant**
  * The index of the variant is serialized as an `u32` value followed by
    wrapped, serialized fields in the order in which the were defined.

[Serde data model]: https://serde.rs/data-model.html#types
